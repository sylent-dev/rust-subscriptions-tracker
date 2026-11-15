//! The window of the application: its texts in the language of the settings, and the settings
//! saved as soon as the user changes them.

use egui::{Context, Ui, ViewportCommand};

use crate::i18n::{Message, Translator};
use crate::settings::{LanguageSetting, SettingChange, Settings, SettingsFile};

/// The state of the window between two frames.
pub struct App {
    settings: Settings,
    settings_file: SettingsFile,
    /// The messages of the language of the settings, made again when it changes.
    translator: Translator,
}

impl App {
    #[must_use]
    pub fn new(settings: Settings, settings_file: SettingsFile) -> Self {
        Self {
            translator: Translator::new(settings.language().language()),
            settings,
            settings_file,
        }
    }

    /// Applies `change`, saves the settings and passes the change on to the window; nothing
    /// when the setting already had this value.
    fn apply(&mut self, ctx: &Context, change: SettingChange) {
        if !self.settings.apply(change) {
            return;
        }
        self.settings_file.save(&self.settings);
        match change {
            SettingChange::Language(language) => {
                self.translator = Translator::new(language.language());
            }
            SettingChange::AlwaysOnTop(_) => {
                ctx.send_viewport_cmd(ViewportCommand::WindowLevel(self.settings.window_level()));
            }
        }
    }

    /// The settings, and the change the user made in them during this frame.
    fn settings_ui(&self, ui: &mut Ui) -> Option<SettingChange> {
        let mut change = None;
        let current = self.settings.language();
        ui.horizontal(|ui| {
            ui.label(self.translator.text(Message::LanguageLabel));
            egui::ComboBox::from_id_salt("language")
                .selected_text(self.translator.text(language_message(current)))
                .show_ui(ui, |ui| {
                    for choice in LanguageSetting::ALL {
                        let name = self.translator.text(language_message(choice));
                        if ui.selectable_label(choice == current, name).clicked() {
                            change = Some(SettingChange::Language(choice));
                        }
                    }
                });
        });
        let mut always_on_top = self.settings.always_on_top();
        if ui
            .checkbox(
                &mut always_on_top,
                self.translator.text(Message::AlwaysOnTop),
            )
            .changed()
        {
            change = Some(SettingChange::AlwaysOnTop(always_on_top));
        }
        change
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        let change = egui::CentralPanel::default()
            .show(ui, |ui| {
                ui.heading(crate::APP_NAME);
                ui.label(self.translator.text(Message::Greeting));
                ui.separator();
                let change = self.settings_ui(ui);
                ui.separator();
                ui.weak(self.translator.version(crate::VERSION));
                change
            })
            .inner;
        if let Some(change) = change {
            self.apply(&ctx, change);
        }
    }
}

/// The name of a choice of language in the settings.
fn language_message(setting: LanguageSetting) -> Message {
    match setting {
        LanguageSetting::System => Message::LanguageSystem,
        LanguageSetting::English => Message::LanguageEnglish,
        LanguageSetting::French => Message::LanguageFrench,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::files::TestDirectory;
    use crate::language::Language;

    #[test]
    fn a_change_is_saved_at_once_and_applied_to_the_window() {
        let directory = TestDirectory::new("app-apply");
        let file = || SettingsFile::in_directory(directory.0.clone());
        let mut app = App::new(Settings::default(), file());
        let ctx = Context::default();

        app.apply(&ctx, SettingChange::Language(LanguageSetting::French));
        app.apply(&ctx, SettingChange::AlwaysOnTop(true));

        let saved = file().load();
        assert_eq!(saved.language(), LanguageSetting::French);
        assert!(saved.always_on_top());
        assert_eq!(
            app.translator.text(Message::LanguageLabel),
            Translator::new(Language::French).text(Message::LanguageLabel)
        );
    }

    #[test]
    fn each_choice_of_language_has_a_name_of_its_own() {
        let names: std::collections::HashSet<Message> = LanguageSetting::ALL
            .into_iter()
            .map(language_message)
            .collect();

        assert_eq!(names.len(), LanguageSetting::ALL.len());
    }
}
