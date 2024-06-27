use std::{ffi::c_char, sync::Arc};
use flatbox_core::logger::error;
use flatbox_assets::{
    parking_lot::Mutex, ron, scene::{Scene as NativeScene, SerializableEntity}};
use flatbox_native_macro::native;

use crate::{Model, SharedNativeScene, ToNative};

pub struct Scene(SharedNativeScene);

impl Default for Scene {
    fn default() -> Self {
        Scene(Arc::new(Mutex::new(NativeScene::default())))
    }
}

#[native]
impl Scene {
    pub fn new() -> Scene {
        Scene::default()
    }

    pub fn open(path: &str) -> Scene {
        match std::fs::read_to_string(path) {
            Ok(ser) => match ron::from_str(&ser) {
                Ok(scene) => Scene(scene),
                Err(e) => {
                    error!("Cannot load scene `{}`:\n{}", path, e);
                    Scene::new()
                },
            },
            Err(e) => {
                error!("Cannot open file `{}`: {}", path, e);
                Scene::new()
            },
        }
    }

    pub fn add_model(scene: &mut Scene, model: &Model) {    
        scene.0.lock().entities.push(
            SerializableEntity {
                components: vec![
                    model.native(),
                    Model::transform(model).native(),
                    Model::material(model).native(),
                ]
            }
        );    
    }

    pub fn save(scene: &Scene, path: &str) {
        let scene = match ron::ser::to_string_pretty(
            &scene.0,
            ron::ser::PrettyConfig::default()
        ){
            Ok(scene) => scene,
            Err(e) => {
                error!("Cannot save scene: {}", e);
                return;
            }
        };

        if let Err(e) = std::fs::write(path, scene) {
            error!("Cannot save scene: {}", e);
        }
    }
}

impl Scene {
    pub fn native_scene(&self) -> SharedNativeScene {
        Arc::clone(&self.0)
    }
}