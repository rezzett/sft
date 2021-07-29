use std::collections::HashMap;

use sfml::{graphics::Texture, SfBox};

pub const TEXTURES: &'static [&'static str] = &[
    // PUT TEXTURE HERE
    "ship.png",
    "go1.png",
    "bg.jpg",
    "bang1.png",
];

pub struct AssetManager {
    textures: HashMap<String, SfBox<Texture>>,
}

impl AssetManager {
    pub fn new() -> AssetManager {
        AssetManager {
            textures: HashMap::new(),
        }
    }

    pub fn load_textures(&mut self, textures: &'static [&'static str]) {
        for path in textures {
            self.textures.insert(
                path.split('.').next()
                    .expect(&format!("ERROR: Failed to get extension of file '{}' at assets::AssetManager::load_texture", path)).to_string(),
                Texture::from_file(path).expect(&format!(
                    "ERROR: Cannot load texture '{}' at assets::AssetManager::load_texture",
                    path
                )),
            );
        }
    }

    pub fn get_texture(&self, name: &str) -> &Texture {
        self.textures.get(name).expect(&format!(
            "ERROR: Texture '{}' not found. At assets::AssetManager::get_texture",
            name
        ))
    }
}
