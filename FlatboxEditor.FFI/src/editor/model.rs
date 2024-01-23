use flatbox_core::logger::debug;
use flatbox_render::pbr::model::Model as NativeModel;
use native_macro::native;

pub struct Model(pub NativeModel);

#[native]
impl Model {
    pub fn cube() -> Model {
        debug!("Model::cube()");
        Model(NativeModel::cube())
    }
}

// #[no_mangle]
// pub extern "C" fn model_plane() -> *mut Model {
//     Box::into_raw(Box::new(Model::plane()))
// }

// #[no_mangle]
// pub extern "C" fn model_icosahedron() -> *mut Model {
//     Box::into_raw(Box::new(Model::icosahedron()))
// }

// #[no_mangle]
// pub extern "C" fn model_sphere() -> *mut Model {
//     Box::into_raw(Box::new(Model::sphere()))
// }

// ///
// /// # Safety
// /// `model` must be a valid `Model` pointer
// #[no_mangle]
// pub unsafe extern "C" fn model_free(model: *mut NativeModel) {
//     free_ptr(model);
//     debug!("Model::free()");
// }