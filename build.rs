fn main() {
    println!("cargo:rerun-if-changed=layouts");

    let layouts_dir = std::path::Path::new("layouts");
    if layouts_dir.exists() {
        let public_layouts_dir = std::path::Path::new("public").join("layouts");
        let _ = std::fs::create_dir_all(&public_layouts_dir);
        
        let out_dir = std::env::var("OUT_DIR").unwrap_or_default();
        let target_layouts_dir = if !out_dir.is_empty() {
            Some(std::path::Path::new(&out_dir).join("../../../layouts"))
        } else {
            None
        };
        
        if let Some(ref target_dir) = target_layouts_dir {
            let _ = std::fs::create_dir_all(target_dir);
        }

        if let Ok(entries) = std::fs::read_dir(layouts_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Some(filename) = path.file_name() {
                        let dest = public_layouts_dir.join(filename);
                        let _ = std::fs::copy(&path, &dest);
                        
                        if let Some(ref target_dir) = target_layouts_dir {
                            let dest_target = target_dir.join(filename);
                            let _ = std::fs::copy(&path, &dest_target);
                        }
                    }
                }
            }
        }
    }

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


