use std::ffi::c_char;
use flatbox_core::logger::{FlatboxLogger, warn, info, error, debug};
use native_macro::native;

pub struct Logger;

#[native]
impl Logger {
    pub fn init() {
        FlatboxLogger::init();
        debug!("Logger::init()");
    }

    pub fn info(msg: &str) {
        info!("{msg}");
    }

    pub fn warn(msg: &str) {
        warn!("{msg}");
    }

    pub fn error(msg: &str) {
        error!("{msg}");
    }

    pub fn debug(msg: &str) {
        debug!("{msg}");
    }
}