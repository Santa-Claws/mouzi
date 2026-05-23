use crate::db::get_watched_folders;
use crate::rules::{process_file, should_ignore_file};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tauri::Emitter;
use tauri_plugin_notification::NotificationExt;

const IGNORE_DURATION_SECS: u64 = 5;

#[derive(Debug, Clone)]
struct PendingFile {
    path: std::path::PathBuf,
    scheduled: Instant,
}

pub struct FolderWatcher {
    watchers: HashMap<String, RecommendedWatcher>,
    pending: Arc<Mutex<Vec<PendingFile>>>,
    ignored_files: Arc<Mutex<HashMap<String, Instant>>>,
    /// Shared with AppState — stores the last destination folder to open on notification click
    pending_open_folder: Arc<Mutex<Option<String>>>,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl FolderWatcher {
    pub fn new(
        ignored_files: Arc<Mutex<HashMap<String, Instant>>>,
        pending_open_folder: Arc<Mutex<Option<String>>>,
    ) -> Self {
        Self {
            watchers: HashMap::new(),
            pending: Arc::new(Mutex::new(Vec::new())),
            ignored_files,
            pending_open_folder,
            handle: None,
        }
    }

    pub fn start(&mut self, app_handle: tauri::AppHandle) {
        let pending = self.pending.clone();
        let handle = app_handle.clone();
        let pending_open_folder = self.pending_open_folder.clone();

        // Spawn a thread that processes pending files after a delay
        let handle_thread = std::thread::spawn(move || {
            loop {
                std::thread::sleep(Duration::from_millis(500));
                let now = Instant::now();
                let to_process: Vec<std::path::PathBuf> = {
                    let mut guard = pending.lock().unwrap();
                    let ready: Vec<_> = guard
                        .iter()
                        .filter(|p| now >= p.scheduled)
                        .cloned()
                        .collect();
                    guard.retain(|p| now < p.scheduled);
                    ready.into_iter().map(|p| p.path).collect()
                };

                let mut organized_count = 0;
                let mut last_file_name = String::new();
                let mut last_rule_name = String::new();
                let mut last_dest_folder = String::new();

                for path in to_process {
                    if path.exists() && path.is_file() {
                        match process_file(&path) {
                            Ok(Some((rule, dest))) => {
                                let file_name = path.file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .to_string();
                                let dest_folder = std::path::Path::new(&dest)
                                    .parent()
                                    .map(|p| p.to_string_lossy().to_string())
                                    .unwrap_or_else(|| {
                                        path.parent()
                                            .map(|p| p.to_string_lossy().to_string())
                                            .unwrap_or_default()
                                    });

                                last_file_name = file_name.clone();
                                last_rule_name = rule.name.clone();
                                last_dest_folder = dest_folder.clone();
                                organized_count += 1;

                                // Emit event to frontend (in-app toast)
                                let _ = handle.emit("file-organized", serde_json::json!({
                                    "file": file_name,
                                    "rule": rule.name,
                                    "destination": dest,
                                    "destination_folder": dest_folder,
                                    "success": true
                                }));
                            }
                            Ok(None) => {}
                            Err(e) => {
                                let _ = handle.emit("file-organized", serde_json::json!({
                                    "file": path.to_string_lossy(),
                                    "error": e,
                                    "success": false
                                }));
                            }
                        }
                    }
                }

                // Show a single notification for this batch
                if organized_count > 0 {
                    // Store the destination folder so single-instance handler can open it
                    // when the user clicks the notification (Windows activates the app)
                    *pending_open_folder.lock().unwrap() = Some(last_dest_folder.clone());

                    let body = if organized_count == 1 {
                        format!("{} → {}", last_file_name, last_rule_name)
                    } else {
                        format!("Organized {} files", organized_count)
                    };

                    #[cfg(target_os = "windows")]
                    {
                        let dest_folder_clone = last_dest_folder.clone();
                        let body_clone = body.clone();
                        let _ = std::thread::spawn(move || {
                            let _ = tauri_winrt_notification::Toast::new("cc.mouzi.app")
                                .title("Mouzi – click to open folder")
                                .text1(&body_clone)
                                .on_activated(move |_action| {
                                    // Open the folder in Explorer robustly
                                    let _ = std::process::Command::new("cmd")
                                        .args(["/c", "start", "", &dest_folder_clone])
                                        .spawn();
                                    Ok(())
                                })
                                .show();
                        });
                    }

                    #[cfg(not(target_os = "windows"))]
                    {
                        let _ = handle
                            .notification()
                            .builder()
                            .title("Mouzi – click to open folder")
                            .body(body)
                            .extra("destFolder", last_dest_folder)
                            .show();
                    }
                }
            }
        });

        self.handle = Some(handle_thread);
    }

    pub fn watch_folders(&mut self, app_handle: tauri::AppHandle) -> Result<(), String> {
        let folders = get_watched_folders().map_err(|e| e.to_string())?;
        let pending = self.pending.clone();
        let ignored = self.ignored_files.clone();

        for folder in folders {
            if !folder.enabled {
                continue;
            }
            if !Path::new(&folder.path).exists() {
                eprintln!("[watcher] Skipping missing folder: {}", folder.path);
                continue;
            }
            let path = folder.path.clone();
            let p = pending.clone();
            let ig = ignored.clone();

            let mut watcher = RecommendedWatcher::new(
                move |res: Result<Event, notify::Error>| {
                    if let Ok(event) = res {
                        for path in event.paths {
                            if path.is_file() && !should_ignore_file(&path) {
                                let path_str = path.to_string_lossy().to_string();
                                let mut ignore_guard = ig.lock().unwrap();
                                if let Some(&instant) = ignore_guard.get(&path_str) {
                                    if Instant::now().duration_since(instant) < Duration::from_secs(IGNORE_DURATION_SECS) {
                                        continue;
                                    }
                                    ignore_guard.remove(&path_str);
                                }
                                drop(ignore_guard);
                                let grace = crate::db::get_settings()
                                    .map(|s| s.grace_period_seconds as u64)
                                    .unwrap_or(300);
                                let mut guard = p.lock().unwrap();
                                // Remove existing pending entry for this path to reschedule
                                guard.retain(|x| x.path != path);
                                guard.push(PendingFile {
                                    path,
                                    scheduled: Instant::now() + Duration::from_secs(grace),
                                });
                            }
                        }
                    }
                },
                Config::default()
                    .with_poll_interval(Duration::from_secs(2))
                    .with_compare_contents(true),
            )
            .map_err(|e| e.to_string())?;

            watcher
                .watch(Path::new(&folder.path), RecursiveMode::NonRecursive)
                .map_err(|e| e.to_string())?;

            self.watchers.insert(folder.path, watcher);
        }

        self.start(app_handle);
        Ok(())
    }

    pub fn refresh(&mut self, app_handle: tauri::AppHandle) -> Result<(), String> {
        self.watchers.clear();
        self.watch_folders(app_handle)
    }

    pub fn set_ignored_files(&mut self, ignored_files: Arc<Mutex<HashMap<String, Instant>>>) {
        self.ignored_files = ignored_files;
    }
}
