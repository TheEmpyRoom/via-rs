fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();

    if target_os == "windows" && (target_env == "msvc" || target_env == "gnu") {
        #[cfg(target_os = "windows")]
        {
            let mut res = winres::WindowsResource::new();
            res.set_icon("icon.ico");
            let _ = res.compile();
        }
    }
}


