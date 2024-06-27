use std::sync::Arc;

use flatbox_assets::{
    parking_lot::Mutex, 
    ser_component::SerializableComponent, 
    scene::Scene as NativeScene,
};
use flatbox_core::math::glm;
use serde::{Serialize, Deserialize};

pub mod camera;
pub mod grid;
pub mod model;
pub mod render;
pub mod scene;
pub mod transform;
pub mod material;

pub use camera::*;
pub use grid::*;
pub use model::*;
pub use render::*;
pub use scene::*;
pub use transform::*;
pub use material::*;

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct NativeColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl NativeColor {
    pub fn new(r: u8, g: u8, b: u8) -> NativeColor {
        NativeColor { r, g, b }
    }
}

impl From<NativeColor> for glm::Vec3 {
    fn from(color: NativeColor) -> Self {
        glm::vec3(
            (color.r / 255) as f32, 
            (color.g / 255) as f32, 
            (color.b / 255) as f32
        )
    }
}

pub trait ToNative {
    fn native(&self) -> SharedNativeComponent;
}

pub type SharedNativeComponent = Arc<Mutex<Box<dyn SerializableComponent>>>;

pub type SharedNativeScene = Arc<Mutex<NativeScene>>;

#[macro_export]
macro_rules! shared {
    ($e:expr) => {
        ::std::sync::Arc::new(
            ::flatbox_assets::parking_lot::Mutex::new(
                Box::new($e)
            )
        )
    };
}

#[macro_export]
macro_rules! shared_ref {
    ($native:expr => $cast:ty) => {
        $native
            .lock()
            .as_any()
            .downcast_ref::<$cast>()
            .unwrap_or_else(|| {
                panic!("Object `{}` is not of a type `{}`", stringify!($expr), stringify!($cast));
            })
    };
}

#[macro_export]
macro_rules! shared_mut {
    ($native:expr => $cast:ty) => {
        $native
            .lock()
            .as_any_mut()
            .downcast_mut::<$cast>()
            .unwrap_or_else(|| {
                panic!("Object `{}` is not of a type `{}`", stringify!($expr), stringify!($cast));
            })
    };
}

#[macro_export]
macro_rules! is_shared_type {
    ($native:expr => $cast:ty) => {
        $native
            .lock()
            .as_any()
            .downcast_ref::<$cast>()
            .is_some()
    };
}