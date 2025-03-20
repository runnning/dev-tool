fn main() {
    slint_build::compile("src/gui/main.slint").unwrap();
    
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_manifest_file("resources/app.manifest");
        res.compile().unwrap();
    }
}