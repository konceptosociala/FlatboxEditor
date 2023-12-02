use flatbox_core::logger::debug;
use flatbox_render::pbr::model::Model;

use crate::free_ptr;


// #[no_mangle]
// pub extern "C" fn model_plane() -> *mut Model {
//     Box::into_raw(Box::new(Model::plane()))
// }

#[no_mangle]
pub extern "C" fn model_cube() -> *mut Model {
    debug!("Model::cube()");
    Box::into_raw(Box::new(Model::cube()))
}

// #[no_mangle]
// pub extern "C" fn model_icosahedron() -> *mut Model {
//     Box::into_raw(Box::new(Model::icosahedron()))
// }

// #[no_mangle]
// pub extern "C" fn model_sphere() -> *mut Model {
//     Box::into_raw(Box::new(Model::sphere()))
// }

///
/// # Safety
/// `model` must be a valid `Model` pointer
#[no_mangle]
pub unsafe extern "C" fn model_free(model: *mut Model) {
    free_ptr(model);
    debug!("Model::free()");
}