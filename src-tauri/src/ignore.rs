use std::fs;
use std::path::Path;

/// Load .mouziignore patterns from a folder. Returns Vec of non-empty, non-comment lines.
pub fn load_mouziignore(folder_path: &str) -> Vec<String> {
    let path = Path::new(folder_path).join(".mouziignore");
    if !path.exists() {
        return Vec::new();
    }
    match fs::read_to_string(&path) {
        Ok(content) => content
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .map(|l| l.to_string())
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// Save patterns to .mouziignore in the given folder.
/// Writes a header comment, then one pattern per line.
pub fn save_mouziignore(folder_path: &str, patterns: &[String]) -> Result<(), String> {
    let path = Path::new(folder_path).join(".mouziignore");
    let mut content = String::from("# Mouzi ignore rules\n# https://mouzi.cc/docs\n\n");
    for p in patterns {
        content.push_str(p);
        content.push('\n');
    }
    fs::write(&path, content).map_err(|e| e.to_string())
}

/// Check if a file name matches any of the ignore patterns.
/// Supports: literal match, `*` wildcard, and `folder/` directory suffix.
/// On Windows, matching is case-insensitive for both literals and wildcards.
pub fn is_ignored(name: &str, patterns: &[String]) -> bool {
    #[cfg(windows)]
    let name = name.to_lowercase();

    for original_pat in patterns {
        #[cfg(windows)]
        let pat = original_pat.to_lowercase();
        #[cfg(not(windows))]
        let pat = original_pat.as_str();

        // Directory pattern: ends with /
        if pat.ends_with('/') {
            let dir_pat = &pat[..pat.len() - 1];
            if name.eq_ignore_ascii_case(dir_pat) {
                return true;
            }
            continue;
        }
        // Wildcard pattern: contains *
        if pat.contains('*') {
            let parts: Vec<&str> = pat.split('*').collect();
            if parts.len() == 2 {
                let prefix = parts[0];
                let suffix = parts[1];
                if name.starts_with(prefix) && name.ends_with(suffix) {
                    return true;
                }
            }
            continue;
        }
        // Literal match
        if name.eq_ignore_ascii_case(&pat) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(windows)]
    fn wildcard_case_insensitive_on_windows() {
        assert!(is_ignored("FOO.TMP", &["*.tmp".to_string()]));
        assert!(is_ignored("Foo.Tmp", &["*.tmp".to_string()]));
        assert!(is_ignored("BAR.EXE", &["*.exe".to_string()]));
        assert!(is_ignored("prefixSUFFIX.txt", &["prefix*.TXT".to_string()]));
        assert!(is_ignored("README", &["readme".to_string()]));
        assert!(!is_ignored("foo.tmp", &["*.txt".to_string()]));
        assert!(!is_ignored("FOO.TMP", &["*.txt".to_string()]));
    }

    #[test]
    #[cfg(not(windows))]
    fn wildcard_case_sensitive_on_non_windows() {
        assert!(is_ignored("foo.tmp", &["*.tmp".to_string()]));
        assert!(!is_ignored("FOO.TMP", &["*.tmp".to_string()]));
    }
}
