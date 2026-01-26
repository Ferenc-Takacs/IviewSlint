slint::include_modules!();

use slint::SharedPixelBuffer;
use slint::Rgba8Pixel;
use arboard::Clipboard;


fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;

    // MÁSOLÁS LOGIKA (Rust-ban)
    let ui_handle = ui.as_weak();
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


    ui.run()
}
