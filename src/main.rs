slint::include_modules!();

use slint::SharedPixelBuffer;
use slint::Rgba8Pixel;
use arboard::Clipboard;

mod file_callbacks;

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;

    file_callbacks::file_callbacks(ui.as_weak());

    ui.run()
}
