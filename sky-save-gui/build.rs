fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("res/icon.ico");
        res.compile().unwrap();
    }

    built::write_built_file().expect("Failed to acquire build-time information");
}
