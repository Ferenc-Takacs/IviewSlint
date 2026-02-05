use serde::{Deserialize, Serialize};
use arboard::Clipboard;
use std::path::PathBuf;
use std::env;
use crate::ImageViewer;
use crate::colors::*;
use slint::{Color, Image, SharedPixelBuffer, Rgba8Pixel, ComponentHandle};
use crate::Pf32;
//use image::math::Rect;
use crate::ImageState;

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
    pub fn to(self) -> i32 {
        match self {
            BackgroundStyle::Black => 0,
            BackgroundStyle::Gray => 1,
            BackgroundStyle::White => 2,
            BackgroundStyle::Green => 3,
            BackgroundStyle::DarkBright => 4,
            BackgroundStyle::GreenMagenta => 5,
            BackgroundStyle::BlackBrown => 6,
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
            self.want_magnify = -1.0; // modified image width:height ratio
        }

        let mut rgba_image = img.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        self.image_size = ( width, height ).into();
        
        if let Some(interface) = &self.gpu_interface {
            interface.change_colorcorrection(
                if self.show_original_only { &default_settings } else { &self.color_settings },
                self.image_size.x,
                self.image_size.y);
        }

        self.resize = self.image_size.x / w_orig as f32;
        
        if self.color_settings.is_setted() || self.color_settings.is_blured() {
            if self.gpu_interface.is_some() {
                
                self.gpu_interface.as_ref().unwrap().generate_image(rgba_image.as_mut(), width, height);
                            } else if let Some(lut) = &self.lut {
                lut.apply_lut(&mut rgba_image); 
            }
        }
        self.rgba_image = Some(rgba_image.clone());
        

        let slint_pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
            rgba_image.as_raw(), 
            width, 
            height
        );
        let slint_img = Image::from_rgba8(slint_pixel_buffer);
        self.sizing_window(slint_img);
    }


    fn sizing_window(&mut self, slint_img: slint::Image){

        let old_magnify = self.magnify;
        let old_size = self.image_size * old_magnify;
        let mut old_offset = self.aktualis_offset;
        if let Some(handle) = &self.ui_handle {
            if let Some(ui) = handle.upgrade() {
                old_offset.x = ui.get_offset_x();
                old_offset.y = ui.get_offset_y();
            }
        }
        let display_size_netto = self.display_size - self.window_frame;
        let mut bigger = 1.0;
        
        if self.want_magnify == -1.0 { // set size to fit
            let ratio = display_size_netto / self.image_size; // divide by tags
            self.magnify = ratio.x.min(ratio.y);

            if !self.rgba_image.is_some() {
                self.magnify *= 0.5; // empty window
            }
            //let round_ = if self.magnify < 1.0 { 0.0 } else { 0.5 };
            self.magnify = (((self.magnify * 20.0 ) as i32) as f32) / 20.0;
        }

        if self.change_magnify != 0.0 || self.want_magnify > 0.009 {
            if self.want_magnify > 0.009 { // from menu
                self.magnify = self.want_magnify;
            }
            else {
                if self.magnify >= 1.0 {
                    self.change_magnify *= 2.0;
                }
                else if self.magnify >= 4.0 {
                    self.change_magnify *= 2.0;
                }
                self.magnify = (old_magnify * 1.0 + (0.05 * self.change_magnify)).clamp(0.1, 10.0);
                self.magnify = (((self.magnify * 100.0 + 0.5) as i32) as f32) / 100.0; // round
            }
            bigger = self.magnify / old_magnify;
        }
        
        let zero = Pf32 { x: 0.0, y: 0.0 };
        let mut off = Pf32 { x: 0.0, y: 0.0 };
        let new_size = self.image_size * self.magnify;
        let inner_size = new_size.min(display_size_netto);
        let pos = if self.center { (display_size_netto - inner_size) * 0.5 } else { zero };

        
        //if new_size.x > inner_size.x || new_size.y > inner_size.y {
            //if bigger != 1.0 || self.want_magnify > 0.009 {
                
                let mut pointer = if self.mouse_zoom {
                        self.mouse_pos
                    } else {
                        old_size * 0.5
                    };
                let rel_pos = (old_offset + pointer).max(zero);
                off = (rel_pos - pointer * bigger).min(zero);
                if new_size.x == inner_size.x { off.x = 0.0; } 
                if new_size.y == inner_size.y { off.y = 0.0; } 
                //println!("{:?} {:?} {:?} ",old_offset,off, pointer);
            //}
        //}

        //if need_set || old_offset != off {
            if let Some(handle) = &self.ui_handle {
                if let Some(ui) = handle.upgrade() {
                    //println!("{:?} {:?} {:?} {:?} {:?} ", inner_size, pos, off, self.magnify, self.center);
                    let title: slint::SharedString = format!("iViewer - {}. {}{}   {}",
                        self.actual_index, self.image_name, if self.modified {'*'} else {' '},  self.magnify).into();
                    if bigger != 1.0 || self.want_magnify == -1.0 {
                        ui.window().set_position(slint::PhysicalPosition::new(pos.x as i32, pos.y as i32));
                    }
                    ui.set_offset_x(off.x);
                    ui.set_offset_y(off.y);
                    let new_state = ImageState {
                        window_width: inner_size.x,
                        window_height: inner_size.y,
                        zoom_level: self.magnify,
                        current_image: slint_img,
                        window_title: title,
                    };
                    ui.set_img_state(new_state);
                }
            }
        //}

        /*if zoom != 1.0 {
            if let Some(handle) = &self.ui_handle {
                if let Some(ui) = handle.upgrade() {
                }
            }
        }*/
        self.aktualis_offset = off;
        self.want_magnify = 0.0;
        self.change_magnify = 0.0;
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