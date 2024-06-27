use flatbox_core::math::{glm, transform::Transform as NativeTransform};
use flatbox_native_macro::native;

use crate::{shared, SharedNativeComponent, ToNative};

pub struct Transform(SharedNativeComponent);

#[native]
impl Transform {
    pub fn identity() -> Transform {
        Transform(shared!(NativeTransform::identity()))
    }

    pub fn debug() -> Transform {
        Transform(shared!(NativeTransform {
            translation: glm::vec3(3.0, -3.0, 3.0),
            rotation: glm::safe_quat_look_at(
                &glm::vec3(0.0, 0.0, 0.0), 
                &glm::vec3(3.0, -3.0, 3.0),
                &glm::Vec3::y_axis(), 
                &glm::Vec3::y_axis(),
            ),
            scale: 1.0,
        }))
    }
}

impl ToNative for Transform {
    fn native(&self) -> SharedNativeComponent {
        SharedNativeComponent::clone(&self.0)
    }
}

impl Clone for Transform {
    fn clone(&self) -> Self {
        Transform(SharedNativeComponent::clone(&self.0))
    }
}