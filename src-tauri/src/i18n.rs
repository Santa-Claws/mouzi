use std::collections::HashMap;

pub struct TrayI18n {
    strings: HashMap<&'static str, &'static str>,
}

impl TrayI18n {
    pub fn new(lang: &str) -> Self {
        let mut strings = HashMap::new();
        match lang {
            "pl" => {
                strings.insert("quit", "Zamknij");
                strings.insert("settings", "Ustawienia");
                strings.insert("clean_now", "Posprzątaj teraz");
                strings.insert("tooltip", "Mouzi");
                strings.insert("popup_title", "Mouzi");
                strings.insert("settings_title", "Ustawienia Mouzi");
                strings.insert("organized", "Uporządkowano {} plik(i)");
            }
            "it" => {
                strings.insert("quit", "Esci");
                strings.insert("settings", "Impostazioni");
                strings.insert("clean_now", "Pulisci ora");
                strings.insert("tooltip", "Mouzi");
                strings.insert("popup_title", "Mouzi");
                strings.insert("settings_title", "Impostazioni Mouzi");
                strings.insert("organized", "Organizzati {} file");
            }
            "de" => {
                strings.insert("quit", "Beenden");
                strings.insert("settings", "Einstellungen");
                strings.insert("clean_now", "Jetzt aufräumen");
                strings.insert("tooltip", "Mouzi");
                strings.insert("popup_title", "Mouzi");
                strings.insert("settings_title", "Mouzi Einstellungen");
                strings.insert("organized", "{} Datei(en) organisiert");
            }
            "fr" => {
                strings.insert("quit", "Quitter");
                strings.insert("settings", "Paramètres");
                strings.insert("clean_now", "Nettoyer maintenant");
                strings.insert("tooltip", "Mouzi");
                strings.insert("popup_title", "Mouzi");
                strings.insert("settings_title", "Paramètres Mouzi");
                strings.insert("organized", "{} fichier(s) organisé(s)");
            }
            "ru" => {
                strings.insert("quit", "Выход");
                strings.insert("settings", "Настройки");
                strings.insert("clean_now", "Очистить сейчас");
                strings.insert("tooltip", "Mouzi");
                strings.insert("popup_title", "Mouzi");
                strings.insert("settings_title", "Настройки Mouzi");
                strings.insert("organized", "Организовано {} файл(ов)");
            }
            "ja" => {
                strings.insert("quit", "終了");
                strings.insert("settings", "設定");
                strings.insert("clean_now", "今すぐ整理");
                strings.insert("tooltip", "Mouzi");
                strings.insert("popup_title", "Mouzi");
                strings.insert("settings_title", "Mouziの設定");
                strings.insert("organized", "{}個のファイルを整理しました");
            }
            _ => {
                strings.insert("quit", "Quit");
                strings.insert("settings", "Settings");
                strings.insert("clean_now", "Clean Now");
                strings.insert("tooltip", "Mouzi");
                strings.insert("popup_title", "Mouzi");
                strings.insert("settings_title", "Mouzi Settings");
                strings.insert("organized", "Organized {} file(s)");
            }
        }
        Self { strings }
    }

    pub fn get<'a>(&self, key: &'a str) -> &'a str {
        self.strings.get(key).copied().unwrap_or(key)
    }
}
