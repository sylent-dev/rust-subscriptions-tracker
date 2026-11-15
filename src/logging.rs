//! The log: to the standard error, and to a file of the user's directories, which a release on
//! Windows needs (it has no console).

use std::fs::{self, File};
use std::io;
use std::path::PathBuf;

use simplelog::{CombinedLogger, Config, LevelFilter, SharedLogger, SimpleLogger, WriteLogger};

use crate::files;

/// The file of the log, in the local data directory of the application.
const FILE_NAME: &str = "log.txt";

/// Starts the log. The file is written again at each launch, so that it holds one session and
/// never grows; without it (no home directory, a folder that can't be made), the log goes to the
/// standard error alone.
pub fn init() {
    let mut loggers: Vec<Box<dyn SharedLogger>> =
        vec![SimpleLogger::new(LevelFilter::Info, Config::default())];
    // Told once the log is started.
    let told = match log_file() {
        Some((path, Ok(file))) => {
            loggers.push(WriteLogger::new(LevelFilter::Info, Config::default(), file));
            Ok(path)
        }
        Some((path, Err(error))) => Err(format!(
            "The log can't be written to {}: {error}",
            path.display()
        )),
        None => Err("No home directory: the log isn't written to a file".to_owned()),
    };
    // Only fails when a logger is already set, which `main` rules out.
    let _ = CombinedLogger::init(loggers);
    match told {
        Ok(path) => log::info!("Log file: {}", path.display()),
        Err(warning) => log::warn!("{warning}"),
    }
}

/// The path of the log file and the file, emptied; none without a home directory.
fn log_file() -> Option<(PathBuf, io::Result<File>)> {
    let directory = files::project_directories()?.data_local_dir().to_path_buf();
    let path = directory.join(FILE_NAME);
    let file = fs::create_dir_all(&directory).and_then(|()| File::create(&path));
    Some((path, file))
}
