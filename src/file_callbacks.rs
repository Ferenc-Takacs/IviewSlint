use crate::MainWindow; // A build.rs által generált típus
use crate::image_processing::*;
use crate::colors::*;
//use crate::gpu_colors::*;

use slint::{Window, PhysicalSize};
use slint::ComponentHandle;
//use slint::*;
use std::rc::Rc;
use std::cell::RefCell;
use crate::ImageViewer;
use std::env;
use std::path::PathBuf;
use display_info::DisplayInfo;

pub fn file_callbacks(ui_weak: slint::Weak<MainWindow>, state: Rc<RefCell<ImageViewer>>) {
    let ui = ui_weak.unwrap();    
    let state_copy = state.clone();
    
    { // startup setting
        
        let display_info = DisplayInfo::all().unwrap()[0].clone();
       
        let args: Vec<String> = env::args().collect();
        let (start_image, clipboard) = if args.len() > 1 {
            // Ha van argumentum, azt útvonalként kezeljük
            (Some(PathBuf::from(&args[1])), false)
        } else {
            // 2. Ha nincs, megnézzük a vágólapot (Ctrl+C-vel másolt kép)
            (save_clipboard_image(), true)
        };
        
        let mut viewer = state_copy.borrow_mut();
        viewer.screen_size = (display_info.width as f32, display_info.height as f32);
        
        let w = ui.get_screen_width();
        let h = ui.get_screen_height();
        let pos_x = (viewer.screen_size.0 - w) / 2.0;
        let pos_y = (viewer.screen_size.1 - h) / 2.0;

        ui.window().set_position(slint::PhysicalPosition::new(pos_x as i32, pos_y as i32));
        println!("{:?}",display_info);
        viewer.load_settings();
        
        if let Some(path) = start_image {
            if clipboard {
                // az előző futás könyvtárát vesszük
                viewer.make_image_list()
            }
            viewer.open_image(&path, !clipboard);
        } else {
            viewer.open_image_dialog(&None);
        }
        if viewer.config.recent_files.len() == 0 {
            viewer.add_to_recent(&PathBuf::from("c:\\rust\\a.jpg"));
            viewer.add_to_recent(&PathBuf::from("c:\\rust\\b.jpg"));
            viewer.add_to_recent(&PathBuf::from("c:\\rust\\c.jpg"));
        }

        viewer.refresh_recent_list();
        let rec = &viewer.config.recent_files;
        println!("{:?}",viewer.screen_size);
    }
    let value = state_copy.clone();
    ui.on_copy_image(move || {
        println!("Copy");
        let mut viewer = value.borrow_mut();
        viewer.save_original = true;
        viewer.copy_to_clipboard();
    });

    let value = state_copy.clone();
    ui.on_copy_view(move || {
        println!("Copy View");
        let mut viewer = value.borrow_mut();
        viewer.save_original = false;
        viewer.copy_to_clipboard();
    });

    let value = state_copy.clone();
    ui.on_paste_image( move || {
        println!("Paste");
        let mut viewer = value.borrow_mut();
        viewer.copy_from_clipboard();
        // TODO show image
    });

    let value = state_copy.clone();
    ui.on_save_file(move || {
        println!("Save");
        let mut viewer = value.borrow_mut();
        viewer.save_original = true;
        viewer.starting_save(&None);
    });

    let value = state_copy.clone();
    ui.on_save_view( move || {
        println!("Save View");
        let mut viewer = value.borrow_mut();
        viewer.save_original = false;
        viewer.starting_save(&None);
    });

    /*let slint_img = ui.get_current_image(); // Ez kéri le a képet a GUI-ból

    if let Some(pixel_buffer) = slint_img.to_rgba8() {
        let img_width = pixel_buffer.width();
        let img_height = pixel_buffer.height();

        // Ablak méretezése
        ui.window().set_size(slint::PhysicalSize::new(
            img_width, 
            img_height + 30 // +30 pixel a menüsornak
        ));
    }*/

/*
// MEGJELENÍTÉS (Ez váltja ki az újrarajzolást!)
if let Some(ui) = ui_weak.upgrade() {
    ui.set_current_image(slint_img);
    ui.window().set_size(slint::PhysicalSize::new(w, h + 35));
}
//1. Nagyítás és Kicsinyítés (Zoom)
ui.set_zoom_level(0.5); // 50%-os kicsinyítés
ui.set_zoom_level(2.0); // 200%-os nagyítás
//2. Látható kezdőpozíció (Scroll / Offset)
flick := Flickable {
    viewport-x: root.offset_x; // Új property-k kellenek
    viewport-y: root.offset_y;
    // ...
}
//Beállítás Rust-ból:
ui.set_offset_x(- (kep_szelesseg * zoom / 2.0));
//3. Egér pozíciója a képen
img := Image {
    source: root.current_image;
    ta := TouchArea {}
}
// Ezt a koordinátát leolvashatod Rust-ban:
let pos = ui.get_mouse_pos(); // Ha csinálsz rá property-t
let valos_pixel_x = (mouse_x - ui.get_offset_x()) / ui.get_zoom_level();
//4. Kép méretezése az ablakhoz (Fit to Window)
let window_size = ui.window().size();
let scale_x = window_size.width as f32 / img_width as f32;
let scale_y = (window_size.height as f32 - 35.0) / img_height as f32;
let final_zoom = scale_x.min(scale_y);
ui.set_zoom_level(final_zoom);

*/

    let value = state_copy.clone();
    ui.on_open_recent(move |path| {
        let path_buf = PathBuf::from(path.as_str());
        println!("on_open_recent {:?}", path_buf);
        let mut viewer = value.borrow_mut();
        viewer.open_image(&path_buf,true);
    });
    
    let value = state_copy.clone();
    ui.on_open_here_recent(move |path| {
        let path_buf = PathBuf::from(path.as_str());
        println!("on_open_here_recent {:?}", path_buf);
        let mut viewer = value.borrow_mut();
        viewer.open_image_dialog(&Some(path_buf));
    });
    
    let value = state_copy.clone();
    ui.on_save_recent(move |path| {
        let path_buf = PathBuf::from(path.as_str());
        println!("on_save_recent {:?}", path_buf);
        let mut viewer = value.borrow_mut();
        viewer.save_original = true;
        viewer.starting_save(&Some(path_buf));
    });
    
    let value = state_copy.clone();
    ui.on_save_view_recent(move |path| {
        let path_buf = PathBuf::from(path.as_str());
        println!("on_save_view_recent {:?}", path_buf);
        let mut viewer = value.borrow_mut();
        viewer.save_original = false;
        viewer.starting_save(&Some(path_buf));
    });
    
    let value = state_copy.clone();
    ui.on_open_file(move || {
        println!("Open");
        let mut viewer = value.borrow_mut();
        viewer.open_image_dialog(&None);
    });
    
    let value = state_copy.clone();
    ui.on_change_image(move || {
        println!("Change");
        let mut viewer = value.borrow_mut();
        viewer.save_original = true;
        viewer.change_with_clipboard();
    });
    
    let value = state_copy.clone();
    ui.on_change_view(move || {
        println!("Change View");
        let mut viewer = value.borrow_mut();
        viewer.save_original = false;
        viewer.change_with_clipboard();
    });
    
    let value = state_copy.clone();
    ui.on_reopen_file(move || {
        println!("Reopen");
        let mut viewer = value.borrow_mut();
        viewer.load_image(true);
    });
    
    //let value = state_copy.clone();
    /*ui.on_recent_paths(move || {
        println!("Recent paths");
        //let mut viewer = value.borrow_mut();
        // TODO !!!! viewer.show_recent_window = !self.show_recent_window && !self.config.recent_files.is_empty();
    });*/
    
    let value = state_copy.clone();
    ui.on_prev_image(move || {
        println!("Előző kép (Back)");
        let mut viewer = value.borrow_mut();
        viewer.navigation(-1);
    });

    let value = state_copy.clone();
    ui.on_next_image(move || {
        println!("Következő kép (Next)");
        let mut viewer = value.borrow_mut();
        viewer.navigation(1);
    });
    
    //let value = state_copy.clone();
    ui.on_info_clicked(move || {
        println!("Info");
        //let mut viewer = value.borrow_mut();
    });

    let value = state_copy.clone();
    ui.on_change_background(move |mode| {
        println!("on_change_background");
        let mut viewer = value.borrow_mut();
        let bkgrd = if mode >= 0 { BackgroundStyle::from(mode) } else { viewer.bg_style.clone().inc() };
        viewer.bg_style = bkgrd;
    });

    let value = state_copy.clone();
    ui.on_down(move || {
        println!("on_down");
        let mut viewer = value.borrow_mut();
        // rotate  to 0
        let r = viewer.color_settings.rotate == Rotate::Rotate90
            || viewer.color_settings.rotate == Rotate::Rotate270;
        viewer.color_settings.rotate = Rotate::Rotate0;
        viewer.review(true, r);
    });

    let value = state_copy.clone();
    ui.on_up(move || {
        println!("on_up");
        let mut viewer = value.borrow_mut();
        // rotate 180
        viewer.color_settings.rotate = viewer.color_settings.rotate.add(Rotate::Rotate180);
        viewer.review(true, false);
    });

    let value = state_copy.clone();
    ui.on_left(move || {
        println!("on_left");
        let mut viewer = value.borrow_mut();
        // rotate -90
        viewer.color_settings.rotate = viewer.color_settings.rotate.add(Rotate::Rotate270);
        viewer.review(true, true);
    });

    let value = state_copy.clone();
    ui.on_right(move || {
        println!("on_right");
        let mut viewer = value.borrow_mut();
        // rotate 90
        viewer.color_settings.rotate = viewer.color_settings.rotate.add(Rotate::Rotate90);
        viewer.review(true, true);
    });

    let value = state_copy.clone();
    ui.on_plus(move || {
        println!("on_plus");
        let mut viewer = value.borrow_mut();
        viewer.change_magnify = 1.0;

    });

    let value = state_copy.clone();
    ui.on_minus(move || {
        println!("on_minus");
        let mut viewer = value.borrow_mut();
        viewer.change_magnify = -1.0;
    });

    let value = state_copy.clone();
    ui.on_red_channel(move || {
        println!("on_red_channel");
        let mut viewer = value.borrow_mut();
        viewer.color_settings.show_r = !viewer.color_settings.show_r;
        viewer.review(true, false);
    });

    let value = state_copy.clone();
    ui.on_green_channel(move || {
        println!("on_green_channel");
        let mut viewer = value.borrow_mut();
        viewer.color_settings.show_g = !viewer.color_settings.show_g;
        viewer.review(true, false);
    });

    let value = state_copy.clone();
    ui.on_blue_channel(move || {
        println!("on_blue_channel");
        let mut viewer = value.borrow_mut();
        viewer.color_settings.show_b = !viewer.color_settings.show_b;
        viewer.review(true, false);
    });

    let value = state_copy.clone();
    ui.on_invert_channels(move || {
        println!("on_invert_channels");
        let mut viewer = value.borrow_mut();
        viewer.color_settings.invert = !viewer.color_settings.invert;
        viewer.review(true, false);
    });

    //let value = state_copy.clone();
    ui.on_color_setting(move || {
        println!("on_color_setting");
        //let mut viewer = value.borrow_mut();
        // TODO !!!!    self.color_correction_dialog = !self.color_correction_dialog;
    });

    //let value = state_copy.clone();
    //ui.on_about(move || {
    //    println!("on_about");
    //    //let mut viewer = value.borrow_mut();
    //});
    
    ui.on_exit(move || {
        println!("exit");
        if let Some(ui) = ui_weak.upgrade() {
            let _ = ui.window().hide();
        }
    });
    
    // Példa Timer indítására a Play gombra
    //let timer = slint::Timer::default();
    //let value = state_copy.clone();
    ui.on_play_animation(move || {
        println!("Play/Stop");
        //let mut viewer = value.borrow_mut();
        //timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(100), || {
            // Következő képkocka betöltése...
    });
    ui.on_begin_animation(move || {
        println!("on_begin_animation");
    });
    ui.on_back_animation(move || {
        println!("on_back_animation");
    });
    ui.on_forward_animation(move || {
        println!("on_forward_animation");
    });
    ui.on_loop_animation(move || {
        println!("on_loop_animation");
    });
}

