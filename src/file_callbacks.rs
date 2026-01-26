use crate::MainWindow; // A build.rs által generált típus
use slint::ComponentHandle;

pub fn file_callbacks(ui_handle: slint::Weak<MainWindow>) {
    let ui = ui_handle.clone();
    
    ui.on_request_copy(move || {
        println!("Másolás indul...");
        let mut clipboard = arboard::Clipboard::new().unwrap();
        // Itt jönne a te TIFF/Pixel adatod
        let dummy_pixels = vec![255u8; 100 * 100 * 4]; 
        let img = arboard::ImageData { width: 100, height: 100, bytes: std::borrow::Cow::from(&dummy_pixels) };
        let _ = clipboard.set_image(img);
    });

    ui.on_request_paste(move || {
        let ui = ui_handle.unwrap();
        let mut clipboard = Clipboard::new().expect("Vágólap elérése sikertelen");

        if let Ok(image_data) = clipboard.get_image() {
            // Az arboard RGBA-t ad, a Slint SharedPixelBuffer-t vár
            let mut buffer = SharedPixelBuffer::<Rgba8Pixel>::new(
                image_data.width as u32, 
                image_data.height as u32
            );

            // Pixelek átmásolása a bufferbe
            buffer.make_mut_bytes().copy_from_slice(&image_data.bytes);

            // 2. Slint Image létrehozása és küldése a GUI-nak
            let slint_img = slint::Image::from_rgba8(buffer);
            ui.set_pasted_image(slint_img);
            
            println!("Kép sikeresen beillesztve: {}x{}", image_data.width, image_data.height);
        } else {
            println!("Nincs kép a vágólapon!");
        }
    });

    ui.on_save_tiff(move || {
        // Kép lekérése a Slint property-ből
        let slint_img = ui.get_pasted_image(); 
        if let Some(pixel_buffer) = slint_img.as_rgba8() {
            let width = pixel_buffer.width();
            let height = pixel_buffer.height();
            let pixels = pixel_buffer.make_mut_bytes(); // Nyers RGBA bájtok
            
            // Itt hívd meg a tiff mentő logikádat, amit az Iviewtest-ben írtál
            save_as_tiff("output.tif", width, height, pixels).unwrap();
        }
    });

    // main.rs - Kép betöltésekor
    let img_width = loaded_image.width();
    let img_height = loaded_image.height();

    // Ablak fizikai méretének módosítása (ZOOM figyelembevételével)
    ui.window().set_size(slint::PhysicalSize::new(img_width, img_height + 30)); // +30 a menüsor

    ui.on_open_file(move || {
        let files = rfd::FileDialog::new()
            .add_filter("Image Files", &["png", "jpg", "tiff", "tif"])
            .pick_file();

        if let Some(path) = files {
            // Kép betöltése elérési útról és megjelenítése...
            println!("Kiválasztott fájl: {:?}", path);
        }
    });
}