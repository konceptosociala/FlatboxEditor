use flatbox_render::pbr::camera::{
    Camera as FlatboxCamera, 
    CameraType
};
use flatbox_core::math::{
    transform::Transform, 
    glm
};

use crate::free_ptr;

pub struct Camera {
    pub inner: FlatboxCamera,
    pub transform: Transform,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            inner: FlatboxCamera::builder()
                .camera_type(CameraType::FirstPerson)
                .is_active(true)
                .build(),
            transform: Transform {
                translation: glm::vec3(3.0, -3.0, 3.0),
                rotation: glm::safe_quat_look_at(
                    &glm::vec3(0.0, 0.0, 0.0), 
                    &glm::vec3(3.0, -3.0, 3.0),
                    &glm::Vec3::y_axis(), 
                    &glm::Vec3::y_axis(),
                ),
                scale: 1.0,
            },
        }
    }
}

#[no_mangle]
pub extern "C" fn camera_new() -> *mut Camera {
    Box::into_raw(Box::default())
}

///
/// # Safety
/// `camera` must be a valid `Renderer` pointer
#[no_mangle]
pub unsafe extern "C" fn camera_free(camera: *mut Camera) {
    free_ptr(camera);
}