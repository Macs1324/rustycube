use std::{
    collections::HashMap,
    fs::File,
    io::{Cursor, Read},
};

use glium::{backend::Facade, texture::SrgbTexture2d};

use crate::block::BlockId;

pub struct TextureAtlas {
    pub filename: String,
    numof_blocks: usize,
    id_texture_map: HashMap<BlockId, TextureAtlasCoords>,
    texture: glium::texture::SrgbTexture2d,
}

impl TextureAtlas {
    pub fn load(display: &dyn Facade, filename: String, numof_blocks: usize) -> TextureAtlas {
        TextureAtlas {
            filename: filename.clone(),
            texture: {
                let image = image::load(
                    {
                        let mut file = File::open(filename).unwrap();
                        let mut contents: Vec<u8> = Vec::new();
                        file.read_to_end(&mut contents).unwrap();
                        Cursor::new(contents)
                    },
                    image::ImageFormat::Png,
                )
                .unwrap()
                .to_rgba8();
                let image_dimensions = image.dimensions();

                let image = glium::texture::RawImage2d::from_raw_rgba(
                    image.into_raw(),
                    image_dimensions,
                );

                glium::texture::SrgbTexture2d::new(display, image).unwrap()
            },

            id_texture_map: HashMap::new(),
            numof_blocks,
        }
    }

    pub fn configure_block(&mut self, block_id: BlockId, uv_x: f32, uv_y: f32) {
        self.id_texture_map
            .insert(block_id, TextureAtlasCoords { uv_x, uv_y });
    }

    pub fn get_texture(&self) -> &SrgbTexture2d {
        &self.texture
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TextureAtlasCoords {
    pub uv_x: f32,
    pub uv_y: f32,
}

impl TextureAtlasCoords {
    pub fn new(uv_x: f32, uv_y: f32) -> TextureAtlasCoords {
        TextureAtlasCoords { uv_x, uv_y }
    }

    fn top(&self, horizontal_offset: f32) -> TextureAtlasCoords {
        TextureAtlasCoords {
            uv_x: self.uv_x + horizontal_offset,
            uv_y: self.uv_y,
        }
    }
}
