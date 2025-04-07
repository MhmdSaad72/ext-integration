pub mod cache;
use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    // Global map to store loggers for different files
    pub static ref LOGGER_STATE: Mutex<Option<String>> = Mutex::new(None);
}
