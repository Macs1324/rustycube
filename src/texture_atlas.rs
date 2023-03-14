use std::io::Cursor;

use glium::{backend::Facade, texture::RawImage2d};

pub struct TextureAtlas {
    pub filename: String,
    texture: glium::Texture2d,
}

// impl TextureAtlas {
//     pub fn load(display: &dyn Facade, filename: String) -> TextureAtlas {
//         TextureAtlas {
//             filename,
//             texture: {
//                 let image = image::load(
//                     Cursor::new(include_bytes!("Culo.txt")),
//                     image::ImageFormat::Png,
//                 )
//                 .unwrap()
//                 .to_rgba8();
//                 let image_dimensions = image.dimensions();
//
//                 let image = glium::texture::RawImage2d::from_raw_rgba_reversed(
//                     &image.into_raw(),
//                     image_dimensions,
//                 );
//
//                 glium::texture::SrgbTexture2d::new(display, image).unwrap()
//             },
//         }
//     }
// }
