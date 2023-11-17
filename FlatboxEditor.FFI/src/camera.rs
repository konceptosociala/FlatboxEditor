use flatbox_render::pbr::camera::Camera as FlatboxCamera;
use flatbox_core::math::transform::Transform;

pub struct Camera {
    pub inner: FlatboxCamera,
    pub transform: Transform,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            inner: FlatboxCamera::builder().is_active(true).build(),
            transform: Transform::identity(),
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
    if camera.is_null() {
        return;
    }
    
    let _ = Box::from_raw(camera);
}