use flatbox_native_macro::native;
use flatbox_render::{
    pbr::{
        material::DefaultMaterial, 
        texture::Texture,
    },
    include_texture,
};
use crate::{shared, SharedNativeComponent, ToNative};

pub struct Material(SharedNativeComponent);

#[native]
impl Material {
    pub fn debug() -> Material {
        Material(shared!(DefaultMaterial {
            diffuse_map: include_texture!("../assets/textures/dev.png"),
            ..Default::default()
        }))
    }
}

impl ToNative for Material {    
    fn native(&self) -> SharedNativeComponent {
        SharedNativeComponent::clone(&self.0)
    }
}

impl Clone for Material {
    fn clone(&self) -> Self {
        Material(SharedNativeComponent::clone(&self.0))
    }
}