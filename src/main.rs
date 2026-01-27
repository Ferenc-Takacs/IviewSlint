slint::include_modules!();

mod file_callbacks;
mod gpu_colors;
mod colors;
mod file_handlers;
mod image_processing;
mod exif_my;
use colors::*;
use crate::image_processing::*;
use crate::file_handlers::*;
use crate::exif_my::*;
use std::path::PathBuf;
use std::fs;
use std::env;

//use slint::ModelHandle;
use std::rc::Rc;
use std::cell::RefCell;

fn main() -> Result<(), slint::PlatformError> {
    let icon = load_icon();
    let ui = MainWindow::new()?;
    let state = Rc::new(RefCell::new(ImageViewer::default()));
    file_callbacks::file_callbacks(ui.as_weak(), state);
    ui.run()
}

pub struct ImageViewer {
    pub image_full_path: Option<PathBuf>,
    pub file_meta: Option<fs::Metadata>,
    // pub exif: Option<ExifBlock>, // Ha átmásoltad a típust
    pub image_name: String,
    // pub image_format: SaveFormat,
    pub image_folder: Option<PathBuf>,
    pub list_of_images: Vec<fs::DirEntry>,
    pub actual_index: usize,
    pub magnify: f32,
    
    // Slint kompatibilis kép tárolás
    pub current_slint_image: Option<slint::Image>, 
    pub original_image: Option<image::DynamicImage>,
    pub rgba_image: Option<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
    
    pub image_size: (f32, f32), 
    pub center: bool,
    pub show_info: bool,
    pub aktualis_offset: (f32, f32),
    
    // Animáció kezelés (Slint-ben a Timer fogja hajtani)
    pub is_animated: bool,
    pub anim_playing: bool,
    pub anim_loop: bool,
    pub current_frame: usize,
    pub last_frame_time: std::time::Instant,
    
    pub modified: bool,
    // A többi meződet (settings, gpu, stb.) fokozatosan tölthetjük be
}

impl Default for ImageViewer {
    fn default() -> Self {
        Self {
            image_full_path: None,
            file_meta: None,
            image_name: String::new(),
            image_folder: None,
            list_of_images: Vec::new(),
            actual_index: 0,
            magnify: 1.0,
            current_slint_image: None,
            original_image: None,
            rgba_image: None,
            image_size: (0.0, 0.0),
            center: true,
            show_info: false,
            aktualis_offset: (0.0, 0.0),
            is_animated: false,
            anim_playing: false,
            anim_loop: true,
            current_frame: 0,
            last_frame_time: std::time::Instant::now(),
            modified: false,
        }
    }
}