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
use std::fs;
use std::path::PathBuf;
//use slint::ModelHandle;
use std::rc::Rc;
use std::cell::RefCell;

fn main() -> Result<(), slint::PlatformError> {
    //let icon = load_icon();
    let ui = MainWindow::new()?;
    let state = Rc::new(RefCell::new(ImageViewer::default()));
    state.borrow_mut().ui_handle = Some(ui.as_weak());
    file_callbacks::file_callbacks(ui.as_weak(), state);
    ui.run()
}

pub struct ImageViewer {
    pub ui_handle: Option<slint::Weak<MainWindow>>,
    pub image_full_path: Option<PathBuf>,
    pub file_meta: Option<fs::Metadata>,
    pub exif: Option<ExifBlock>,
    pub image_name: String,
    pub image_format: SaveFormat,
    pub image_folder: Option<PathBuf>,
    pub list_of_images: Vec<fs::DirEntry>,
    pub actual_index: usize,
    pub magnify: f32,
    pub resize: f32,
    pub first_appear: u32,
    
    // Slint kompatibilis kép tárolás
    pub current_slint_image: Option<slint::Image>, 
    pub original_image: Option<image::DynamicImage>,
    pub rgba_image: Option<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
    
    pub image_size: (f32, f32), 
    pub center: bool,
    pub show_info: bool,
    pub aktualis_offset: (f32, f32),
    pub sort: SortDir,
    pub save_original: bool,
    pub save_dialog: Option<SaveSettings>,
    pub color_settings: ColorSettings,
    pub lut: Option<Lut4ColorSettings>,
    pub gpu_interface : Option<gpu_colors::GpuInterface>,
    pub gpu_tried_init: bool,
    pub use_gpu: bool,
    pub refit_reopen: bool,
    pub fit_open: bool,
    pub same_correction_open: bool,
    pub bg_style: BackgroundStyle,
    pub config: AppSettings,
    pub resolution: Option<Resolution>,
    // Animáció kezelés (Slint-ben a Timer fogja hajtani)
    pub recent_file_modified: bool,
    pub is_animated: bool,
    pub anim_playing: bool,
    pub anim_loop: bool,
    pub current_frame: usize,
    pub last_frame_time: std::time::Instant,
    pub anim_data: Option<AnimatedImage>,
    pub change_magnify: f32,
    pub show_original_only: bool,
    pub modified: bool,
    // A többi meződet (settings, gpu, stb.) fokozatosan tölthetjük be
}

impl Default for ImageViewer {
    fn default() -> Self {
        Self {
            ui_handle : None,
            image_full_path: None,
            file_meta: None,
            exif: None,
            image_name: String::new(),
            image_format: SaveFormat::Bmp,
            image_folder: None,
            list_of_images: Vec::new(),
            actual_index: 0,
            magnify: 1.0,
            resize: 1.0,
            first_appear: 1,
            current_slint_image: None,
            original_image: None,
            rgba_image: None,
            image_size: (800.0, 600.0),
            center: true,
            show_info: false,
            aktualis_offset: (0.0, 0.0),
            sort: SortDir::Name,
            save_original: false, //always set before use
            save_dialog: None,
            color_settings: ColorSettings::default(),
            lut: None,
            gpu_interface : None,
            gpu_tried_init: false,
            use_gpu: true,
            refit_reopen: false,
            fit_open: true,
            same_correction_open: false,
            bg_style: BackgroundStyle::DarkBright,
            config: AppSettings::default(),
            resolution: None,
            recent_file_modified: false,
            is_animated: false,
            anim_playing: false,
            anim_loop: true,
            current_frame: 0,
            last_frame_time: std::time::Instant::now(),
            anim_data: None,
            change_magnify: 0.0,
            show_original_only: false,
            modified: false,
        }
    }
}