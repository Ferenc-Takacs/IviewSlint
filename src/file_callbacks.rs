use crate::MainWindow; // A build.rs által generált típus
use crate::ColorCorrectionWindow;
use crate::AboutWindow;
use crate::InfoWindow;
use crate::SaveWindow;
use crate::image_processing::*;
use crate::colors::*;
use crate::Pf32;

use slint::{ComponentHandle,BackendSelector,Image,Color,SharedPixelBuffer,Rgba8Pixel};
use crate::ImageState;
use crate::SlintColorSettings;
use std::rc::Rc;
use std::cell::RefCell;
use crate::ImageViewer;
use std::env;
use std::path::PathBuf;
use display_info::DisplayInfo;

pub fn file_callbacks(
        ui_weak: slint::Weak<MainWindow>,
        settings_ui: ColorCorrectionWindow,
        about_ui: AboutWindow,
        info_ui: InfoWindow,
        save_window_ui: SaveWindow,
        state: Rc<RefCell<ImageViewer>>)
{
    let ui = ui_weak.unwrap();    
    let state_copy = state.clone();

    {

        // startup setting
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
        let bkg = viewer.bg_style.clone().to();
        ui.set_background_type(bkg);
        let pos = (viewer.display_size - viewer.window_size) * 0.5;
        let new_state = ImageState {
            window_width: viewer.window_size.x,     // length -> f32
            window_height: viewer.window_size.y,    // length -> f32
            zoom_level: viewer.magnify,       // float -> f32
            current_image: slint::Image::default(),       // image -> slint::Image
            window_title: "iViewer".into(),            // string -> slint::SharedString
        };
        ui.set_img_state(new_state);
        ui.set_offset_x(0.0);
        ui.set_offset_y(0.0);
        ui.window().set_position(slint::PhysicalPosition::new(pos.x as i32, pos.y as i32));
        
        //println!("{:?}",display_info);
        viewer.load_settings();
        
        if bkg > 3 {
            let tile = generate_checker_tile(bkg);
            ui.set_checker_tile(tile);
        }
        ui.set_red_checked(viewer.color_settings.show_r);
        ui.set_green_checked(viewer.color_settings.show_g);
        ui.set_blue_checked(viewer.color_settings.show_b);
        ui.set_invert_checked(viewer.color_settings.invert);
        
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
        //let rec = &viewer.config.recent_files;
        //println!("{:?}",viewer.display_size);
        
        /*let selector = BackendSelector::new().require_opengl_es_with_version(3, 0);
        if let Err(err) = selector.select() {
            println!("Error selecting backend with OpenGL ES support:\n   {err}");
            viewer.use_gpu = false;
            viewer.gpu_tried_init = true;
        }*/
        viewer.settings_window = Some(settings_ui);
        viewer.about_window = Some(about_ui);
        viewer.info_window = Some(info_ui);
        viewer.save_window = Some(save_window_ui);
    }
    
    {
        let value = state_copy.clone();
        if let Some(settings_ui)  = &state_copy.borrow().settings_window {
            let settings_handle = settings_ui.as_weak();
            settings_ui.on_changed({
                let state_rc = value.clone();
                move || {            
                    if let Some(s_ui) = settings_handle.upgrade() { 
                        let mut viewer = state_rc.borrow_mut();
                        let colset = s_ui.get_colset();
                        viewer.color_settings.show_r = colset.show_r;
                        viewer.color_settings.show_g = colset.show_g;
                        viewer.color_settings.show_b = colset.show_b;
                        viewer.color_settings.invert = colset.invert;
                        viewer.color_settings.rotate = Rotate::from_u8(colset.rotate as u8);
                        viewer.color_settings.oklab  = colset.oklab;
                        viewer.color_settings.gamma = s_ui.get_gamma();
                        viewer.color_settings.contrast = s_ui.get_contrast();
                        viewer.color_settings.brightness = s_ui.get_brightness();
                        viewer.color_settings.hue_shift = s_ui.get_hue_shift();
                        viewer.color_settings.saturation = s_ui.get_saturation();
                        viewer.color_settings.sharpen_amount = s_ui.get_sharpen_amount();
                        viewer.color_settings.sharpen_radius = s_ui.get_sharpen_radius();
                        viewer.review(true, false);
                    }
                }
            });
        }
    }
    {
        let value = state_copy.clone();
        if let Some(settings_ui)  = &state_copy.borrow().settings_window {
            let settings_handle = settings_ui.as_weak();
            settings_ui.on_hide({
                let state_rc = value.clone();
                move || {            
                    if let Some(s_ui) = settings_handle.upgrade() { 
                        let mut viewer = state_rc.borrow_mut();
                        viewer.show_settings = false;
                        s_ui.hide().unwrap();
                    }
                }
            });
        }
    }
    {
        let value = state_copy.clone();
        if let Some(info_ui)  = &state_copy.borrow().info_window {
            let info_handle = info_ui.as_weak();
            info_ui.on_hide({
                let state_rc = value.clone();
                move || {            
                    if let Some(s_ui) = info_handle.upgrade() { 
                        let mut viewer = state_rc.borrow_mut();
                        viewer.show_info = false;
                        s_ui.hide().unwrap();
                    }
                }
            });
        }
    }
    {
        let value = state_copy.clone();
        if let Some(info_ui)  = &state_copy.borrow().info_window {
            let info_handle = info_ui.as_weak();
            info_ui.on_go_map({
                let state_rc = value.clone();
                move || {            
                    if let Some(s_ui) = info_handle.upgrade() { 
                        let mut viewer = state_rc.borrow_mut();
                        let map_url = s_ui.get_map_url().to_string();
                        if let Err(e) = webbrowser::open(&map_url) {
                            eprintln!("Can not open the Browser: {}", e);
                        }
                    }
                }
            });
        }
    }

    
    {
        let value = state_copy.clone();
        let viewer = value.borrow_mut();
        if let Some(about_ui)  = &viewer.about_window {
            let about_handle = about_ui.as_weak();    
            ui.on_show_about(move || {
                if let Some(s_ui) = about_handle.upgrade() {
                    println!("on_show_about");
                    s_ui.show().unwrap(); // Megjeleníti a független ablakot
                }
            });
        }
    }

    /*
    let save_window_handle = save_window_ui.as_weak();
    ui.on_show_save_window(move || {
        if let Some(s_ui) = save_window_handle.upgrade() {
            println!("on_show_save_window");
            s_ui.show().unwrap(); // Megjeleníti a független ablakot
        }
    });
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
        on_info_clicked(&mut value.borrow_mut(), false);
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
    ui.on_plus(move |i| {
        on_plus(&mut value.borrow_mut(),i);
    });

    let value = state_copy.clone();
    ui.on_minus(move |i| {
        on_minus(&mut value.borrow_mut(),i);
    });

    let value = state_copy.clone();
    ui.on_red_channel(move |r| {
        on_red_channel( &mut value.borrow_mut(), r, true);
    });

    let value = state_copy.clone();
    ui.on_green_channel(move |g| {
        on_green_channel(&mut value.borrow_mut(), g, true);
    });

    let value = state_copy.clone();
    ui.on_blue_channel(move |b| {
        on_blue_channel(&mut value.borrow_mut(), b, true);
    });

    let value = state_copy.clone();
    ui.on_invert_channels(move |i| {
        on_invert_channels(&mut value.borrow_mut(), i, true);
    });

    let value = state_copy.clone();
    ui.on_color_settings(move || {
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
    
    let value = state_copy.clone();
    ui.on_mouse_move(move |posx,posy| {
        let mut viewer = value.borrow_mut();
        viewer.mouse_pos = Pf32{ x: posx, y: posy };
    });
    
    let value = state_copy.clone();
    ui.on_mouse_pos(move |posx,posy| {
        let mut viewer = value.borrow_mut();
        viewer.mouse_pos = Pf32{ x: posx, y: posy };
    });
    
    //let value = state_copy.clone();
    ui.on_mouse_off(move || {
        //let mut viewer = value.borrow_mut();
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
                    if text == "r" { on_red_channel(&mut state.borrow_mut(),false,false); return true;}
                    if text == "g" { on_green_channel(&mut state.borrow_mut(),false,false); return true;}
                    if text == "b" { on_blue_channel(&mut state.borrow_mut(),false,false); return true;}
                    if text == "i" { on_invert_channels(&mut state.borrow_mut(),false,false); return true;}
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
                    if text == "i" { on_info_clicked(&mut state.borrow_mut(), false); return true; }
                    if text == "c" { on_color_setting(&mut state.borrow_mut()); return true; }
                    if text == "o" { on_open_file(&mut state.borrow_mut()); return true; }
                    if text == "r" { on_reopen_file(&mut state.borrow_mut()); return true; }
                    if text == "b" { on_prev_image(&mut state.borrow_mut()); return true; }
                    if text == "n" { on_next_image(&mut state.borrow_mut()); return true; }
                    if text == "s" { on_save_file(&mut state.borrow_mut()); return true; }
                    if text == "d" { on_change_background(&mut state.borrow_mut(),-1); return true; }
                    if text == "+" { on_plus(&mut state.borrow_mut(),0.0); return true; }
                    if text == "-" { on_minus(&mut state.borrow_mut(),0.0); return true; }
                    if text == "f" { on_zoom(&mut state.borrow_mut(),-1.0); return true; }
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

pub fn on_info_clicked(viewer: &mut ImageViewer, refresh_only: bool) {
    println!("on_info_clicked");
    if let Some(info_ui)  = &viewer.info_window {
        let info_handle = info_ui.as_weak();    
        if let Some(s_ui) = info_handle.upgrade() {
            s_ui.set_filename( viewer.image_name.clone().into());
            s_ui.set_imagesize( format!( "{} x {} pixel", viewer.image_size.x, viewer.image_size.y ).into());
            let mut s = "".to_string();
            if let Some(meta) = &viewer.file_meta {
                s = format!("{}", meta.len());
                let l = s.len();
                if l > 3 {
                    s = format!(
                        "{} {}",
                        s[..l - 3].to_string(),
                        s[l - 3..].to_string()
                    );
                }
                if l > 6 {
                    s = format!(
                        "{} {}",
                        s[..l - 6].to_string(),
                        s[l - 6..].to_string()
                    );
                }
                if l > 9 {
                    s = format!(
                        "{} {}",
                        s[..l - 9].to_string(),
                        s[l - 9..].to_string()
                    );
                }
                s = format!("{} Byte", s);
            }
            s_ui.set_filesize( s.into());
            s = "".to_string();
            if let Some(meta) = &viewer.file_meta {
                if let Ok(time) = meta.created() {
                    let ts = time_format::from_system_time(time).unwrap();
                    let c = time_format::components_utc(ts).unwrap();
                    s = format!(
                        "{}-{:02}-{:02} {:02}:{:02}:{:02}",
                        c.year, c.month, c.month_day, c.hour, c.min, c.sec
                    );
                }
            }
            s_ui.set_filetime( s.into());
            s = "".to_string();
            if let Some(resol) = &viewer.resolution {
                let x_res = resol.xres;
                let y_res = resol.yres;
                let dpi = resol.dpi;
                let x_val = x_res.to_string();
                let y_val = y_res.to_string();
                let unit_str = if dpi { "dpi" } else { "dpcm" };
                if x_val == y_val {
                    s = format!("{} {}", x_val, unit_str);
                } else {
                    s = format!("{}x{} {}", x_val, y_val, unit_str);
                }
            }
            s_ui.set_resolution( s.into());
            if let Some(exif) = &viewer.exif {
                s_ui.set_exif( true);
                if let Some(f) = exif.get_field("DateTimeOriginal".into()) {
                    s_ui.set_created(f.into());
                }
                if let Some(f) = exif.get_field("Model".into()) {
                    s_ui.set_model(f.into());
                }
                let la = exif .get_num_field("GPSLatitude".into());
                let lo = exif.get_num_field("GPSLongitude".into());
                let lar = exif.get_field("GPSLatitudeRef".into());
                let lor = exif.get_field("GPSLongitudeRef".into());
                if let (Some(mut la_), Some(mut lo_), Some(lar_), Some(lor_), ) = (la, lo, lar, lor) {
                    s_ui.set_gps( true);
                    if lar_.contains('S') {
                        la_ = -la_;
                    }
                    if lor_.contains('W') {
                        lo_ = -lo_;
                    }
                    let koord_szoveg = format!("{:.6}, {:.6}", la_, lo_);
                    s_ui.set_location(koord_szoveg.into());
                    let map_url = format!(
                        "https://www.google.com/maps/place/{:.6},{:.6}",
                        la_, lo_
                    );
                    s_ui.set_map_url(map_url.into());
                }
                else {
                    s_ui.set_gps( false);
                }
            }
            else {
                s_ui.set_exif( false);
            }
            if !refresh_only {
                if !viewer.show_info {
                    viewer.show_info = true;
                    s_ui.show().unwrap(); // Megjeleníti a független ablakot
                    
                    /*if let Some(handle) = &viewer.ui_handle {                    
                        if let Some(ui) = handle.upgrade() {
                            slint::Timer::single_shot(std::time::Duration::from_millis(250), move || {
                                ui.window().show().unwrap();
                                ui.invoke_grab_keyboard_focus();
                            });
                        }
                    }*/
                }
                else {
                    viewer.show_info = false;
                    s_ui.hide().unwrap(); // Megjeleníti a független ablakot
                }
            }
        }
    }
}

fn on_change_background(viewer: &mut ImageViewer, mode: i32) {
    println!("on_change_background");
    let bkgrd = if mode >= 0 { BackgroundStyle::from(mode) } else { viewer.bg_style.clone().inc() };
    viewer.bg_style = bkgrd.clone();
    let bkg = bkgrd.to();
    
    if let Some(handle) = &viewer.ui_handle {
        if let Some(ui) = handle.upgrade() {
            if bkg > 3 {
                let bkg_t = generate_checker_tile(bkg);
                ui.set_checker_tile(bkg_t);
            }
            ui.set_background_type(bkg);
        }
    }
}


fn generate_checker_tile(style: i32) -> Image {
    // Stílus alapján kiválasztjuk a két színt (RGBA)
    let (c1, c2) = match style {
        4 => (Color::from_rgb_u8(35,35,35), Color::from_rgb_u8(70,70,70)),
        5 => (Color::from_rgb_u8(40, 180, 40),Color::from_rgb_u8(180, 50, 180)),
        6 => (Color::from_rgb_u8(0, 0, 0), Color::from_rgb_u8(200, 50, 10)),
        _ => (Color::from_rgb_u8(0,0,0), Color::from_rgb_u8(255,255,255)),
        //(Color::from_rgb_u8(238, 238, 238), Color::from_rgb_u8(204, 204, 204)), // Szürke
        //(Color::from_rgb_u8(68, 34, 34), Color::from_rgb_u8(17, 0, 0)),        // Pirosas
        //(Color::from_rgb_u8(17, 34, 68), Color::from_rgb_u8(0, 5, 17)),        // Kékes
        //(Color::from_rgb_u8(0, 0, 0), Color::from_rgb_u8(255, 255, 255)),     // Alapértelmezett B/W
    };

    let size: u32 = 16; // Egy kocka mérete pixelben
    let full_size = size * 2;
    let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(full_size, full_size);
    let mut pixels = pixel_buffer.make_mut_slice();

    for y in 0..full_size {
        for x in 0..full_size {
            let is_color1 = (x < size && y < size) || (x >= size && y >= size);
            let color = if is_color1 { c1.to_argb_u8() } else { c2.to_argb_u8() };            
            pixels[(y * full_size + x) as usize] = Rgba8Pixel {
                r: color.red, g: color.green, b: color.blue, a: color.alpha,
            };
        }
    }
    Image::from_rgba8(pixel_buffer)
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



fn on_plus(viewer: &mut ImageViewer, i: f32) {
    //println!("on_plus");
    viewer.mouse_zoom = i != 0.0;
    viewer.change_magnify = 1.0;
    viewer.review(true, false);
}

fn on_minus(viewer: &mut ImageViewer, i : f32) {
    //println!("on_minus");
    viewer.mouse_zoom = i != 0.0;
    viewer.change_magnify = -1.0;
    viewer.review(true, false);
}

fn on_red_channel(viewer: &mut ImageViewer, val : bool, no_set: bool) {
    println!("on_red_channel");
    //if no_set {
    //    viewer.color_settings.show_r = !val;
    //}
    //else  {
        viewer.color_settings.show_r = !viewer.color_settings.show_r;
        if let Some(handle) = &viewer.ui_handle {
            if let Some(ui) = handle.upgrade() {
                ui.set_red_checked(viewer.color_settings.show_r);
            }
        }
    //}
    viewer.review(true, false);
}

fn on_green_channel(viewer: &mut ImageViewer, val : bool, no_set: bool) {
    println!("on_green_channel");
    //if no_set {
    //    viewer.color_settings.show_g = !val;
    //}
    //else  {
        viewer.color_settings.show_g = !viewer.color_settings.show_g;
        if let Some(handle) = &viewer.ui_handle {
            if let Some(ui) = handle.upgrade() {
                ui.set_green_checked(viewer.color_settings.show_g);
            }
        }
    //}
    viewer.review(true, false);
}

fn on_blue_channel(viewer: &mut ImageViewer, val : bool, no_set: bool) {
    println!("on_blue_channel");
    //if no_set {
    //    viewer.color_settings.show_b = !val;
    //}
    //else  {
        viewer.color_settings.show_b = !viewer.color_settings.show_b;
        if let Some(handle) = &viewer.ui_handle {
            if let Some(ui) = handle.upgrade() {
                ui.set_blue_checked(viewer.color_settings.show_b);
            }
        }
    //}
    viewer.review(true, false);
}

fn on_invert_channels(viewer: &mut ImageViewer, val : bool, no_set: bool) {
    println!("on_invert_channels");
    //if no_set {
    //    viewer.color_settings.invert = !val;
    //}
    //else  {
        viewer.color_settings.invert = !viewer.color_settings.invert;
        if let Some(handle) = &viewer.ui_handle {
            if let Some(ui) = handle.upgrade() {
                ui.set_invert_checked(viewer.color_settings.invert);
            }
        }
    //}
    viewer.review(true, false);
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

fn on_color_setting(viewer: &mut ImageViewer) {
    println!("on_color_setting");
    if let Some(settings_ui)  = &viewer.settings_window {
        let settings_handle = settings_ui.as_weak();    
        if let Some(s_ui) = settings_handle.upgrade() {
            if !viewer.show_settings {
                viewer.show_settings = true;
                let colset = SlintColorSettings{
                    show_r: viewer.color_settings.show_r,
                    show_g: viewer.color_settings.show_g,
                    show_b: viewer.color_settings.show_b,
                    invert: viewer.color_settings.invert,
                    rotate: viewer.color_settings.rotate.to_u8() as i32, // realy image setting
                    oklab: viewer.color_settings.oklab,
                };
                println!("on_show_color_settings");
                s_ui.set_gamma( viewer.color_settings.gamma);
                s_ui.set_contrast( viewer.color_settings.contrast);
                s_ui.set_brightness( viewer.color_settings.brightness);
                s_ui.set_hue_shift( viewer.color_settings.hue_shift);
                s_ui.set_saturation( viewer.color_settings.saturation);
                s_ui.set_sharpen_amount( viewer.color_settings.sharpen_amount);
                s_ui.set_sharpen_radius( viewer.color_settings.sharpen_radius);
                s_ui.set_colset(colset);
                s_ui.show().unwrap(); // Megjeleníti a független ablakot
            }
            else {
                viewer.show_settings = false;
                s_ui.hide().unwrap(); // Megjeleníti a független ablakot
            }
        }
    }
}
