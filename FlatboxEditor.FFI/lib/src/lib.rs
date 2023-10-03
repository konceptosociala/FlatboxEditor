pub mod render;

pub use render::*;

use std::ffi::*;

#[no_mangle]
pub extern "C" fn init_gl(mut init_function: extern fn(*const c_char) -> *const c_void) {
    gl::load_with(|ptr| {
        let cstring = CString::new(ptr).unwrap();
        let ptr = cstring.as_ptr() as *const _;
        init_function(ptr)
    });
}

#[no_mangle]
pub extern "C" fn render_gl(){
    unsafe {
        gl::ClearColor(1.0, 0.3, 0.5, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}