use serde::{Deserialize, Serialize};
use arboard::Clipboard;
use std::path::PathBuf;
use std::env;
use crate::ImageViewer;
use crate::colors::*;
use slint::Color;
//use image::math::Rect;

// Segédfüggvény a vágólapon lévő kép kimentéséhez egy ideiglenes fájlba
pub fn save_clipboard_image() -> Option<PathBuf> {
    let mut clipboard = Clipboard::new().ok()?;
    if let Ok(image_data) = clipboard.get_image() {
        let temp_path = env::temp_dir().join("rust_image_viewer_clipboard.png");
        // Konvertálás arboard formátumból image formátumba
        if let Some(buf) = image::ImageBuffer::<image::Rgba<u8>, std::vec::Vec<u8>>::from_raw(
            image_data.width as u32,
            image_data.height as u32,
            image_data.bytes.into_owned(),
        ) {
            if buf.save(&temp_path).is_ok() {
                return Some(temp_path);
            }
        }
    }
    None
}


#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum BackgroundStyle {
    Black,
    Gray,
    White,
    Green,
    DarkBright,
    GreenMagenta,
    BlackBrown,
}

impl BackgroundStyle {
    pub fn inc(self) -> BackgroundStyle {
        match self {
            BackgroundStyle::Black => BackgroundStyle::Gray,
            BackgroundStyle::Gray => BackgroundStyle::White,
            BackgroundStyle::White => BackgroundStyle::Green,
            BackgroundStyle::Green => BackgroundStyle::DarkBright,
            BackgroundStyle::DarkBright => BackgroundStyle::GreenMagenta,
            BackgroundStyle::GreenMagenta => BackgroundStyle::BlackBrown,
            BackgroundStyle::BlackBrown => BackgroundStyle::Black,
        }
    }
    pub fn from(i: i32) -> BackgroundStyle {
        match i {
            1 => BackgroundStyle::Gray,
            2 => BackgroundStyle::White,
            3 => BackgroundStyle::Green,
            4 => BackgroundStyle::DarkBright,
            5 => BackgroundStyle::GreenMagenta,
            6 => BackgroundStyle::BlackBrown,
            _ => BackgroundStyle::Black,
        }
    }
}


#[derive(Clone, Debug)]
pub struct Resolution {
    pub xres: f32,
    pub yres: f32,
    pub dpi: bool,
}

pub struct AnimatedImage {
    //pub anim_frames: Vec<egui::TextureHandle>, // GPU textúrák // old
    pub anim_frames: Vec<image::DynamicImage>,
    pub delays: Vec<std::time::Duration>, // Időzítések
    pub total_frames: usize,
}

/*pub fn color_image_to_dynamic(color_image: ColorImage) -> image::DynamicImage {
    let size = color_image.size;
    // Flatten Color32 (RGBA) pixels into a Vec<u8>
    let pixels = color_image.pixels.iter()
        .flat_map(|p| [p.r(), p.g(), p.b(), p.a()])
        .collect::<Vec<u8>>();

    // Create an RgbaImage buffer
    let buffer = image::RgbaImage::from_raw(size[0] as u32, size[1] as u32, pixels)
        .expect("Failed to create image buffer");

    // Wrap in DynamicImage
    image::DynamicImage::ImageRgba8(buffer)
}*/

pub fn draw_custom_background(bg_style: &BackgroundStyle) {
    /*let rect = Rect::new(); // TODO !!!! ui.max_rect(); // A terület, ahová a kép kerülne
    if rect.width() <= 0.0 {
        return;
    }
    let paint = ui.painter();
    let (col1, col2) = if *bg_style == BackgroundStyle::DarkBright {
        (Color::from_rgb_u8(35,35,35), Color::from_rgb_u8(70,70,70))
    } else if *bg_style == BackgroundStyle::GreenMagenta {
        (
            Color::from_rgb_u8(40, 180, 40),
            Color::from_rgb_u8(180, 50, 180),
        )
    } else if *bg_style == BackgroundStyle::BlackBrown {
        (
            Color::from_rgb_u8(0, 0, 0),
            Color::from_rgb_u8(200, 50, 10),
        )
    } else {
        (Color::from_rgb_u8(0,0,0), Color::from_rgb_u8(255,255,255))
    };
    match *bg_style {
        BackgroundStyle::Black => {
            paint.rect_filled(rect, Color::from_rgb_u8(0,0,0));
        }
        BackgroundStyle::White => {
            paint.rect_filled(rect, 0.0, Color::from_rgb_u8(255,255,255));
        }
        BackgroundStyle::Gray => {
            paint.rect_filled(rect, 0.0, Color::from_rgb_u8(128,128,128));
        }
        BackgroundStyle::Green => {
            paint.rect_filled(rect, 0.0, Color::from_rgb_u8(50, 200, 50));
        }
        _ => {
            paint.rect_filled(rect, 0.0, col1);
            let tile_size = 16.0; // A négyzetek mérete pixelben
            let color_light = col2;
            let num_x = (rect.width() / tile_size).ceil() as i32 + 1;
            let num_y = (rect.height() / tile_size).ceil() as i32 + 1;
            for y in 0..=num_y {
                for x in 0..=num_x {
                    if (x + y) % 2 == 0 {
                        let tile_rect = Rect::from_min_size(
                            pos2(
                                rect.left() + x as f32 * tile_size,
                                rect.top() + y as f32 * tile_size,
                            ),
                            vec2(tile_size, tile_size),
                        );
                        let visible_tile = tile_rect.intersect(rect);
                        if visible_tile.width() > 0.0 && visible_tile.height() > 0.0 {
                            paint.rect_filled(visible_tile, 0.0, color_light);
                        }
                    }
                }
            }
        }
    }*/
}

impl ImageViewer {

    pub fn review(&mut self, coloring: bool, new_rotate: bool) {
        if let Some(mut img) = self.original_image.clone() {
            self.review_core(&mut img, coloring, new_rotate)
        }
    }
    
    fn review_core(&mut self, img: & mut image::DynamicImage, coloring: bool, new_rotate: bool) {
        let default_settings = ColorSettings::default();
        if coloring {
            if let Some(_interface) = &self.gpu_interface {
            }
            else {
                let lut_ref = self.lut.get_or_insert_with(Lut4ColorSettings::default);
                lut_ref.update_lut( if self.show_original_only { &default_settings} else { &self.color_settings} );
            }
        } else {
            self.lut = None;
            self.color_settings = default_settings.clone();
        }

        let max_gpu_size = 4096;// TODO !!! ctx.input(|i| i.max_texture_side) as u32;
        let w_orig = img.width();
        if img.width() > max_gpu_size || img.height() > max_gpu_size {
            *img = img.resize(
                max_gpu_size,
                max_gpu_size,
                image::imageops::FilterType::Triangle,
            );
        }
        self.modified = !self.show_original_only &&
                (self.color_settings.is_setted() || self.color_settings.is_blured());
        match self.color_settings.rotate {
            Rotate::Rotate90  => { *img = img.rotate90() ; self.modified = true; }, 
            Rotate::Rotate180 => { *img = img.rotate180(); self.modified = true; },
            Rotate::Rotate270 => { *img = img.rotate270(); self.modified = true; },
            _ => {}
        }
        if new_rotate {
            self.first_appear = 1;
        }

        let mut rgba_image = img.to_rgba8();
        self.image_size.0 = rgba_image.dimensions().0 as f32;
        self.image_size.1 = rgba_image.dimensions().1 as f32;
        
        if let Some(interface) = &self.gpu_interface {
            interface.change_colorcorrection(
                if self.show_original_only { &default_settings } else { &self.color_settings },
                self.image_size.0,
                self.image_size.1);
        }

        self.resize = self.image_size.0 / w_orig as f32;
        
        if self.color_settings.is_setted() || self.color_settings.is_blured() {
            if self.gpu_interface.is_some() {
                let (width, height) = rgba_image.dimensions();
                self.gpu_interface.as_ref().unwrap().generate_image(rgba_image.as_mut(), width, height);
                            } else if let Some(lut) = &self.lut {
                lut.apply_lut(&mut rgba_image); 
            }
        }

        self.rgba_image = Some(rgba_image.clone());
        let pixel_data = rgba_image.into_raw();
        //let color_image = egui::ColorImage::from_rgba_unmultiplied(
        //    [self.image_size.0 as usize, self.image_size.1 as usize],
        //    &pixel_data,
        //);
        // TODO !!!!
        //self.texture = Some(ctx.load_texture("kep", color_image, Default::default()));
    }

    pub fn pick_color(&self, pixel_x : u32,pixel_y: u32) -> Option<Color> {
        if let Some(rgba_image) = &self.rgba_image {
            if pixel_x < rgba_image.width() && pixel_y < rgba_image.height() {
                let pixel = rgba_image.get_pixel(pixel_x, pixel_y);
                return Some(Color::from_rgb_u8(pixel[0], pixel[1], pixel[2]));
            }
        }
        None
    }

    pub fn navigation(&mut self, irany: i32) {
        if self.list_of_images.is_empty() {
            return;
        }
        let uj_index = if irany > 0 {
            (self.actual_index + 1) % self.list_of_images.len()
        } else {
            (self.actual_index + self.list_of_images.len() - 1) % self.list_of_images.len()
        };
        self.actual_index = uj_index;
        self.open_image(&self.list_of_images[uj_index].path(), false);
    }

}