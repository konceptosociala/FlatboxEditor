use flatbox_render::pbr::camera::{
    Camera as FlatboxCamera, 
    CameraType,
};
use flatbox_core::math::{
    transform::Transform, 
    glm,
};
use flatbox_native_macro::native;

pub struct Camera {
    pub inner: FlatboxCamera,
    pub transform: Transform,
}

#[native]
impl Camera {
    pub fn new() -> Camera {
        Camera::default()
    }
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