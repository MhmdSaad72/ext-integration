use std::{fs::read_to_string, path::Path, sync::Once};

use chrono::Local;
use fern::Dispatch;
use serde::Deserialize;
use toml::from_str;

static INIT: Once = Once::new();

#[derive(Deserialize, Debug)]
struct Config {
    log_files: LogFiles,
}

#[derive(Deserialize, Debug)]
struct LogFiles {
    file_names: Vec<String>,
}

fn load_config() -> Config {
    let file_path = Path::new("src/loggers/config.toml");
    let config_content = read_to_string(file_path).expect("Failed to read the configuration file");
    from_str(&config_content).expect("Failed to parse the configuration file")
}

#[allow(dead_code)]
pub fn setup_logger() {
    let config = load_config();
    let file_names = config.log_files.file_names;
    let current_date = Local::now().format("%Y-%m-%d").to_string();

    INIT.call_once(|| {
        // Create a new Fern dispatcher
        let mut base_dispatcher = Dispatch::new().format(|out, message, _record| {
            out.finish(format_args!(
                "[{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                message
            ))
        });

        // Dynamically add each file to the dispatcher
        for file_name in file_names {
            let log_file = format!("storage/logs/{}_{}.log", file_name, current_date);
            base_dispatcher = base_dispatcher.chain(
                Dispatch::new()
                    .filter(move |metadata| metadata.target() == file_name)
                    .chain(fern::log_file(log_file).unwrap()),
            );
        }

        // Apply the logger
        base_dispatcher.apply().expect("Failed to apply logger");
    });
}
