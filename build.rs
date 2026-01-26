// build.rs
//use winres::WindowsResource;

fn main() {
    slint_build::compile("src/ui/main.slint").unwrap();
    
    let target = std::env::var("TARGET").unwrap_or_default();
    if target.contains("windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("src/assets/magnifier.ico"); 
        
        // Csak akkor compile-oljunk, ha tényleg Windowsra fordítunk
        if let Err(e) = res.compile() {
            eprintln!("Hiba a Windows erőforrás fordításakor: {}", e);
        }
    }

}
