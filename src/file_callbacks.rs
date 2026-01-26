use crate::MainWindow; // A build.rs által generált típus
use slint::ComponentHandle;
//use slint::SharedPixelBuffer;
use slint::*;
use arboard::*;
use rfd::FileDialog;


pub fn file_callbacks(ui_weak: slint::Weak<MainWindow>) {
    let ui = ui_weak.unwrap();
    
    ui.on_copy_image(move || {
        println!("Copy");
        let mut clipboard = Clipboard::new().unwrap();
        // Itt jönne a te TIFF/Pixel adatod
        let dummy_pixels = vec![255u8; 100 * 100 * 4]; 
        let img = ImageData { width: 100, height: 100, bytes: std::borrow::Cow::from(&dummy_pixels) };
        let _ = clipboard.set_image(img);
    });

    ui.on_copy_view(move || {
        println!("Copy View");
        let mut clipboard = Clipboard::new().unwrap();
        // Itt jönne a te TIFF/Pixel adatod
        let dummy_pixels = vec![255u8; 100 * 100 * 4]; 
        let img = ImageData { width: 100, height: 100, bytes: std::borrow::Cow::from(&dummy_pixels) };
        let _ = clipboard.set_image(img);
    });

    ui.on_paste_image({
        let ui_weak = ui_weak.clone();
        move || {
            println!("Paste");
            if let Some(ui) = ui_weak.upgrade() {
                let mut clipboard = Clipboard::new().expect("Vágólap elérése sikertelen");
                if let Ok(image) = clipboard.get_image() {
                    // Az arboard RGBA-t ad, a Slint SharedPixelBuffer-t vár
                    let mut buffer = slint::SharedPixelBuffer::<Rgba8Pixel>::new(
                        image.width as u32, image.height as u32
                    );
                    buffer.make_mut_bytes().copy_from_slice(&image.bytes);
                    let slint_img = slint::Image::from_rgba8(buffer);
                    ui.set_current_image(slint_img);
                    // Ablak átméretezése a képhez
                    ui.window().set_size(slint::PhysicalSize::new(
                        image.width as u32, 
                        (image.height as u32) + 30
                    ));
    
                    // Pixelek átmásolása a bufferbe
                    /*buffer.make_mut_bytes().copy_from_slice(&image_data.bytes);

                    // 2. Slint Image létrehozása és küldése a GUI-nak
                    let slint_img = slint::Image::from_rgba8(buffer);
                    ui.set_pasted_image(slint_img);
                    
                    println!("Kép sikeresen beillesztve: {}x{}", image_data.width, image_data.height);*/
                } else {
                    println!("Nincs kép a vágólapon!");
                }
            }
        }
    });

    ui.on_save_file({
        let ui_weak = ui_weak.clone();
        move || {
            println!("Save");
            if let Some(ui) = ui_weak.upgrade() {
                // Kép lekérése a Slint property-ből
                let slint_img = ui.get_pasted_image(); 
                if let Some(mut pixel_buffer) = slint_img.to_rgba8() {
                    let width = pixel_buffer.width();
                    let height = pixel_buffer.height();
                    let pixels = pixel_buffer.make_mut_bytes(); // Nyers RGBA bájtok
                    
                    // Itt hívd meg a tiff mentő logikádat, amit az Iviewtest-ben írtál
                    //save_as_tiff("output.tif", width, height, pixels).unwrap();
                }
            }
        }
    });

    ui.on_save_view({
        let ui_weak = ui_weak.clone();
        move || {
            println!("Save View");
            if let Some(ui) = ui_weak.upgrade() {
                // Kép lekérése a Slint property-ből
                let slint_img = ui.get_pasted_image(); 
                if let Some(mut pixel_buffer) = slint_img.to_rgba8() {
                    let width = pixel_buffer.width();
                    let height = pixel_buffer.height();
                    let pixels = pixel_buffer.make_mut_bytes(); // Nyers RGBA bájtok
                    
                    // Itt hívd meg a tiff mentő logikádat, amit az Iviewtest-ben írtál
                    //save_as_tiff("output.tif", width, height, pixels).unwrap();
                }
            }
        }
    });

    let slint_img = ui.get_current_image(); // Ez kéri le a képet a GUI-ból

    if let Some(pixel_buffer) = slint_img.to_rgba8() {
        let img_width = pixel_buffer.width();
        let img_height = pixel_buffer.height();

        // Ablak méretezése
        ui.window().set_size(slint::PhysicalSize::new(
            img_width, 
            img_height + 30 // +30 pixel a menüsornak
        ));
    }

    ui.on_open_file(move || {
        println!("Open");
        let files = rfd::FileDialog::new()
            .add_filter("Image Files", &["png", "jpg", "tiff", "tif"])
            .pick_file();

        if let Some(path) = files {
            // Kép betöltése elérési útról és megjelenítése...
            println!("Kiválasztott fájl: {:?}", path);
        }
    });
    
    ui.on_change_image(move || {
        println!("Change");
    });
    
    ui.on_change_view(move || {
        println!("Change View");
    });
    ui.on_reopen_file(move || {
        println!("Reopen");
    });
    
    ui.on_recent_paths(move || {
        println!("Recent paths");
    });
    
    ui.on_prev_image(move || {
        println!("Előző kép (Back)");
    });

    ui.on_next_image(move || {
        println!("Következő kép (Next)");
    });
    
    ui.on_about(move || {
        println!("About");
    });
    
    ui.on_exit(move || {
        println!("exit");
        if let Some(ui) = ui_weak.upgrade() {
            let _ = ui.window().hide();
        }
    });
    
    // Példa Timer indítására a Play gombra
    let timer = slint::Timer::default();
    ui.on_play_animation(move || {
        println!("Play/Stop");
        timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(100), || {
            // Következő képkocka betöltése...
        });
    });
    
    ui.on_info_clicked(move || {
        println!("Info");
    });
}

