use std::ffi::c_char;
use flatbox_core::logger::{FlatboxLogger, warn, info, error, debug};

use crate::ptr_to_string;

#[no_mangle]
pub extern "C" fn logger_init() {
    FlatboxLogger::init();
    debug!("Logger::init()");
}

///
/// # Safety
/// `msg` must be a valid C string pointer
#[no_mangle]
pub unsafe extern "C" fn logger_info(msg: *const c_char) {
    info!("{}", ptr_to_string(msg));
}

///
/// # Safety
/// `msg` must be a valid C string pointer
#[no_mangle]
pub unsafe extern "C" fn logger_warn(msg: *const c_char) {
    warn!("{}", ptr_to_string(msg));
}

///
/// # Safety
/// `msg` must be a valid C string pointer
#[no_mangle]
pub unsafe extern "C" fn logger_error(msg: *const c_char) {
    error!("{}", ptr_to_string(msg));
}

///
/// # Safety
/// `msg` must be a valid C string pointer
#[no_mangle]
pub unsafe extern "C" fn logger_debug(msg: *const c_char) {
    debug!("{}", ptr_to_string(msg));
}
