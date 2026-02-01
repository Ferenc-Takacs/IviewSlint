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
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::{Add,Sub,Mul,Div,AddAssign,SubAssign,MulAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pf32{ x: f32, y: f32, }
impl Default for Pf32 { fn default() -> Self { Self { x: 0.0, y: 0.0 } } }
impl Into<(f32,f32)> for Pf32 { fn into(self) -> (f32,f32) { ( self.x, self.y ) } }
impl Into<Pf32> for (f32,f32) { fn into(self) -> Pf32 { Pf32{ x:self.0, y:self.1 } } }
impl Into<(i32,i32)> for Pf32 { fn into(self) -> (i32,i32) { ( (self.x+0.5) as i32, (self.y+0.5) as i32 ) } }
impl Into<Pf32> for (i32,i32) { fn into(self) -> Pf32 { Pf32{ x:self.0 as f32, y:self.1 as f32 } } }
impl Into<(u32,u32)> for Pf32 { fn into(self) -> (u32,u32) { ( (self.x+0.5) as u32, (self.y+0.5) as u32 ) } }
impl Into<Pf32> for (u32,u32) { fn into(self) -> Pf32 { Pf32{ x:self.0 as f32, y:self.1 as f32 } } }
impl Add for Pf32 { type Output = Self;
    fn add(self, a: Self) -> Self { Pf32{ x: self.x + a.x, y: self.y + a.y } }
}
impl AddAssign for Pf32 {
    fn add_assign(&mut self, b: Self) { *self = Self { x: self.x + b.x, y: self.y + b.y, }; }
}
impl Sub for Pf32 { type Output = Self;
    fn sub(self, a: Self) -> Self { Pf32{ x: self.x - a.x, y: self.y - a.y } }
}
impl SubAssign for Pf32 {
    fn sub_assign(&mut self, b: Self) { *self = Self { x: self.x - b.x, y: self.y - b.y, }; }
}
impl Mul<f32> for Pf32 { type Output = Self;
    fn mul(self, a: f32) -> Self { Pf32{ x: self.x * a, y: self.y * a } }
}
impl MulAssign<f32> for Pf32 {
    fn mul_assign(&mut self, b: f32) { *self = Self { x: self.x * b, y: self.y * b, }; }
}
impl Mul for Pf32 { type Output = f32;
    fn mul(self, a: Self) -> f32 { self.x * a.x + self.y * a.y }
}
impl Div<f32> for Pf32 { type Output = Self;
    fn div(self, a: f32) -> Self { Pf32{ x: self.x / a, y: self.y / a } }
}
impl Div for Pf32 { type Output = Pf32;
    fn div(self, a: Self) -> Pf32 { Pf32{ x: self.x / a.x, y: self.y / a.y } }
}
impl Pf32 {
    fn hypot(self, b: Pf32) -> f32 { (self - b).length() }
    fn length( self ) -> f32 {  (self.x*self.x+self.y*self.y).sqrt() }
    fn min( self, b: Pf32 ) -> Pf32 { Pf32{ x: self.x.min(b.x), y: self.y.min(b.y) } }
    fn max( self, b: Pf32 ) -> Pf32 { Pf32{ x: self.x.max(b.x), y: self.y.max(b.y) } }
}


fn main() -> Result<(), slint::PlatformError> {
    //let icon = load_icon();
    let ui = MainWindow::new()?;
    let state = Rc::new(RefCell::new(ImageViewer::default()));
    state.borrow_mut().ui_handle = Some(ui.as_weak());
    file_callbacks::file_callbacks(ui.as_weak(), state.clone());
    let res = ui.run();
    let mut viewer = state.borrow_mut();
    viewer.save_settings();
    res
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
    pub change_magnify: f32,
    pub want_magnify: f32,
    pub resize: f32,
    
    // Slint kompatibilis kép tárolás
    pub current_slint_image: Option<slint::Image>, 
    pub original_image: Option<image::DynamicImage>,
    pub rgba_image: Option<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
    
    pub display_size: Pf32, 
    pub window_frame: Pf32, // title, menu, padding, rendszer tálca
    pub window_size: Pf32, 
    pub image_size: Pf32, 
    pub mouse_pos: Pf32,
    pub mouse_zoom: bool,
    pub center: bool,
    pub show_info: bool,
    pub aktualis_offset: Pf32,
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
            change_magnify: 0.0,
            want_magnify: -1.0,
            resize: 1.0,
            current_slint_image: None,
            original_image: None,
            rgba_image: None,
            display_size: Pf32{x:1280.0, y:1024.0},
            window_frame: Pf32{ x:10.0 , y:60.0 }, // title, menu, padding, rendszer tálca
            window_size: Pf32{x:800.0, y:600.0},
            image_size: Pf32{x:800.0, y:600.0},
            mouse_pos: Pf32{x:0.0, y:0.0},
            mouse_zoom: false,
            center: true,
            show_info: false,
            aktualis_offset: Pf32::default(),
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
            show_original_only: false,
            modified: false,
        }
    }
}