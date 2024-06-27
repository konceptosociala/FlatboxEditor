use flatbox_render::pbr::model::Model as NativeModel;
use flatbox_native_macro::native;

use crate::{shared, Material, SharedNativeComponent, ToNative, Transform};

pub struct Model {
    model: SharedNativeComponent,
    material: Material,
    transform: Transform,
}

#[native]
impl Model {
    pub fn cube(transform: &Transform, material: &Material) -> Model {
        Model {
            model: shared!(NativeModel::cube()),
            material: material.clone(),
            transform: transform.clone(),
        }
    }
}

impl Model {
    pub fn transform(&self) -> Transform {
        self.transform.clone()
    }

    pub fn material(&self) -> Material {
        self.material.clone()
    }
}

impl ToNative for Model {
    fn native(&self) -> SharedNativeComponent {
        SharedNativeComponent::clone(&self.model)
    }
}

impl Clone for Model {
    fn clone(&self) -> Self {
        Model {
            model: SharedNativeComponent::clone(&self.model),
            material: self.material.clone(),
            transform: self.transform.clone(),
        }
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