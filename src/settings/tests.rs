//! Once a version is released, add the file it writes, every setting changed, next to this one
//! (`settings-v0.1.0.toml`), with a test that reads it with every setting and one that this
//! version still writes it: the users have that file, and a later version must read it.

use super::*;
use crate::files::TestDirectory;

impl TestDirectory {
    fn file(&self) -> SettingsFile {
        SettingsFile::in_directory(self.0.clone())
    }

    fn write(&self, text: &str) {
        fs::create_dir_all(&self.0).unwrap();
        fs::write(self.0.join(FILE_NAME), text).unwrap();
    }
}

/// Every setting away from its default.
fn changed() -> Settings {
    let mut settings = Settings::default();
    settings.apply(SettingChange::Language(LanguageSetting::French));
    settings.apply(SettingChange::AlwaysOnTop(true));
    settings
}

/// The settings as a TOML table, one key per setting.
fn table(settings: &Settings) -> toml::Table {
    toml::from_str(&toml::to_string(settings).unwrap()).unwrap()
}

/// A setting added to [`changed`] keeps the tests below on every setting.
#[test]
fn the_changed_settings_change_every_setting() {
    let default = table(&Settings::default());
    let changed = table(&changed());

    assert_eq!(
        default.keys().collect::<Vec<_>>(),
        changed.keys().collect::<Vec<_>>()
    );
    for (key, value) in &changed {
        assert_ne!(Some(value), default.get(key), "{key}");
    }
}

/// A setting added without its `deserialize_with` would lose every setting of the user for
/// one value it can't read.
#[test]
fn every_setting_that_cant_be_read_takes_its_default_and_keeps_the_others() {
    let default = table(&Settings::default());
    let changed = table(&changed());
    for key in changed.keys() {
        let mut odd = changed.clone();
        let mut unreadable = toml::Table::new();
        unreadable.insert("unreadable".to_owned(), toml::Value::Boolean(true));
        odd.insert(key.clone(), toml::Value::Table(unreadable));
        let directory = TestDirectory::new("every-odd-value");
        directory.write(&toml::to_string(&odd).unwrap());

        let loaded = table(&directory.file().load());

        for (other, value) in &loaded {
            let expected = if other == key { &default } else { &changed };
            assert_eq!(
                Some(value),
                expected.get(other),
                "{other} with {key} unreadable"
            );
        }
    }
}

/// A language added to the interface gets its choice in the settings, which the compiler
/// doesn't ask for.
#[test]
fn each_language_of_the_interface_can_be_chosen_besides_the_system() {
    let fixed: Vec<Language> = LanguageSetting::ALL
        .into_iter()
        .filter(|setting| *setting != LanguageSetting::System)
        .map(LanguageSetting::language)
        .collect();

    assert_eq!(fixed, Language::ALL);
}

#[test]
fn the_default_settings_follow_the_system_below_the_other_windows() {
    let settings = Settings::default();

    assert_eq!(settings.language(), LanguageSetting::System);
    assert!(!settings.always_on_top());
    assert_eq!(settings.window_level(), WindowLevel::Normal);
}

#[test]
fn a_change_says_whether_it_changed_something() {
    let mut settings = Settings::default();

    assert!(settings.apply(SettingChange::AlwaysOnTop(true)));
    assert!(!settings.apply(SettingChange::AlwaysOnTop(true)));
    assert!(settings.apply(SettingChange::Language(LanguageSetting::English)));
    assert!(!settings.apply(SettingChange::Language(LanguageSetting::English)));
    assert_eq!(settings.window_level(), WindowLevel::AlwaysOnTop);
}

#[test]
fn the_file_is_readable_toml() {
    let text = toml::to_string(&changed()).unwrap();

    assert_eq!(text, "language = \"french\"\nalways_on_top = true\n");
}

#[test]
fn saved_settings_load_back() {
    let directory = TestDirectory::new("round-trip");
    let file = directory.file();

    file.save(&changed());

    assert_eq!(file.load(), changed());
}

#[test]
fn a_new_save_replaces_the_previous_one() {
    let directory = TestDirectory::new("replace");
    let file = directory.file();

    file.save(&changed());
    file.save(&Settings::default());

    assert_eq!(file.load(), Settings::default());
}

#[test]
fn a_missing_file_gives_the_default_settings() {
    let directory = TestDirectory::new("missing");

    assert_eq!(directory.file().load(), Settings::default());
}

#[test]
fn a_missing_value_takes_its_default() {
    let directory = TestDirectory::new("partial");
    directory.write("always_on_top = true\n");

    let settings = directory.file().load();

    assert_eq!(settings.language(), LanguageSetting::System);
    assert!(settings.always_on_top());
}

/// One value this version doesn't know would otherwise lose every setting, the file being
/// replaced by the default ones at the next change.
#[test]
fn a_value_that_cant_be_read_takes_its_default_and_the_others_are_kept() {
    for (text, language, always_on_top) in [
        (
            "language = \"german\"\nalways_on_top = true\n",
            LanguageSetting::System,
            true,
        ),
        (
            "language = 5\nalways_on_top = true\n",
            LanguageSetting::System,
            true,
        ),
        (
            "language = \"french\"\nalways_on_top = \"yes\"\n",
            LanguageSetting::French,
            false,
        ),
    ] {
        let directory = TestDirectory::new("odd-value");
        directory.write(text);

        let settings = directory.file().load();

        assert_eq!(settings.language(), language, "{text}");
        assert_eq!(settings.always_on_top(), always_on_top, "{text}");
        assert!(directory.0.join(FILE_NAME).exists(), "{text}");
    }
}

/// The file of a later version, read by this one after a downgrade: what it doesn't know is
/// left out, and the rest is read. A `deny_unknown_fields` added by mistake would lose every
/// setting instead.
#[test]
fn the_settings_of_a_later_version_are_ignored_and_the_others_are_read() {
    let directory = TestDirectory::new("later-version");
    directory.write(
        "future_setting = 1\nlanguage = \"french\"\nalways_on_top = true\n[panel]\nopacity = 0.8\n",
    );

    let settings = directory.file().load();

    assert_eq!(settings, changed());
    assert!(directory.0.join(FILE_NAME).exists(), "not set aside");
}

#[test]
fn a_file_that_isnt_toml_gives_the_default_settings_and_is_kept_aside() {
    let directory = TestDirectory::new("unreadable");
    directory.write("not toml");
    let file = directory.file();

    assert_eq!(file.load(), Settings::default());

    assert!(!directory.0.join(FILE_NAME).exists());
    let aside = fs::read_to_string(directory.0.join(INVALID_FILE_NAME)).unwrap();
    assert_eq!(aside, "not toml");
    // The settings saved after it don't touch the file kept aside, and a second unreadable
    // file replaces the first one there.
    file.save(&changed());
    directory.write("still = not toml");
    assert_eq!(file.load(), Settings::default());
    let aside = fs::read_to_string(directory.0.join(INVALID_FILE_NAME)).unwrap();
    assert_eq!(aside, "still = not toml");
}

#[test]
fn without_a_configuration_directory_nothing_is_written() {
    let file = SettingsFile { directory: None };

    file.save(&changed());

    assert_eq!(file.load(), Settings::default());
    assert_eq!(file.window_file(), None);
}

#[test]
fn the_window_file_is_next_to_the_settings() {
    let directory = TestDirectory::new("window-file");

    assert_eq!(
        directory.file().window_file(),
        Some(directory.0.join(WINDOW_FILE_NAME))
    );
}

/// A settings folder that can't be made, whatever the rights of the user running the tests:
/// its path goes through an ordinary file.
#[test]
fn a_failed_write_keeps_the_settings_of_the_session_without_a_panic() {
    let directory = TestDirectory::new("blocked");
    fs::create_dir_all(&directory.0).unwrap();
    fs::write(directory.0.join("blocked"), "").unwrap();
    let file = SettingsFile {
        directory: Some(directory.0.join("blocked").join("settings")),
    };

    file.save(&changed());

    assert_eq!(file.load(), Settings::default());
    assert_eq!(fs::read(directory.0.join("blocked")).unwrap(), b"");
}
