slint::include_modules!();

mod file_callbacks;

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;

    file_callbacks::file_callbacks(ui.as_weak());

    ui.run()
}
