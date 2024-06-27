use flatbox_render::pbr::camera::{
    Camera as NativeCamera, 
    CameraType,
};
use flatbox_native_macro::native;

use crate::{shared, SharedNativeComponent, ToNative, Transform};

pub struct Camera {
    inner: SharedNativeComponent,
    transform: Transform,
}

#[native]
impl Camera {
    pub fn new(transform: &Transform) -> Camera {
        Camera {
            inner: shared!(NativeCamera::builder()
                .camera_type(CameraType::FirstPerson)
                .is_active(true)
                .build()
            ),
            transform: transform.clone(),
        }
    }
}

impl Camera {
    pub fn transform(&self) -> Transform {
        self.transform.clone()
    }
}

impl ToNative for Camera {
    fn native(&self) -> SharedNativeComponent {
        SharedNativeComponent::clone(&self.inner)
    }
}

impl Clone for Camera {
    fn clone(&self) -> Self {
        Camera {
            inner: SharedNativeComponent::clone(&self.inner),
            transform: self.transform.clone(),
        }
    }
}