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
    numof_block_faces: usize,
    id_texture_map: HashMap<BlockId, BlockUv>,
    texture: glium::texture::SrgbTexture2d,
}

impl TextureAtlas {
    pub fn load(display: &dyn Facade, filename: String) -> TextureAtlas {
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

                let image =
                    glium::texture::RawImage2d::from_raw_rgba(image.into_raw(), image_dimensions);

                glium::texture::SrgbTexture2d::new(display, image).unwrap()
            },

            id_texture_map: HashMap::new(),
            numof_blocks: 0,
            numof_block_faces: 0,
        }
    }

    pub fn with_blocks(mut self, numof_block_faces: usize, blocks: &Vec<BlockId>) -> Self {
        self.numof_blocks = blocks.len();
        self.numof_block_faces = numof_block_faces;
        let w = 1.0 / self.numof_blocks as f32;
        let h = 1.0 / self.numof_block_faces as f32;

        for (i, block) in blocks.iter().enumerate() {
            let x = i as f32 * w;

            self.id_texture_map.insert(
                *block,
                BlockUv {
                    top: [
                        TextureAtlasCoords {
                            uv_x: x + w,
                            uv_y: 0.0,
                        },
                        TextureAtlasCoords {
                            uv_x: x + w,
                            uv_y: h,
                        },
                        TextureAtlasCoords { uv_x: x, uv_y: h },
                        TextureAtlasCoords { uv_x: x, uv_y: 0.0 },
                    ],
                    left: [
                        TextureAtlasCoords {
                            uv_x: x + w,
                            uv_y: h,
                        },
                        TextureAtlasCoords {
                            uv_x: x + w,
                            uv_y: h * 2.0,
                        },
                        TextureAtlasCoords {
                            uv_x: x,
                            uv_y: h * 2.0,
                        },
                        TextureAtlasCoords { uv_x: x, uv_y: h },
                    ],
                    front: [
                        TextureAtlasCoords {
                            uv_x: x + w,
                            uv_y: h * 2.0,
                        },
                        TextureAtlasCoords {
                            uv_x: x + w,
                            uv_y: h * 3.0,
                        },
                        TextureAtlasCoords {
                            uv_x: x,
                            uv_y: h * 3.0,
                        },
                        TextureAtlasCoords {
                            uv_x: x,
                            uv_y: h * 2.0,
                        },
                    ],
                    right: [
                        TextureAtlasCoords {
                            uv_x: x + w,
                            uv_y: h * 3.0,
                        },
                        TextureAtlasCoords {
                            uv_x: x + w,
                            uv_y: h * 4.0,
                        },
                        TextureAtlasCoords {
                            uv_x: x,
                            uv_y: h * 4.0,
                        },
                        TextureAtlasCoords {
                            uv_x: x,
                            uv_y: h * 3.0,
                        },
                    ],

                    bottom: [
                        TextureAtlasCoords {
                            uv_x: x + w,
                            uv_y: h * 4.0,
                        },
                        TextureAtlasCoords {
                            uv_x: x + w,
                            uv_y: h * 5.0,
                        },
                        TextureAtlasCoords {
                            uv_x: x,
                            uv_y: h * 5.0,
                        },
                        TextureAtlasCoords {
                            uv_x: x,
                            uv_y: h * 4.0,
                        },
                    ],

                    back: [
                        TextureAtlasCoords {
                            uv_x: x + w,
                            uv_y: h * 5.0,
                        },
                        TextureAtlasCoords {
                            uv_x: x + w,
                            uv_y: h * 6.0,
                        },
                        TextureAtlasCoords {
                            uv_x: x,
                            uv_y: h * 6.0,
                        },
                        TextureAtlasCoords {
                            uv_x: x,
                            uv_y: h * 5.0,
                        },
                    ],
                },
            );
        }

        self
    }

    pub fn get_texture(&self) -> &SrgbTexture2d {
        &self.texture
    }

    pub fn get_block_uv(&self, block: BlockId) -> &BlockUv {
        self.id_texture_map
            .get(&block)
            .expect(format!("Block {:?} no configured!", block).as_str())
    }
}

pub struct BlockUv {
    pub top: [TextureAtlasCoords; 4],
    pub bottom: [TextureAtlasCoords; 4],
    pub left: [TextureAtlasCoords; 4],
    pub right: [TextureAtlasCoords; 4],
    pub front: [TextureAtlasCoords; 4],
    pub back: [TextureAtlasCoords; 4],
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
}
