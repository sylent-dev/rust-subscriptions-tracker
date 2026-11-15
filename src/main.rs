//! Subscriptions Tracker, from the skeleton of rust-desktop-template: one window, its texts
//! translated, its settings kept between the launches.

// No console window next to the application on Windows, except in debug builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod files;
mod i18n;
mod language;
mod logging;
mod settings;

use std::process::ExitCode;

/// The name of the application, in every language.
const APP_NAME: &str = "Subscriptions Tracker";
/// The version of the application, from `Cargo.toml`.
const VERSION: &str = env!("CARGO_PKG_VERSION");
/// The name of the executable (`[[bin]]` in `Cargo.toml`), also the application id of the
/// window, which the desktop groups its windows by.
const APP_ID: &str = env!("CARGO_BIN_NAME");
/// The name of the folders of the application in the user's directories.
const FOLDER_NAME: &str = APP_ID;

/// The size of the window at the first launch.
const WINDOW_SIZE: [f32; 2] = [480.0, 360.0];
/// The smallest window the layout still fits in.
const MIN_WINDOW_SIZE: [f32; 2] = [320.0, 240.0];

fn main() -> ExitCode {
    logging::init();
    log::info!("{APP_NAME} {VERSION} started");
    let settings_file = settings::SettingsFile::in_config_directory();
    let settings = settings_file.load();
    let result = eframe::run_native(
        APP_NAME,
        native_options(&settings, &settings_file),
        Box::new(move |_creation| Ok(Box::new(app::App::new(settings, settings_file)))),
    );
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            log::error!("The window failed: {error}");
            ExitCode::FAILURE
        }
    }
}

/// The options of the window: the title bar of the system, its level from the settings, its
/// size and position kept by eframe next to the settings file.
fn native_options(
    settings: &settings::Settings,
    settings_file: &settings::SettingsFile,
) -> eframe::NativeOptions {
    on_x11(eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(APP_NAME)
            .with_app_id(APP_ID)
            .with_inner_size(WINDOW_SIZE)
            .with_min_inner_size(MIN_WINDOW_SIZE)
            .with_window_level(settings.window_level()),
        // eframe restores the size and the position of the window of the last session, on a
        // screen that is still there.
        persist_window: true,
        persistence_path: settings_file.window_file(),
        ..Default::default()
    })
}

/// On Linux, the window always goes through X11 (XWayland under Wayland): see
/// docs/technical-choices.md (Linux display). Without it, winit would pick Wayland, which the
/// OpenGL renderer isn't built for.
#[cfg(target_os = "linux")]
fn on_x11(options: eframe::NativeOptions) -> eframe::NativeOptions {
    use winit::platform::x11::EventLoopBuilderExtX11;
    eframe::NativeOptions {
        event_loop_builder: Some(Box::new(|builder| {
            builder.with_x11();
        })),
        ..options
    }
}

/// The other systems have one display server only.
#[cfg(not(target_os = "linux"))]
fn on_x11(options: eframe::NativeOptions) -> eframe::NativeOptions {
    options
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A rename of the executable moves the folders of the user (the settings are then lost)
    /// and the names of the release archives (the links of a download page break): only on
    /// purpose, with this test.
    #[test]
    fn renaming_the_executable_moves_the_settings_and_breaks_the_download_links() {
        assert_eq!(APP_ID, "subscriptions-tracker");
    }
}
