use crate::MainWindow; // A build.rs által generált típus
use crate::image_processing;

use slint::ComponentHandle;
use slint::*;
use arboard::*;
use rfd::FileDialog;
use std::rc::Rc;
use std::cell::RefCell;
use crate::ImageViewer;
use std::env;
use std::path::PathBuf;

pub fn file_callbacks(ui_weak: slint::Weak<MainWindow>, state: Rc<RefCell<ImageViewer>>) {
    let ui = ui_weak.unwrap();    
    let state_copy = state.clone();
    
    { // startup setting
        let args: Vec<String> = env::args().collect();
        let (start_image, clipboard) = if args.len() > 1 {
            // Ha van argumentum, azt útvonalként kezeljük
            (Some(PathBuf::from(&args[1])), false)
        } else {
            // 2. Ha nincs, megnézzük a vágólapot (Ctrl+C-vel másolt kép)
            (save_clipboard_image(), true)
        };
        
        let mut viewer = state_copy.borrow_mut();
        viewer.load_settings();
        
        if let Some(path) = start_image {
            if clipboard {
                // az előző könyvtárt vesszük
                viewer.make_image_list()
            }
            viewer.open_image(&path, !clipboard);
        } else {
            viewer.open_image_dialog(&None);
        }
    }
    
    ui.on_copy_image(move || {
        println!("Copy");
        let mut viewer = state_copy.borrow_mut();
        viewer.save_original = true;
        viewer.copy_to_clipboard();
    });

    ui.on_copy_view(move || {
        println!("Copy View");
        let mut viewer = state_copy.borrow_mut();
        viewer.save_original = false;
        viewer.copy_to_clipboard();
    });

    ui.on_paste_image( move || {
        println!("Paste");
        let mut viewer = state_copy.borrow_mut();
        viewer.copy_from_clipboard();
        // TODO show image
    });

    ui.on_save_file(move || {
        println!("Save");
        let mut viewer = state_copy.borrow_mut();
        viewer.save_original = true;
        viewer.starting_save(&None);
    });

    ui.on_save_view( move || {
        println!("Save View");
        let mut viewer = state_copy.borrow_mut();
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

    ui.on_open_file(move || {
        println!("Open");
        let mut viewer = state_copy.borrow_mut();
        viewer.open_image_dialog(&None);
    });
    
    ui.on_change_image(move || {
        println!("Change");
        let mut viewer = state_copy.borrow_mut();
        viewer.save_original = true;
        viewer.change_with_clipboard();
    });
    
    ui.on_change_view(move || {
        println!("Change View");
        let mut viewer = state_copy.borrow_mut();
        viewer.save_original = false;
        viewer.change_with_clipboard();
    });
    ui.on_reopen_file(move || {
        println!("Reopen");
        let mut viewer = state_copy.borrow_mut();
        viewer.load_image(true);
    });
    
    ui.on_recent_paths(move || {
        println!("Recent paths");
        let mut viewer = state_copy.borrow_mut();
        // TODO !!!! viewer.show_recent_window = !self.show_recent_window && !self.config.recent_files.is_empty();
    });
    
    ui.on_prev_image(move || {
        println!("Előző kép (Back)");
        let mut viewer = state_copy.borrow_mut();
        viewer.navigation(-1);
    });

    ui.on_next_image(move || {
        println!("Következő kép (Next)");
        let mut viewer = state_copy.borrow_mut();
        viewer.navigation(1);
    });
    
    ui.on_info_clicked(move || {
        println!("Info");
        let mut viewer = state_copy.borrow_mut();
    });

    ui.on_change_background(move || {
        println!("on_change_background");
        let mut viewer = state_copy.borrow_mut();
        viewer.bg_style = viewer.bg_style.clone().inc();
    });

    ui.on_down(move || {
        println!("on_down");
        let mut viewer = state_copy.borrow_mut();
        // rotate  to 0
        let r = viewer.color_settings.rotate == Rotate::Rotate90
            || viewer.color_settings.rotate == Rotate::Rotate270;
        viewer.color_settings.rotate = Rotate::Rotate0;
        viewer.review(true, r);
    });

    ui.on_up(move || {
        println!("on_up");
        let mut viewer = state_copy.borrow_mut();
        // rotate 180
        viewer.color_settings.rotate = viewer.color_settings.rotate.add(Rotate::Rotate180);
        viewer.review(true, false);
    });

    ui.on_left(move || {
        println!("on_left");
        let mut viewer = state_copy.borrow_mut();
        // rotate -90
        viewer.color_settings.rotate = viewer.color_settings.rotate.add(Rotate::Rotate270);
        viewer.review(true, true);
    });

    ui.on_right(move || {
        println!("on_right");
        let mut viewer = state_copy.borrow_mut();
        // rotate 90
        viewer.color_settings.rotate = viewer.color_settings.rotate.add(Rotate::Rotate90);
        viewer.review(true, true);
    });

    ui.on_plus(move || {
        println!("on_plus");
        let mut viewer = state_copy.borrow_mut();
    });

    ui.on_minus(move || {
        println!("on_minus");
        let mut viewer = state_copy.borrow_mut();
    });

    ui.on_red_channel(move || {
        println!("on_red_channel");
        let mut viewer = state_copy.borrow_mut();
        viewer.color_settings.show_r = !viewer.color_settings.show_r;
        viewer.review(true, false);
    });

    ui.on_green_channel(move || {
        println!("on_green_channel");
        let mut viewer = state_copy.borrow_mut();
        viewer.color_settings.show_g = !viewer.color_settings.show_g;
        viewer.review(true, false);
    });

    ui.on_blue_channel(move || {
        println!("on_blue_channel");
        let mut viewer = state_copy.borrow_mut();
        viewer.color_settings.show_b = !viewer.color_settings.show_b;
        viewer.review(true, false);
    });

    ui.on_invert_channels(move || {
        println!("on_invert_channels");
        let mut viewer = state_copy.borrow_mut();
        viewer.color_settings.invert = !viewer.color_settings.invert;
        viewer.review(true, false);
    });

    ui.on_color_setting(move || {
        println!("on_color_setting");
        let mut viewer = state_copy.borrow_mut();
        // TODO !!!!    self.color_correction_dialog = !self.color_correction_dialog;
    });

    ui.on_about(move || {
        println!("on_about");
        let mut viewer = state_copy.borrow_mut();
    });
    
    ui.on_exit(move || {
        println!("exit");
        let mut viewer = state_copy.borrow_mut();
        if let Some(ui) = ui_weak.upgrade() {
            let _ = ui.window().hide();
        }
    });
    
    // Példa Timer indítására a Play gombra
    let timer = slint::Timer::default();
    ui.on_play_animation(move || {
        println!("Play/Stop");
        let mut viewer = state_copy.borrow_mut();
        timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(100), || {
            // Következő képkocka betöltése...
        });
    });
    
}

