use std::collections::HashMap;
use crate::layout::parser::ViaDefinition;

const STORAGE_KEY: &str = "saved_layouts";

#[cfg(not(target_arch = "wasm32"))]
const STORAGE_PATH: &str = "saved_layouts.json";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutSource {
    Predefined(String),
    UserLoaded(String),
    Cached(String),
    None,
}

pub fn parse_hex_or_dec(s: &str) -> u16 {
    let clean = s.trim().trim_start_matches("0x").trim_start_matches("0X");
    u16::from_str_radix(clean, 16).unwrap_or_else(|_| s.trim().parse::<u16>().unwrap_or(0))
}

pub fn load_saved_layouts() -> HashMap<String, ViaDefinition> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Ok(content) = std::fs::read_to_string(STORAGE_PATH) {
            if let Ok(map) = serde_json::from_str::<HashMap<String, ViaDefinition>>(&content) {
                return map;
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(content)) = storage.get_item(STORAGE_KEY) {
                    if let Ok(map) = serde_json::from_str::<HashMap<String, ViaDefinition>>(&content) {
                        return map;
                    }
                }
            }
        }
    }

    HashMap::new()
}

pub fn load_predefined_layouts() -> HashMap<String, ViaDefinition> {
    let mut map = HashMap::new();

    let embedded: &[(&str, &str)] = &[
        ("046D:C31C", include_str!("../../layouts/046D_C31C.json")),
        ("05AC:024F", include_str!("../../layouts/05AC_024F.json")),
        ("DEEF:0001", include_str!("../../layouts/DEEF_0001.json")),
    ];

    for (key, json_str) in embedded {
        if let Ok(def) = serde_json::from_str::<ViaDefinition>(json_str) {
            map.insert(key.to_uppercase(), def);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Ok(entries) = std::fs::read_dir("layouts") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        if let Ok(def) = serde_json::from_str::<ViaDefinition>(&content) {
                            let vid = parse_hex_or_dec(&def.vendor_id);
                            let pid = parse_hex_or_dec(&def.product_id);
                            let key = format!("{:04X}:{:04X}", vid, pid);
                            map.insert(key, def);
                        } else if let Ok(dict) = serde_json::from_str::<HashMap<String, ViaDefinition>>(&content) {
                            for (k, def) in dict {
                                map.insert(k.to_uppercase(), def);
                            }
                        }
                    }
                }
            }
        }
    }

    map
}

pub fn get_layout_with_source(vid: u16, pid: u16) -> Option<(ViaDefinition, LayoutSource)> {
    let key = format!("{:04X}:{:04X}", vid, pid);
    
    let user_map = load_saved_layouts();
    if let Some(def) = user_map.get(&key) {
        return Some((def.clone(), LayoutSource::Cached(def.name.clone())));
    }

    let predefined_map = load_predefined_layouts();
    if let Some(def) = predefined_map.get(&key) {
        return Some((def.clone(), LayoutSource::Predefined(def.name.clone())));
    }

    None
}

pub fn get_layout(vid: u16, pid: u16) -> Option<ViaDefinition> {
    get_layout_with_source(vid, pid).map(|(def, _)| def)
}

pub fn save_layout(vid: u16, pid: u16, def: &ViaDefinition) {
    let mut map = load_saved_layouts();
    let key = format!("{:04X}:{:04X}", vid, pid);
    map.insert(key, def.clone());
    
    if let Ok(json) = serde_json::to_string_pretty(&map) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = std::fs::write(STORAGE_PATH, json);
        }

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item(STORAGE_KEY, &json);
                }
            }
        }
    }
}

pub async fn pick_and_read_json_file() -> Result<Option<(String, String)>, String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(path) = rfd::FileDialog::new().add_filter("JSON", &["json"]).pick_file() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                return Ok(Some((name, content)));
            }
        }
        Ok(None)
    }

    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen_futures::JsFuture;

        let eval_code = r#"
            new Promise((resolve) => {
                const input = document.createElement('input');
                input.type = 'file';
                input.accept = '.json';
                input.onchange = () => {
                    const file = input.files[0];
                    if (!file) return resolve(null);
                    const reader = new FileReader();
                    reader.onload = () => resolve({ name: file.name, content: reader.result });
                    reader.onerror = () => resolve(null);
                    reader.readAsText(file);
                };
                input.click();
            })
        "#;

        let promise_js = js_sys::eval(eval_code).map_err(|e| format!("JS error: {:?}", e))?;
        let promise = js_sys::Promise::from(promise_js);
        let res = JsFuture::from(promise).await.map_err(|e| format!("File pick error: {:?}", e))?;

        if res.is_null() || res.is_undefined() {
            return Ok(None);
        }

        let name = js_sys::Reflect::get(&res, &"name".into())
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_else(|| "layout.json".to_string());

        let content = js_sys::Reflect::get(&res, &"content".into())
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_default();

        Ok(Some((name, content)))
    }
}
