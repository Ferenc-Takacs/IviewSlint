use crate::MainWindow; // A build.rs által generált típus
use crate::Pf32;
use crate::image_processing::*;
use crate::colors::*;
//use crate::gpu_colors::*;

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
        viewer.display_size = (display_info.width as f32, display_info.height as f32).into();
        viewer.window_size = ( ui.get_screen_width(), ui.get_screen_height()).into();
        let pos = (viewer.display_size - viewer.window_size) * 0.5;
        ui.window().set_position(slint::PhysicalPosition::new(pos.x as i32, pos.y as i32));
        
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
        println!("{:?}",viewer.display_size);
    }
    
    
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
    ui.on_copy_image(move || {
        on_copy_image(&mut value.borrow_mut());
    });

    let value = state_copy.clone();
    ui.on_copy_view(move || {
        on_copy_view(&mut value.borrow_mut());
    });

    let value = state_copy.clone();
    ui.on_paste_image( move || {
        on_paste_image(&mut value.borrow_mut());
    });

    let value = state_copy.clone();
    ui.on_save_file(move || {
        on_save_file(&mut value.borrow_mut());
    });

    let value = state_copy.clone();
    ui.on_save_view( move || {
        on_save_view(&mut value.borrow_mut());
    });

    let value = state_copy.clone();
    ui.on_open_recent(move |path| {
        on_open_recent(&mut value.borrow_mut(), PathBuf::from(path.to_string()));
    });
    
    let value = state_copy.clone();
    ui.on_open_here_recent(move |path| {
        on_open_here_recent(&mut value.borrow_mut(), PathBuf::from(path.to_string()));
    });
    
    let value = state_copy.clone();
    ui.on_save_recent(move |path| {
        on_save_recent(&mut value.borrow_mut(), PathBuf::from(path.to_string()));
    });
    
    let value = state_copy.clone();
    ui.on_save_view_recent(move |path| {
        on_save_view_recent(&mut value.borrow_mut(), PathBuf::from(path.to_string()));
    });
    
    let value = state_copy.clone();
    ui.on_open_file(move || {
        on_open_file(&mut value.borrow_mut());
    });
    
    let value = state_copy.clone();
    ui.on_change_image(move || {
        on_change_image(&mut value.borrow_mut());
    });
    
    let value = state_copy.clone();
    ui.on_change_view(move || {
        on_change_view(&mut value.borrow_mut());
    });
    
    let value = state_copy.clone();
    ui.on_reopen_file(move || {
        on_reopen_file(&mut value.borrow_mut());
    });
    
    let value = state_copy.clone();
    ui.on_prev_image(move || {
        on_prev_image(&mut value.borrow_mut());
    });

    let value = state_copy.clone();
    ui.on_next_image(move || {
        on_next_image(&mut value.borrow_mut());
    });
    
    let value = state_copy.clone();
    ui.on_info_clicked(move || {
        on_info_clicked(&mut value.borrow_mut());
    });

    let value = state_copy.clone();
    ui.on_change_background(move |mode| {
        on_change_background(&mut value.borrow_mut(), mode);
    });

    let value = state_copy.clone();
    ui.on_down(move || {
        on_down(&mut value.borrow_mut());
    });

    let value = state_copy.clone();
    ui.on_up(move || {
        on_up(&mut value.borrow_mut());
    });

    let value = state_copy.clone();
    ui.on_left(move || {
        on_left(&mut value.borrow_mut());
    });

    let value = state_copy.clone();
    ui.on_right(move || {
        on_right(&mut value.borrow_mut());
    });

    let value = state_copy.clone();
    ui.on_zoom(move |mag| {
        on_zoom(&mut value.borrow_mut(), mag);
    });

    let value = state_copy.clone();
    ui.on_plus(move || {
        on_plus(&mut value.borrow_mut());
    });

    let value = state_copy.clone();
    ui.on_minus(move || {
        on_minus(&mut value.borrow_mut());
    });

    let value = state_copy.clone();
    ui.on_red_channel(move || {
        on_red_channel(&mut value.borrow_mut());
    });

    let value = state_copy.clone();
    ui.on_green_channel(move || {
        on_green_channel(&mut value.borrow_mut());
    });

    let value = state_copy.clone();
    ui.on_blue_channel(move || {
        on_blue_channel(&mut value.borrow_mut());
    });

    let value = state_copy.clone();
    ui.on_invert_channels(move || {
        on_invert_channels(&mut value.borrow_mut());
    });

    let value = state_copy.clone();
    ui.on_color_setting(move || {
        on_color_setting(&mut value.borrow_mut());
    });

    //let value = state_copy.clone();
    //ui.on_about(move || {
    //    println!("on_about");
    //    //let mut viewer = value.borrow_mut();
    //});
    
    // Példa Timer indítására a Play gombra
    //let timer = slint::Timer::default();
    let value = state_copy.clone();
    ui.on_play_animation(move || {
        on_play_animation(&mut value.borrow_mut());
    });
    
    let value = state_copy.clone();
    ui.on_begin_animation(move || {
        on_begin_animation(&mut value.borrow_mut());
    });
    
    let value = state_copy.clone();
    ui.on_back_animation(move || {
        on_back_animation(&mut value.borrow_mut());
    });
    let value = state_copy.clone();
    ui.on_forward_animation(move || {
        on_forward_animation(&mut value.borrow_mut());
    });
    
    let value = state_copy.clone();
    ui.on_loop_animation(move || {
        on_loop_animation(&mut value.borrow_mut());
    });
    
    let ui_weak_keys = ui.as_weak();
    ui.on_key_pressed_event(move |text, ctrl, shift, alt| {
        if alt {
        }
        else {
            if ctrl {
                if shift {
                    if text == "c" || text == "C" { on_copy_view(&mut state.borrow_mut()); return true; }
                    if text == "x" || text == "X" { on_change_view(&mut state.borrow_mut()); return true; }
                }
                else {
                    if text == "c" { on_copy_image(&mut state.borrow_mut()); return true; }
                    if text == "v" { on_paste_image(&mut state.borrow_mut()); return true; }
                    if text == "x" { on_change_image(&mut state.borrow_mut()); return true;}
                    if text == "r" { on_red_channel(&mut state.borrow_mut()); return true;}
                    if text == "g" { on_green_channel(&mut state.borrow_mut()); return true;}
                    if text == "b" { on_blue_channel(&mut state.borrow_mut()); return true;}
                    if text == "i" { on_invert_channels(&mut state.borrow_mut()); return true;}
                    if text == "1" { on_zoom(&mut state.borrow_mut(),1.0); return true; }
                    if text == "2" { on_zoom(&mut state.borrow_mut(),2.0); return true; }
                    if text == "3" { on_zoom(&mut state.borrow_mut(),3.0); return true; }
                    if text == "4" { on_zoom(&mut state.borrow_mut(),4.0); return true; }
                    if text == "5" { on_zoom(&mut state.borrow_mut(),5.0); return true; }
                    if text == "6" { on_zoom(&mut state.borrow_mut(),6.0); return true; }
                    if text == "7" { on_zoom(&mut state.borrow_mut(),7.0); return true; }
                    if text == "8" { on_zoom(&mut state.borrow_mut(),8.0); return true; }
                    if text == "9" { on_zoom(&mut state.borrow_mut(),9.0); return true; }
                    if text == "0" { on_zoom(&mut state.borrow_mut(),10.0); return true; }
                    let up_ar = slint::SharedString::from(slint::platform::Key::UpArrow);
                    if text == up_ar    { on_up(&mut state.borrow_mut()); return true;}
                    let down_ar = slint::SharedString::from(slint::platform::Key::DownArrow);
                    if text == down_ar  { on_down(&mut state.borrow_mut()); return true;}
                    let left_ar = slint::SharedString::from(slint::platform::Key::LeftArrow);
                    if text == left_ar  { on_left(&mut state.borrow_mut()); return true;}
                    let right_ar = slint::SharedString::from(slint::platform::Key::RightArrow);
                    if text == right_ar { on_right(&mut state.borrow_mut()); return true;}
                }
            }
            else {
                if shift {
                    if text == "s" || text == "S" { on_save_view(&mut state.borrow_mut()); return true; }
                }
                else {
                    if text == "i" { on_info_clicked(&mut state.borrow_mut()); return true; }
                    if text == "c" { on_color_setting(&mut state.borrow_mut()); return true; }
                    if text == "o" { on_open_file(&mut state.borrow_mut()); return true; }
                    if text == "r" { on_reopen_file(&mut state.borrow_mut()); return true; }
                    if text == "b" { on_prev_image(&mut state.borrow_mut()); return true; }
                    if text == "n" { on_next_image(&mut state.borrow_mut()); return true; }
                    if text == "s" { on_save_file(&mut state.borrow_mut()); return true; }
                    if text == "d" { on_change_background(&mut state.borrow_mut(),-1); return true; }
                    if text == "+" { on_plus(&mut state.borrow_mut()); return true; }
                    if text == "-" { on_minus(&mut state.borrow_mut()); return true; }
                    if text == "1" { on_zoom(&mut state.borrow_mut(),1.0); return true; }
                    if text == "2" { on_zoom(&mut state.borrow_mut(),0.5); return true; }
                    if text == "3" { on_zoom(&mut state.borrow_mut(),0.45); return true; }
                    if text == "4" { on_zoom(&mut state.borrow_mut(),0.4); return true; }
                    if text == "4" { on_zoom(&mut state.borrow_mut(),0.35); return true; }
                    if text == "5" { on_zoom(&mut state.borrow_mut(),0.3); return true; }
                    if text == "7" { on_zoom(&mut state.borrow_mut(),0.25); return true; }
                    if text == "8" { on_zoom(&mut state.borrow_mut(),0.2); return true; }
                    if text == "9" { on_zoom(&mut state.borrow_mut(),0.15); return true; }
                    if text == "0" { on_zoom(&mut state.borrow_mut(),0.1); return true; }
                    let esc = slint::SharedString::from(slint::platform::Key::Escape);
                    if text == esc {
                        if let Some(ui) = ui_weak_keys.upgrade() {
                            let _ = ui.window().hide();
                        }
                        return true; }
                }
            }
        }
        return false;
    });
    
    let ui_weak_exit = ui.as_weak();
    ui.on_exit(move || {
        println!("exit");
        if let Some(ui) = ui_weak_exit.upgrade() {
            let _ = ui.window().hide();
        }
    });
    
}


fn on_copy_image(viewer: &mut ImageViewer) {
    println!("on_copy_image");
    viewer.save_original = true;
    viewer.copy_to_clipboard();
}

fn on_copy_view(viewer: &mut ImageViewer) {
    println!("on_copy_view");
    viewer.save_original = false;
    viewer.copy_to_clipboard();
}

fn on_paste_image(viewer: &mut ImageViewer) {
    println!("on_paste_image");
    viewer.copy_from_clipboard();
}

fn on_save_file(viewer: &mut ImageViewer) {
    println!("on_save_file");
    viewer.save_original = true;
    viewer.starting_save(&None);
}

fn on_save_view(viewer: &mut ImageViewer) {
    println!("on_save_view");
    viewer.save_original = false;
    viewer.starting_save(&None);
}

fn on_open_recent(viewer: &mut ImageViewer, path_buf : PathBuf) {
    println!("on_open_recent");
    viewer.open_image(&path_buf,true);
}

fn on_open_here_recent(viewer: &mut ImageViewer, path_buf : PathBuf) {
    println!("on_open_here_recent");
    viewer.open_image_dialog(&Some(path_buf));
}

fn on_save_recent(viewer: &mut ImageViewer, path_buf : PathBuf) {
    println!("on_save_recent");
    viewer.save_original = true;
    viewer.starting_save(&Some(path_buf));
}
    
fn on_save_view_recent(viewer: &mut ImageViewer, path_buf : PathBuf) {
    println!("on_save_view_recent");
    viewer.save_original = false;
    viewer.starting_save(&Some(path_buf));
}

fn on_open_file(viewer: &mut ImageViewer) {
    println!("on_open_file");
    viewer.open_image_dialog(&None);
}

fn on_change_image(viewer: &mut ImageViewer) {
    println!("on_change_image");
    viewer.save_original = true;
    viewer.change_with_clipboard();
}

fn on_change_view(viewer: &mut ImageViewer) {
    println!("on_change_view");
    viewer.save_original = false;
    viewer.change_with_clipboard();
}

fn on_reopen_file(viewer: &mut ImageViewer) {
    println!("on_reopen_file");
    viewer.load_image(true);
}

fn on_prev_image(viewer: &mut ImageViewer) {
    println!("on_prev_image");
    viewer.navigation(-1);
}

fn on_next_image(viewer: &mut ImageViewer) {
    println!("on_next_image");
    viewer.navigation(1);
}

fn on_info_clicked(viewer: &mut ImageViewer) {
    println!("on_info_clicked");
}

fn on_change_background(viewer: &mut ImageViewer, mode: i32) {
    println!("on_change_background");
    let bkgrd = if mode >= 0 { BackgroundStyle::from(mode) } else { viewer.bg_style.clone().inc() };
    viewer.bg_style = bkgrd;
}

fn on_down(viewer: &mut ImageViewer) {
    println!("on_down");
    // rotate to 0
    let r = viewer.color_settings.rotate == Rotate::Rotate90
        || viewer.color_settings.rotate == Rotate::Rotate270;
    viewer.color_settings.rotate = Rotate::Rotate0;
    viewer.review(true, r);
}

fn on_up(viewer: &mut ImageViewer) {
    println!("on_up");
    // rotate 180
    viewer.color_settings.rotate = viewer.color_settings.rotate.add(Rotate::Rotate180);
    viewer.review(true, false);
}

fn on_left(viewer: &mut ImageViewer) {
    println!("on_left");
    // rotate -90
    viewer.color_settings.rotate = viewer.color_settings.rotate.add(Rotate::Rotate270);
    viewer.review(true, true);
}

fn on_right(viewer: &mut ImageViewer) {
    println!("on_right");
    // rotate 90
    viewer.color_settings.rotate = viewer.color_settings.rotate.add(Rotate::Rotate90);
    viewer.review(true, true);
}

fn on_zoom(viewer: &mut ImageViewer, mag : f32) {
    println!("on_zoom");
    if viewer.magnify != mag {
        viewer.want_magnify = mag;
        viewer.review(true, false);
    }
}



fn on_plus(viewer: &mut ImageViewer) {
    println!("on_plus");
    viewer.change_magnify = 1.0;
    viewer.review(true, false);
}

fn on_minus(viewer: &mut ImageViewer) {
    println!("on_minus");
    viewer.change_magnify = -1.0;
    viewer.review(true, false);
}

fn on_red_channel(viewer: &mut ImageViewer) {
    println!("on_red_channel");
    viewer.color_settings.show_r = !viewer.color_settings.show_r;
    viewer.review(true, false);
}

fn on_green_channel(viewer: &mut ImageViewer) {
    println!("on_green_channel");
    viewer.color_settings.show_g = !viewer.color_settings.show_g;
    viewer.review(true, false);
}

fn on_blue_channel(viewer: &mut ImageViewer) {
    println!("on_blue_channel");
    viewer.color_settings.show_b = !viewer.color_settings.show_b;
    viewer.review(true, false);
}

fn on_invert_channels(viewer: &mut ImageViewer) {
    println!("on_invert_channels");
    viewer.color_settings.invert = !viewer.color_settings.invert;
    viewer.review(true, false);
}

fn on_color_setting(viewer: &mut ImageViewer) {
    println!("on_color_setting");
    // TODO !!!!    self.color_correction_dialog = !self.color_correction_dialog;
}

//let value = state_copy.clone();
//ui.on_about(move || {
//    println!("on_about");
//    //let mut viewer = value.borrow_mut();
//});

fn on_play_animation(viewer: &mut ImageViewer) {
    println!("on_play_animation");
// Példa Timer indítására a Play gombra
//let timer = slint::Timer::default();
    //timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(100), || {
        // Következő képkocka betöltése...
}

fn on_begin_animation(viewer: &mut ImageViewer) {
    println!("on_begin_animation");
}

fn on_back_animation(viewer: &mut ImageViewer) {
    println!("on_back_animation");
}

fn on_forward_animation(viewer: &mut ImageViewer) {
    println!("on_forward_animation");
}

fn on_loop_animation(viewer: &mut ImageViewer) {
    println!("on_loop_animation");
}
