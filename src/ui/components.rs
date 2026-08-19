use dioxus::prelude::*;
use std::collections::HashMap;
use crate::hid::manager::KeyboardInfo;
use crate::layout::parser::ViaDefinition;
use crate::layout::kle::PhysicalKey;

const STYLE: &str = include_str!("style.css");
const LOGO_BYTES: &[u8] = include_bytes!("assets/logo.png");

fn base64_encode(bytes: &[u8]) -> String {
    const CHARSET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut res = String::with_capacity(bytes.len() * 4 / 3 + 4);
    for chunk in bytes.chunks(3) {
        match chunk.len() {
            3 => {
                res.push(CHARSET[(chunk[0] >> 2) as usize] as char);
                res.push(CHARSET[(((chunk[0] & 3) << 4) | (chunk[1] >> 4)) as usize] as char);
                res.push(CHARSET[(((chunk[1] & 15) << 2) | (chunk[2] >> 6)) as usize] as char);
                res.push(CHARSET[(chunk[2] & 63) as usize] as char);
            }
            2 => {
                res.push(CHARSET[(chunk[0] >> 2) as usize] as char);
                res.push(CHARSET[(((chunk[0] & 3) << 4) | (chunk[1] >> 4)) as usize] as char);
                res.push(CHARSET[((chunk[1] & 15) << 2) as usize] as char);
                res.push('=');
            }
            1 => {
                res.push(CHARSET[(chunk[0] >> 2) as usize] as char);
                res.push(CHARSET[((chunk[0] & 3) << 4) as usize] as char);
                res.push('=');
                res.push('=');
            }
            _ => unreachable!(),
        }
    }
    res
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct FeatureBackupValue {
    label: String,
    channel: u8,
    offset: u8,
    value: u16,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct KeycodeBackupValue {
    row: u8,
    col: u8,
    value: u16,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct LayoutBackup {
    version: u32,
    vendor_id: u16,
    product_id: u16,
    layers: Vec<Vec<KeycodeBackupValue>>,
    macros: Vec<String>,
    features: Vec<FeatureBackupValue>,
}

fn find_controls(node: &serde_json::Value, parent_path: &str, controls: &mut Vec<(String, serde_json::Value)>) {
    let label = node.get("label").and_then(|l| l.as_str()).unwrap_or("").to_string();
    let ctype = node.get("type").and_then(|t| t.as_str()).unwrap_or("").to_string();
    
    let current_path = if parent_path.is_empty() {
        label.clone()
    } else if !label.is_empty() {
        format!("{}/{}", parent_path, label)
    } else {
        parent_path.to_string()
    };

    if !ctype.is_empty() {
        controls.push((current_path, node.clone()));
    } else if let Some(content) = node.get("content").and_then(|c| c.as_array()) {
        for child in content {
            find_controls(child, &current_path, controls);
        }
    }
}

pub async fn fetch_keyboard_data(
    dev_path: String,
    mut protocol_version: Signal<u16>,
    mut layer_count: Signal<u8>,
    mut macro_count: Signal<u8>,
    mut macro_buffer_size: Signal<u16>,
    mut macro_buffer: Signal<Vec<u8>>,
    mut decoded_macros: Signal<Vec<String>>,
) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Ok(api) = hidapi::HidApi::new() {
            if let Ok(c_path) = std::ffi::CString::new(dev_path) {
                if let Ok(dev) = api.open_path(&c_path) {
                    let via = crate::hid::via_protocol::ViaKeyboard::new(&dev);
                    
                    let prot = via.get_protocol_version().unwrap_or_else(|_| {
                        let cur = *protocol_version.read();
                        if cur == 0 { 11 } else { cur }
                    });
                    protocol_version.set(prot);
                    
                    if let Ok(count) = via.get_layer_count() {
                        if count > 0 && count <= 16 {
                            layer_count.set(count);
                        }
                    }
                    
                    let m_count = via.get_macro_count().unwrap_or_else(|_| {
                        let cur = *macro_count.read();
                        if cur == 0 { 16 } else { cur }
                    });
                    macro_count.set(m_count);
                    
                    if let Ok(m_size) = via.get_macro_buffer_size() {
                        macro_buffer_size.set(m_size);
                        if m_size > 0 {
                            let mut buffer = Vec::new();
                            for offset in (0..m_size).step_by(28) {
                                let chunk_size = std::cmp::min(28, m_size - offset) as u8;
                                if let Ok(chunk) = via.get_macro_buffer(offset, chunk_size) {
                                    buffer.extend(chunk);
                                } else {
                                    buffer.resize(buffer.len() + chunk_size as usize, 0);
                                }
                            }
                            if buffer.len() < m_size as usize {
                                buffer.resize(m_size as usize, 0);
                            } else {
                                buffer.truncate(m_size as usize);
                            }
                            
                            let split = crate::layout::macro_parser::split_macro_buffer(&buffer, m_count, m_size);
                            let decoded: Vec<String> = split.iter().map(|b| crate::layout::macro_parser::decode_macro(b, prot)).collect();
                            decoded_macros.set(decoded);
                            macro_buffer.set(buffer);
                        }
                    }
                }
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        let via = crate::hid::via_protocol::ViaKeyboard;
        let prot = via.get_protocol_version_async().await.unwrap_or_else(|_| {
            let cur = *protocol_version.read();
            if cur == 0 { 11 } else { cur }
        });
        protocol_version.set(prot);
        
        if let Ok(count) = via.get_layer_count_async().await {
            if count > 0 && count <= 16 {
                layer_count.set(count);
            }
        }
        
        let m_count = via.get_macro_count_async().await.unwrap_or_else(|_| {
            let cur = *macro_count.read();
            if cur == 0 { 16 } else { cur }
        });
        macro_count.set(m_count);
        
        if let Ok(m_size) = via.get_macro_buffer_size_async().await {
            macro_buffer_size.set(m_size);
            if m_size > 0 {
                let mut buffer = Vec::new();
                for offset in (0..m_size).step_by(28) {
                    let chunk_size = std::cmp::min(28, m_size - offset) as u8;
                    if let Ok(chunk) = via.get_macro_buffer_async(offset, chunk_size).await {
                        buffer.extend(chunk);
                    } else {
                        buffer.resize(buffer.len() + chunk_size as usize, 0);
                    }
                }
                if buffer.len() < m_size as usize {
                    buffer.resize(m_size as usize, 0);
                } else {
                    buffer.truncate(m_size as usize);
                }
                
                let split = crate::layout::macro_parser::split_macro_buffer(&buffer, m_count, m_size);
                let decoded: Vec<String> = split.iter().map(|b| crate::layout::macro_parser::decode_macro(b, prot)).collect();
                decoded_macros.set(decoded);
                macro_buffer.set(buffer);
            }
        }
    }
}

#[component]
fn KeycodeButton(
    label: String,
    title: String,
    code: u16,
    mut selected_key: Signal<Option<PhysicalKey>>,
    physical_keys: Signal<Vec<PhysicalKey>>,
    selected_device: Signal<Option<KeyboardInfo>>,
    mut status: Signal<String>,
    mut layer_keycodes: Signal<HashMap<(u8, u8), u16>>,
    active_layer: Signal<u8>,
    mut dragged_keycode: Signal<Option<(String, String, u16)>>,
    #[props(optional)] show_any_modal: Option<Signal<bool>>,
    mut stored_keycodes: Signal<HashMap<(u8, u8, u8), u16>>,
    #[props(optional)] mut any_custom_text: Option<Signal<String>>,
    #[props(optional)] via_def: Option<Signal<Option<ViaDefinition>>>,
) -> Element {
    let label_clone1 = label.clone();
    let title_clone1 = title.clone();
    rsx! {
        button {
            class: "palette-btn",
            title: "{title}",
            onmousedown: move |_| {
                if code != 0x7FFF {
                    dragged_keycode.set(Some((label_clone1.clone(), title_clone1.clone(), code)));
                }
            },
            onclick: move |_| {
                if code == 0x7FFF {
                    if let Some(mut modal_sig) = show_any_modal {
                        if selected_key.read().is_none() {
                            if let Some(first_k) = physical_keys.read().first().cloned() {
                                selected_key.set(Some(first_k));
                            }
                        }
                        if let Some(mut text_sig) = any_custom_text {
                            if let Some(k) = selected_key.read().as_ref() {
                                let r = k.matrix_row as u8;
                                let c = k.matrix_col as u8;
                                if let Some(&curr_code) = layer_keycodes.read().get(&(r, c)) {
                                    let v_def = via_def.and_then(|vd| vd.read().clone());
                                    let default_str = crate::ui::keycodes::format_keycode_for_any_input(curr_code, v_def.as_ref());
                                    text_sig.set(default_str);
                                } else {
                                    text_sig.set("".to_string());
                                }
                            } else {
                                text_sig.set("".to_string());
                            }
                        }
                        modal_sig.set(true);
                    }
                    return;
                }
                if let Some(k) = selected_key.read().as_ref().cloned() {
                    dragged_keycode.set(None);
                    let layer = *active_layer.read();
                    let row = k.matrix_row as u8;
                    let col = k.matrix_col as u8;
                    stored_keycodes.write().insert((layer, row, col), code);
                    layer_keycodes.write().insert((row, col), code);

                    if let Some(dev_info) = selected_device.read().as_ref() {
                        let path = dev_info.path.clone();
                        let c = code;
                        let l = label.clone();
                        
                        let mut selected_key_sig = selected_key.clone();
                        let physical_keys_sig = physical_keys.clone();
                        
                        spawn(async move {
                            #[cfg(not(target_arch = "wasm32"))]
                            if let Ok(api) = hidapi::HidApi::new() {
                                if let Ok(c_path) = std::ffi::CString::new(path) {
                                    if let Ok(device) = api.open_path(&c_path) {
                                        let via = crate::hid::via_protocol::ViaKeyboard::new(&device);
                                        match via.set_keycode(layer, row, col, c) {
                                            Ok(_) => {
                                                let _ = via.custom_save(0, 0);
                                                status.set(format!("Assigned {} to {},{}", l, row, col));
                                                
                                                let p_keys = physical_keys_sig.read().clone();
                                                if let Some(pos) = p_keys.iter().position(|pk| pk.matrix_row == k.matrix_row && pk.matrix_col == k.matrix_col) {
                                                    let next_pos = pos + 1;
                                                    if next_pos < p_keys.len() {
                                                        selected_key_sig.set(Some(p_keys[next_pos].clone()));
                                                    } else {
                                                        selected_key_sig.set(Some(p_keys[0].clone()));
                                                    }
                                                }
                                            },
                                            Err(e) => status.set(format!("Error writing: {}", e)),
                                        }
                                    }
                                }
                            }

                            #[cfg(target_arch = "wasm32")]
                            {
                                let via = crate::hid::via_protocol::ViaKeyboard;
                                match via.set_keycode_async(layer, row, col, c).await {
                                    Ok(_) => {
                                        gloo_timers::future::TimeoutFuture::new(50).await;
                                        let _ = via.custom_save_async(0, 0).await;
                                        status.set(format!("Saved {} to {},{} in EEPROM", l, row, col));

                                        let p_keys = physical_keys_sig.read().clone();
                                        if let Some(pos) = p_keys.iter().position(|pk| pk.matrix_row == k.matrix_row && pk.matrix_col == k.matrix_col) {
                                            let next_pos = pos + 1;
                                            if next_pos < p_keys.len() {
                                                selected_key_sig.set(Some(p_keys[next_pos].clone()));
                                            } else {
                                                selected_key_sig.set(Some(p_keys[0].clone()));
                                            }
                                        }
                                    },
                                    Err(e) => status.set(format!("Error writing: {}", e)),
                                }
                            }
                        });
                    } else {
                        status.set(format!("Set {} to {},{}", label, row, col));
                    }
                }
            },
            if let Some(shift_sym) = crate::ui::keycodes::get_shift_symbol(code) {
                div { class: "key-labels-dual",
                    span { class: "key-label-shift", "{shift_sym}" }
                    span { class: "key-label-primary", "{label}" }
                }
            } else {
                "{label}"
            }
        }
    }
}

#[component]
pub fn App() -> Element {
    let mut devices = use_signal(|| Vec::<KeyboardInfo>::new());
    let mut status = use_signal(|| "Ready".to_string());
    let mut selected_device = use_signal(|| None::<KeyboardInfo>);
    
    let mut via_def = use_signal(|| None::<ViaDefinition>);
    let mut physical_keys = use_signal(|| Vec::<PhysicalKey>::new());
    
    let mut selected_key = use_signal(|| None::<PhysicalKey>);
    let mut dragged_keycode = use_signal(|| None::<(String, String, u16)>);
    let mut hovered_key = use_signal(|| None::<PhysicalKey>);
    let mut mouse_pos = use_signal(|| None::<(f32, f32)>);
    let mut active_main_tab = use_signal(|| "Keymap".to_string());
    let mut active_tab = use_signal(|| "Basic".to_string());
    
    let mut layer_keycodes = use_signal(|| HashMap::<(u8, u8), u16>::new());
    let mut active_layer = use_signal(|| 0u8);
    let layer_count = use_signal(|| 4u8);
    let macro_count = use_signal(|| 16u8);
    let macro_buffer = use_signal(|| Vec::<u8>::new());
    let macro_buffer_size = use_signal(|| 0u16);
    let mut protocol_version = use_signal(|| 0u16);
    let decoded_macros = use_signal(|| vec!["".to_string(); 16]);
    let is_loading_layer = use_signal(|| false);
    let mut expanded_nodes = use_signal(|| std::collections::HashSet::<String>::new());
    let logo_data_uri = use_signal(|| {
        format!("data:image/png;base64,{}", base64_encode(LOGO_BYTES))
    });
    let mut import_warnings = use_signal(|| None::<Vec<String>>);
    let features_version = use_signal(|| 0u64);
    let mut is_dragging_keys = use_signal(|| false);
    let mut show_any_modal = use_signal(|| false);
    let mut any_custom_text = use_signal(|| "".to_string());
    let mut stored_keycodes = use_signal(|| HashMap::<(u8, u8, u8), u16>::new());
    let mut layout_source = use_signal(|| crate::layout::storage::LayoutSource::None);

    #[cfg(not(target_arch = "wasm32"))]
    use_future(move || async move {
        loop {
            if let Ok(found) = crate::hid::manager::scan_for_keyboards() {
                let current = devices.read().clone();
                let mut merged: Vec<_> = current.into_iter().filter(|d| d.path.starts_with("json_") || d.path.starts_with("webhid_")).collect();
                for d in found {
                    if !merged.iter().any(|existing| existing.path == d.path) {
                        merged.push(d);
                    }
                }
                if *devices.read() != merged {
                    devices.set(merged);
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    });
    
    let reload_layer = move |layer: u8, path: String, keys: Vec<PhysicalKey>, mut keycodes_sig: Signal<HashMap<(u8, u8), u16>>, mut loading_sig: Signal<bool>, mut status_sig: Signal<String>| {
        loading_sig.set(true);
        let stored = stored_keycodes.read().clone();
        spawn(async move {
            let mut new_map = HashMap::new();
            for key in &keys {
                let r = key.matrix_row as u8;
                let c = key.matrix_col as u8;
                if let Some(&code) = stored.get(&(layer, r, c)) {
                    new_map.insert((r, c), code);
                } else if layer == 0 && key.default_code != 0 {
                    new_map.insert((r, c), key.default_code);
                }
            }

            #[cfg(not(target_arch = "wasm32"))]
            if let Ok(api) = hidapi::HidApi::new() {
                if let Ok(c_path) = std::ffi::CString::new(path) {
                    if let Ok(device) = api.open_path(&c_path) {
                        let via = crate::hid::via_protocol::ViaKeyboard::new(&device);
                        for key in &keys {
                            let row = key.matrix_row as u8;
                            let col = key.matrix_col as u8;
                            if let Ok(code) = via.get_keycode(layer, row, col) {
                                new_map.insert((row, col), code);
                            }
                        }
                    }
                }
            }

            #[cfg(target_arch = "wasm32")]
            {
                let via = crate::hid::via_protocol::ViaKeyboard;
                for key in &keys {
                    let row = key.matrix_row as u8;
                    let col = key.matrix_col as u8;
                    if let Ok(code) = via.get_keycode_async(layer, row, col).await {
                        if code != 0 {
                            new_map.insert((row, col), code);
                        }
                    }
                }
            }

            keycodes_sig.set(new_map);
            status_sig.set(format!("Layer {} active", layer));
            loading_sig.set(false);
        });
    };

    let export_layout = {
        let selected_device = selected_device.clone();
        let via_def = via_def.clone();
        let physical_keys = physical_keys.clone();
        let layer_count = layer_count.clone();
        let decoded_macros = decoded_macros.clone();
        let status = status.clone();
        move |_| {
            if let Some(dev_info) = selected_device.read().as_ref() {
                if let Some(def) = via_def.read().as_ref() {
                    let default_name = format!("{}_layout.json", dev_info.product_string.replace(" ", "_").to_lowercase());
                    #[cfg(not(target_arch = "wasm32"))]
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("JSON", &["json"])
                        .set_file_name(&default_name)
                        .save_file() {
                        let dev_path = dev_info.path.clone();
                        let vid = dev_info.vendor_id;
                        let pid = dev_info.product_id;
                        let keys = physical_keys.read().clone();
                        let l_count = *layer_count.read();
                        let m_decoded = decoded_macros.read().clone();
                        let def_clone = def.clone();
                        let mut status_sig = status.clone();
                        
                        spawn(async move {
                            let mut layers_data = Vec::new();
                            let mut warnings = Vec::new();
                            
                            #[cfg(not(target_arch = "wasm32"))]
                            if let Ok(api) = hidapi::HidApi::new() {
                                if let Ok(c_path) = std::ffi::CString::new(dev_path.clone()) {
                                    if let Ok(dev) = api.open_path(&c_path) {
                                        let via = crate::hid::via_protocol::ViaKeyboard::new(&dev);
                                        
                                        // Fetch all keycodes
                                        for layer in 0..l_count {
                                            let mut layer_keys = Vec::new();
                                            for key in &keys {
                                                let row = key.matrix_row as u8;
                                                let col = key.matrix_col as u8;
                                                match via.get_keycode(layer, row, col) {
                                                    Ok(code) => {
                                                        layer_keys.push(KeycodeBackupValue { row, col, value: code });
                                                    }
                                                    Err(_) => {
                                                        warnings.push(format!("Could not read keycode at Layer {}, Row {}, Col {}", layer, row, col));
                                                    }
                                                }
                                            }
                                            layers_data.push(layer_keys);
                                        }
                                        
                                        // Fetch custom features
                                        let mut features_data = Vec::new();
                                        let mut controls = Vec::new();
                                        for menu in &def_clone.menus {
                                            find_controls(menu, "", &mut controls);
                                        }
                                        
                                        for (path, control) in controls {
                                            let (channel, offset) = if let Some(content) = control.get("content").and_then(|c| c.as_array()) {
                                                if content.len() >= 3 {
                                                    let ch = content[1].as_u64().unwrap_or(0) as u8;
                                                    let off = content[2].as_u64().unwrap_or(0) as u8;
                                                    (ch, off)
                                                } else {
                                                    (0, 0)
                                                }
                                            } else {
                                                (0, 0)
                                            };
                                            
                                            let mut max_val = 255u16;
                                            if let Some(options) = control.get("options").and_then(|o| o.as_array()) {
                                                if options.len() >= 2 {
                                                    max_val = options[1].as_u64().unwrap_or(255) as u16;
                                                }
                                            }
                                            
                                            match via.get_custom_value(channel, offset) {
                                                Ok(val_bytes) => {
                                                    let val = if max_val > 255 {
                                                        u16::from_be_bytes(val_bytes)
                                                    } else {
                                                        val_bytes[0] as u16
                                                    };
                                                    features_data.push(FeatureBackupValue {
                                                        label: path.clone(),
                                                        channel,
                                                        offset,
                                                        value: val,
                                                    });
                                                }
                                                Err(_) => {
                                                    warnings.push(format!("Could not read custom feature '{}'", path));
                                                }
                                            }
                                        }
                                        
                                        let backup = LayoutBackup {
                                            version: 1,
                                            vendor_id: vid,
                                            product_id: pid,
                                            layers: layers_data,
                                            macros: m_decoded,
                                            features: features_data,
                                        };
                                        
                                        if let Ok(json_str) = serde_json::to_string_pretty(&backup) {
                                            if std::fs::write(&path, json_str).is_ok() {
                                                status_sig.set(format!("Exported layout to {}", path.display()));
                                            } else {
                                                status_sig.set("Failed to write JSON backup file".to_string());
                                            }
                                        }
                                    }
                                }
                            }
                        });
                    }
                    #[cfg(target_arch = "wasm32")]
                    {
                        let dev_path = dev_info.path.clone();
                        let vid = dev_info.vendor_id;
                        let pid = dev_info.product_id;
                        let keys = physical_keys.read().clone();
                        let l_count = *layer_count.read();
                        let m_decoded = decoded_macros.read().clone();
                        let def_clone = def.clone();
                        let mut status_sig = status.clone();
                        let default_name_clone = default_name.clone();

                        spawn(async move {
                            let mut layers_data = Vec::new();
                            let via = crate::hid::via_protocol::ViaKeyboard;

                            for layer in 0..l_count {
                                let mut layer_keys = Vec::new();
                                for key in &keys {
                                    let row = key.matrix_row as u8;
                                    let col = key.matrix_col as u8;
                                    if let Ok(code) = via.get_keycode_async(layer, row, col).await {
                                        layer_keys.push(KeycodeBackupValue { row, col, value: code });
                                    }
                                }
                                layers_data.push(layer_keys);
                            }

                            let mut features_data = Vec::new();
                            let mut controls = Vec::new();
                            for menu in &def_clone.menus {
                                find_controls(menu, "", &mut controls);
                            }

                            for (path, control) in controls {
                                let (channel, offset) = if let Some(content) = control.get("content").and_then(|c| c.as_array()) {
                                    if content.len() >= 3 {
                                        let ch = content[1].as_u64().unwrap_or(0) as u8;
                                        let off = content[2].as_u64().unwrap_or(0) as u8;
                                        (ch, off)
                                    } else {
                                        (0, 0)
                                    }
                                } else {
                                    (0, 0)
                                };
                                
                                let mut max_val = 255u16;
                                if let Some(options) = control.get("options").and_then(|o| o.as_array()) {
                                    if options.len() >= 2 {
                                        max_val = options[1].as_u64().unwrap_or(255) as u16;
                                    }
                                }
                                
                                if let Ok(val_bytes) = via.get_custom_value_async(channel, offset).await {
                                    let val = if max_val > 255 {
                                        u16::from_be_bytes(val_bytes)
                                    } else {
                                        val_bytes[0] as u16
                                    };
                                    features_data.push(FeatureBackupValue {
                                        label: path.clone(),
                                        channel,
                                        offset,
                                        value: val,
                                    });
                                }
                            }

                            let backup = LayoutBackup {
                                version: 1,
                                vendor_id: vid,
                                product_id: pid,
                                layers: layers_data,
                                macros: m_decoded,
                                features: features_data,
                            };

                            if let Ok(json_str) = serde_json::to_string_pretty(&backup) {
                                let eval_code = format!(r#"
                                    const blob = new Blob([{}], {{ type: 'application/json' }});
                                    const url = URL.createObjectURL(blob);
                                    const a = document.createElement('a');
                                    a.href = url;
                                    a.download = '{}';
                                    document.body.appendChild(a);
                                    a.click();
                                    document.body.removeChild(a);
                                    URL.revokeObjectURL(url);
                                "#, serde_json::to_string(&json_str).unwrap(), default_name_clone);
                                let _ = js_sys::eval(&eval_code);
                                status_sig.set(format!("Exported layout to {}", default_name_clone));
                            }
                        });
                    }
                }
            }
        }
    };

    let import_layout = {
        let selected_device = selected_device.clone();
        let macro_count = macro_count.clone();
        let macro_buffer_size = macro_buffer_size.clone();
        let protocol_version = protocol_version.clone();
        let physical_keys = physical_keys.clone();
        let status = status.clone();
        let layer_keycodes = layer_keycodes.clone();
        let macro_buffer = macro_buffer.clone();
        let decoded_macros = decoded_macros.clone();
        let active_layer = active_layer.clone();
        let is_loading_layer = is_loading_layer.clone();
        let import_warnings = import_warnings.clone();
        let via_def = via_def.clone();
        let features_version = features_version.clone();
        move |_| {
            let dev_info_opt = selected_device.read().clone();
            let m_count = *macro_count.read();
            let m_size = *macro_buffer_size.read();
            let prot = *protocol_version.read();
            let keys = physical_keys.read().clone();
            let active_l = *active_layer.read();
            let via_def_val = via_def.read().clone();
            
            let mut status_sig = status.clone();
            let mut layer_keycodes_sig = layer_keycodes.clone();
            let mut macro_buffer_sig = macro_buffer.clone();
            let mut decoded_macros_sig = decoded_macros.clone();
            let mut is_loading = is_loading_layer.clone();
            let mut import_warnings_sig = import_warnings.clone();
            let mut features_version_sig = features_version.clone();
            
            spawn(async move {
                if let Ok(Some((_filename, json_str))) = crate::layout::storage::pick_and_read_json_file().await {
                    let mut warnings = Vec::new();
                    match serde_json::from_str::<LayoutBackup>(&json_str) {
                        Ok(backup) => {
                            if let Some(dev_info) = dev_info_opt {
                                let dev_path = dev_info.path.clone();
                                let vid = dev_info.vendor_id;
                                let pid = dev_info.product_id;
                                if backup.vendor_id != vid || backup.product_id != pid {
                                    warnings.push(format!(
                                        "Mismatched device! Backup is for Vendor 0x{:04X}, Product 0x{:04X}. Active device is Vendor 0x{:04X}, Product 0x{:04X}.",
                                        backup.vendor_id, backup.product_id, vid, pid
                                    ));
                                }
                                    
                                    #[cfg(not(target_arch = "wasm32"))]
                                    if let Ok(api) = hidapi::HidApi::new() {
                                        if let Ok(c_path) = std::ffi::CString::new(dev_path.clone()) {
                                            if let Ok(dev) = api.open_path(&c_path) {
                                                let via = crate::hid::via_protocol::ViaKeyboard::new(&dev);
                                                
                                                // 1. Write keycodes
                                                for (l_idx, layer_keys) in backup.layers.iter().enumerate() {
                                                    for key_val in layer_keys {
                                                        match via.set_keycode(l_idx as u8, key_val.row, key_val.col, key_val.value) {
                                                            Ok(_) => {},
                                                            Err(_) => {
                                                                warnings.push(format!("Failed to write keycode at Layer {}, Row {}, Col {}", l_idx, key_val.row, key_val.col));
                                                            }
                                                        }
                                                    }
                                                }
                                                
                                                // 2. Write custom features
                                                let mut controls = Vec::new();
                                                if let Some(def) = &via_def_val {
                                                    for menu in &def.menus {
                                                        find_controls(menu, "", &mut controls);
                                                    }
                                                }
                                                
                                                for feature in backup.features {
                                                    let current_control = controls.iter().find(|(path, _)| {
                                                        path == &feature.label
                                                    });
                                                    
                                                    let (channel, offset) = if let Some((_, control)) = current_control {
                                                        let (ch, off) = if let Some(content) = control.get("content").and_then(|c| c.as_array()) {
                                                            if content.len() >= 3 {
                                                                let ch = crate::ui::menus::json_to_u64(&content[1]).unwrap_or(0) as u8;
                                                                let off = crate::ui::menus::json_to_u64(&content[2]).unwrap_or(0) as u8;
                                                                (ch, off)
                                                            } else {
                                                                (feature.channel, feature.offset)
                                                            }
                                                        } else {
                                                            (feature.channel, feature.offset)
                                                        };
                                                        (ch, off)
                                                    } else {
                                                        (feature.channel, feature.offset)
                                                    };
                                                    
                                                    let val_bytes = if feature.value > 255 {
                                                        feature.value.to_be_bytes()
                                                    } else {
                                                        [feature.value as u8, 0]
                                                    };
                                                    match via.set_custom_value(channel, offset, val_bytes) {
                                                        Ok(_) => {
                                                            std::thread::sleep(std::time::Duration::from_millis(50));
                                                            let _ = via.custom_save(channel, offset);
                                                        },
                                                        Err(_) => {
                                                            warnings.push(format!("Failed to write custom feature '{}'", feature.label));
                                                        }
                                                    }
                                                }
                                                
                                                // 3. Write macros
                                                let mut encoded_buffer = Vec::new();
                                                for mac_text in &backup.macros {
                                                    let encoded = crate::layout::macro_parser::encode_macro(mac_text, prot);
                                                    encoded_buffer.extend(encoded);
                                                    encoded_buffer.push(0); // Null terminator
                                                }
                                                
                                                if m_size > 0 {
                                                    if encoded_buffer.len() > m_size as usize {
                                                        warnings.push(format!(
                                                            "Macros are too large! Combined size is {} bytes, but limit is {} bytes. Some macros were truncated.",
                                                            encoded_buffer.len(), m_size
                                                        ));
                                                        encoded_buffer.truncate(m_size as usize);
                                                    } else {
                                                        while encoded_buffer.len() < m_size as usize {
                                                            encoded_buffer.push(0);
                                                        }
                                                    }
                                                    
                                                    let mut write_error = false;
                                                    for offset in (0..m_size).step_by(28) {
                                                        let chunk_size = std::cmp::min(28, m_size - offset) as u8;
                                                        let mut chunk = vec![0u8; chunk_size as usize];
                                                        let start = offset as usize;
                                                        let end = std::cmp::min(start + chunk_size as usize, encoded_buffer.len());
                                                        chunk[0..(end - start)].copy_from_slice(&encoded_buffer[start..end]);
                                                        
                                                        if via.set_macro_buffer(offset, &chunk).is_err() {
                                                            write_error = true;
                                                        }
                                                    }
                                                    
                                                    if write_error {
                                                        warnings.push("Failed to write some chunks of the macro buffer.".to_string());
                                                    }
                                                    
                                                    macro_buffer_sig.set(encoded_buffer);
                                                    
                                                    // Re-decode macros
                                                    let split = crate::layout::macro_parser::split_macro_buffer(&macro_buffer_sig.read(), m_count, m_size);
                                                    let decoded: Vec<String> = split.iter().map(|b| crate::layout::macro_parser::decode_macro(b, prot)).collect();
                                                    decoded_macros_sig.set(decoded);
                                                }
                                                
                                                // Reload active layer
                                                is_loading.set(true);
                                                let mut new_map = HashMap::new();
                                                for key in &keys {
                                                    let row = key.matrix_row as u8;
                                                    let col = key.matrix_col as u8;
                                                    if let Ok(code) = via.get_keycode(active_l, row, col) {
                                                        new_map.insert((row, col), code);
                                                    }
                                                }
                                                layer_keycodes_sig.set(new_map);
                                                is_loading.set(false);
                                            }
                                        }
                                    }
                                }
                        }
                        Err(err) => {
                            warnings.push(format!("Failed to parse backup file: {}", err));
                        }
                    }
                    
                    if !warnings.is_empty() {
                        import_warnings_sig.set(Some(warnings));
                        status_sig.set("Import finished with warnings".to_string());
                    } else {
                        status_sig.set("Imported layout successfully!".to_string());
                    }
                    let current_ver = *features_version_sig.read();
                    features_version_sig.set(current_ver + 1);
                }
            });
        }
    };

    rsx! {
        style { "{STYLE}" }
        div {
            class: format!("app-container {}", if dragged_keycode.read().is_some() { "dragging-keycode" } else { "" }),
            onmouseup: move |_| {
                dragged_keycode.set(None);
                hovered_key.set(None);
                is_dragging_keys.set(false);
                mouse_pos.set(None);
            },
            onmousemove: move |e| {
                if dragged_keycode.read().is_some() {
                    let coords = e.page_coordinates();
                    mouse_pos.set(Some((coords.x as f32, coords.y as f32)));
                }
            },
            if cfg!(not(target_arch = "wasm32")) {
                div {
                    class: "window-controls-far-right",
                    onmousedown: move |e| e.stop_propagation(),
                    button {
                        class: "win-btn",
                        title: "Minimize",
                        onclick: move |_| {
                            #[cfg(not(target_arch = "wasm32"))]
                            dioxus::desktop::window().set_minimized(true);
                        },
                        "🗕"
                    }
                    button {
                        class: "win-btn",
                        title: "Maximize",
                        onclick: move |_| {
                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                let win = dioxus::desktop::window();
                                win.set_maximized(!win.is_maximized());
                            }
                        },
                        "🗖"
                    }
                    button {
                        class: "win-btn close-btn",
                        title: "Close",
                        onclick: move |_| {
                            #[cfg(not(target_arch = "wasm32"))]
                            dioxus::desktop::window().close();
                        },
                        "✕"
                    }
                }
            }
            aside { class: "sidebar",
                div {
                    class: "sidebar-header",
                    style: "cursor: move;",
                    onmousedown: move |_| {
                        #[cfg(not(target_arch = "wasm32"))]
                        dioxus::desktop::window().drag();
                    },
                    img {
                        src: "{logo_data_uri}",
                        class: "nav-logo",
                    }
                    h1 { "via-rs" }
                }
                div { class: "sidebar-content",
                    div { class: "section-title",
                        span { "DEVICES" }
                        if cfg!(not(target_arch = "wasm32")) && devices.read().is_empty() {
                            span { class: "status-icon scanning", title: "Scanning...", "⟳" }
                        }
                    }
                    if cfg!(target_arch = "wasm32") {
                        div { class: "webhid-connect-container",
                            button {
                                class: "webhid-connect-btn",
                                onclick: move |_| {
                                    spawn(async move {
                                        #[cfg(target_arch = "wasm32")]
                                        match crate::hid::manager::request_webhid_device().await {
                                            Ok(Some(dev)) => {
                                                let is_new = !devices.read().iter().any(|d| d.path == dev.path);
                                                if is_new {
                                                    devices.write().push(dev.clone());
                                                }
                                                selected_device.set(Some(dev.clone()));
                                                status.set(format!("Connected to {}", dev.product_string));
                                                
                                                let dev_layout_clone = dev.clone();
                                                let mut via_def_sig = via_def.clone();
                                                let mut layout_source_sig = layout_source.clone();
                                                let mut physical_keys_sig = physical_keys.clone();
                                                let mut status_sig = status.clone();
                                                let active_layer_sig = active_layer.clone();
                                                let layer_keycodes_sig = layer_keycodes.clone();
                                                let is_loading_layer_sig = is_loading_layer.clone();
                                                spawn(async move {
                                                    if let Some((saved, src)) = crate::layout::storage::get_layout_with_source(dev_layout_clone.vendor_id, dev_layout_clone.product_id).await {
                                                        *via_def_sig.write() = Some(saved.clone());
                                                        layout_source_sig.set(src);
                                                        let p_keys = crate::layout::kle::parse_kle(&saved.layouts.keymap);
                                                        physical_keys_sig.set(p_keys.clone());
                                                        status_sig.set(format!("Connected to {}", dev_layout_clone.product_string));
                                                        reload_layer(*active_layer_sig.read(), dev_layout_clone.path.clone(), p_keys, layer_keycodes_sig, is_loading_layer_sig, status_sig);
                                                    } else {
                                                        *via_def_sig.write() = None;
                                                        layout_source_sig.set(crate::layout::storage::LayoutSource::None);
                                                        physical_keys_sig.set(Vec::new());
                                                    }
                                                });

                                                let dev_path = dev.path.clone();
                                                let pv = protocol_version.clone();
                                                let lc = layer_count.clone();
                                                let mc = macro_count.clone();
                                                let mbs = macro_buffer_size.clone();
                                                let mb = macro_buffer.clone();
                                                let dm = decoded_macros.clone();
                                                spawn(async move {
                                                    fetch_keyboard_data(dev_path, pv, lc, mc, mbs, mb, dm).await;
                                                });
                                            },
                                            Ok(None) => {
                                                status.set("No device selected".to_string());
                                            },
                                            Err(err) => {
                                                status.set(format!("WebHID error: {}", err));
                                            }
                                        }
                                    });
                                },
                                "🔌 Connect HID Keyboard"
                            }
                        }
                    }
                    ul { class: "device-list tree-view",
                        for (device, dev_id, layers_id, macros_id, features_id) in devices.read().clone().into_iter().map(|d| {
                            let id = d.path.clone();
                            let l = format!("{}/LAYERS", id);
                            let m = format!("{}/MACROS", id);
                            let f = format!("{}/FEATURES", id);
                            (d, id, l, m, f)
                        }) {
                            li { 
                                class: if selected_device.read().as_ref() == Some(&device) { "tree-item device-item selected" } else { "tree-item device-item" },
                                onclick: {
                                    let dev_id_clone = dev_id.clone();
                                    move |_| {
                                    let is_already_selected = selected_device.read().as_ref() == Some(&device);
                                    
                                    if is_already_selected {
                                        // Just toggle expansion
                                        let mut nodes = expanded_nodes.write();
                                        if nodes.contains(&dev_id_clone) {
                                            nodes.remove(&dev_id_clone);
                                        } else {
                                            nodes.insert(dev_id_clone.clone());
                                        }
                                    } else {
                                        // Select and load
                                        expanded_nodes.write().clear();
                                        expanded_nodes.write().insert(dev_id_clone.clone());
                                        selected_device.set(Some(device.clone()));
                                        status.set(format!("Connected to {}", device.product_string));
                                        selected_key.set(None);
                                        layer_keycodes.set(std::collections::HashMap::new());
                                        
                                        let dev_path = device.path.clone();
                                        let pv = protocol_version.clone();
                                        let lc = layer_count.clone();
                                        let mc = macro_count.clone();
                                        let mbs = macro_buffer_size.clone();
                                        let mb = macro_buffer.clone();
                                        let dm = decoded_macros.clone();
                                        spawn(async move {
                                            fetch_keyboard_data(dev_path, pv, lc, mc, mbs, mb, dm).await;
                                        });

                                        let dev_layout_clone = device.clone();
                                        let mut via_def_sig = via_def.clone();
                                        let mut layout_source_sig = layout_source.clone();
                                        let mut physical_keys_sig = physical_keys.clone();
                                        let mut status_sig = status.clone();
                                        let active_layer_sig = active_layer.clone();
                                        let layer_keycodes_sig = layer_keycodes.clone();
                                        let is_loading_layer_sig = is_loading_layer.clone();
                                        spawn(async move {
                                            if let Some((saved, src)) = crate::layout::storage::get_layout_with_source(dev_layout_clone.vendor_id, dev_layout_clone.product_id).await {
                                                *via_def_sig.write() = Some(saved.clone());
                                                layout_source_sig.set(src);
                                                let p_keys = crate::layout::kle::parse_kle(&saved.layouts.keymap);
                                                physical_keys_sig.set(p_keys.clone());
                                                status_sig.set(format!("Connected to {}", dev_layout_clone.product_string));
                                                reload_layer(*active_layer_sig.read(), dev_layout_clone.path.clone(), p_keys, layer_keycodes_sig, is_loading_layer_sig, status_sig);
                                            } else {
                                                *via_def_sig.write() = None;
                                                layout_source_sig.set(crate::layout::storage::LayoutSource::None);
                                                physical_keys_sig.set(Vec::new());
                                            }
                                        });
                                    }
                                } },
                                span { 
                                    class: if expanded_nodes.read().contains(&dev_id) { "chevron expanded" } else { "chevron collapsed" },
                                    "▶" 
                                }
                                span { class: "device-icon", "⌨" }
                                span { class: "device-name", "{device.product_string}" }
                            }
                            
                            // Render children if the device is expanded AND it is selected (so via_def is loaded)
                            if expanded_nodes.read().contains(&dev_id) && selected_device.read().as_ref() == Some(&device) {
                                if let Some(def) = via_def.read().as_ref() {
                                    div { class: "device-tree-children",
                                        // LAYERS parent node
                                        div {
                                            class: "tree-item parent-node",
                                            onclick: {
                                                let layers_id_clone = layers_id.clone();
                                                move |_| {
                                                    let mut nodes = expanded_nodes.write();
                                                    if nodes.contains(&layers_id_clone) { nodes.remove(&layers_id_clone); } else { nodes.insert(layers_id_clone.clone()); }
                                                }
                                            },
                                            span { class: if expanded_nodes.read().contains(&layers_id) { "chevron expanded" } else { "chevron collapsed" }, "▶" }
                                            span { class: "tree-label", "Layers" }
                                        }
                                        
                                        if expanded_nodes.read().contains(&layers_id) {
                                            for (l, p_path) in (0..*layer_count.read()).map(|l| (l, device.path.clone())) {
                                                div {
                                                    class: if *active_main_tab.read() == "Keymap" && *active_layer.read() == l { "tree-item leaf-node active" } else { "tree-item leaf-node" },
                                                    onclick: move |_| {
                                                        active_main_tab.set("Keymap".to_string());
                                                        active_layer.set(l);
                                                        let p_keys = physical_keys.read().clone();
                                                        reload_layer(l, p_path.clone(), p_keys, layer_keycodes, is_loading_layer, status);
                                                    },
                                                    span { class: "tree-label", "Layer {l}" }
                                                }
                                            }
                                        }
                                        
                                        // MACROS parent node
                                        div {
                                            class: "tree-item parent-node",
                                            onclick: {
                                                let macros_id_clone = macros_id.clone();
                                                move |_| {
                                                    let mut nodes = expanded_nodes.write();
                                                    if nodes.contains(&macros_id_clone) { nodes.remove(&macros_id_clone); } else { nodes.insert(macros_id_clone.clone()); }
                                                }
                                            },
                                            span { class: if expanded_nodes.read().contains(&macros_id) { "chevron expanded" } else { "chevron collapsed" }, "▶" }
                                            span { class: "tree-label", "Macros" }
                                        }
                                        
                                        if expanded_nodes.read().contains(&macros_id) {
                                            for m in 0..*macro_count.read() {
                                                div {
                                                    class: if *active_main_tab.read() == format!("Macro {}", m) { "tree-item leaf-node active" } else { "tree-item leaf-node" },
                                                    onclick: move |_| {
                                                        active_main_tab.set(format!("Macro {}", m));
                                                    },
                                                    span { class: "tree-label", "Macro {m}" }
                                                }
                                            }
                                        }
                                        
                                        if !def.menus.is_empty() {
                                            // FEATURES parent node
                                            div {
                                                class: "tree-item parent-node",
                                                onclick: {
                                                    let features_id_clone = features_id.clone();
                                                    move |_| {
                                                        let mut nodes = expanded_nodes.write();
                                                        if nodes.contains(&features_id_clone) { nodes.remove(&features_id_clone); } else { nodes.insert(features_id_clone.clone()); }
                                                    }
                                                },
                                                span { class: if expanded_nodes.read().contains(&features_id) { "chevron expanded" } else { "chevron collapsed" }, "▶" }
                                                span { class: "tree-label", "Features" }
                                            }
                                            
                                            if expanded_nodes.read().contains(&features_id) {
                                                for menu in def.menus.iter() {
                                                    crate::ui::menus::MenuNode { 
                                                        node: menu.clone(), 
                                                        selected_device: selected_device.clone(), 
                                                        status: status.clone(), 
                                                        level: 2, 
                                                        parent_id: features_id.clone(), 
                                                        expanded_nodes: expanded_nodes.clone(),
                                                        features_version: features_version.clone()
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            main { class: "main-content",
                if let Some(dev) = selected_device.read().as_ref() {
                    div {
                        class: "editor-header",
                        onmousedown: move |_| {
                            #[cfg(not(target_arch = "wasm32"))]
                            dioxus::desktop::window().drag();
                        },
                        div {
                            class: "breadcrumb",
                            onmousedown: move |e| e.stop_propagation(),
                            span { "Devices" }
                            span { class: "separator", "›" }
                            span { "{dev.product_string}" }
                            if *active_main_tab.read() == "Keymap" {
                                span { class: "separator", "›" }
                                span { "Layer {active_layer.read()}" }
                            }
                            span { class: "separator", style: "margin: 0 10px;", "|" }
                            button {
                                class: "reload-json-btn",
                                onclick: move |_| {
                                    let mut via_def_sig = via_def.clone();
                                    let mut physical_keys_sig = physical_keys.clone();
                                    let mut status_sig = status.clone();
                                    let selected_dev = selected_device.read().clone();
                                    let mut pv = protocol_version.clone();
                                    let mut lc_sig = layer_count.clone();
                                    let mut mc_sig = macro_count.clone();
                                    let mut mb_size_sig = macro_buffer_size.clone();
                                    let mut mb_sig = macro_buffer.clone();
                                    let mut dm_sig = decoded_macros.clone();
                                    let active_l = *active_layer.read();
                                    let mut layer_keycodes_sig = layer_keycodes.clone();
                                    let mut is_loading_layer_sig = is_loading_layer.clone();
                                    let mut layout_source_sig = layout_source.clone();

                                    spawn(async move {
                                        if let Ok(Some((filename, content))) = crate::layout::storage::pick_and_read_json_file().await {
                                            if let Ok(new_def) = serde_json::from_str::<crate::layout::parser::ViaDefinition>(&content) {
                                                let p_keys = crate::layout::kle::parse_kle(&new_def.layouts.keymap);
                                                via_def_sig.set(Some(new_def.clone()));
                                                layout_source_sig.set(crate::layout::storage::LayoutSource::UserLoaded(new_def.name.clone()));
                                                physical_keys_sig.set(p_keys.clone());
                                                status_sig.set(format!("Reloaded {}", filename));

                                                if let Some(dev_info) = selected_dev {
                                                    crate::layout::storage::save_layout(dev_info.vendor_id, dev_info.product_id, &new_def);
                                                    let dev_path = dev_info.path.clone();
                                                    spawn(async move {
                                                        fetch_keyboard_data(dev_path, pv, lc_sig, mc_sig, mb_size_sig, mb_sig, dm_sig).await;
                                                    });
                                                    reload_layer(active_l, dev_info.path.clone(), p_keys, layer_keycodes_sig, is_loading_layer_sig, status_sig);
                                                }
                                            }
                                        }
                                    });
                                },
                                "Reload JSON"
                            }
                            span { class: "separator", style: "margin: 0 10px;", "|" }
                            button {
                                class: "reload-json-btn",
                                onclick: move |_| {
                                    dioxus::document::eval("window.print();");
                                },
                                "Print Layout"
                            }
                            span { class: "separator", style: "margin: 0 10px;", "|" }
                            button {
                                class: "reload-json-btn",
                                onclick: export_layout,
                                "Export Layout"
                            }
                            span { class: "separator", style: "margin: 0 10px;", "|" }
                            button {
                                class: "reload-json-btn",
                                onclick: import_layout,
                                "Import Layout"
                            }
                        }
                    }
                    div { class: "editor-pane",
                        if via_def.read().is_none() {
                            div {
                                class: "keyboard-layout-placeholder",
                                style: "cursor: move;",
                                onmousedown: move |_| {
                                    #[cfg(not(target_arch = "wasm32"))]
                                    dioxus::desktop::window().drag();
                                },
                                h3 { "Configure Keyboard" }
                                p { style: "color: #969696; font-size: 13px; margin-bottom: 20px;", "Load a via.json file to configure this device." }
                                button {
                                    class: "primary-btn",
                                    onmousedown: move |e| e.stop_propagation(),
                                    onclick: move |_| {
                                        let mut via_def_sig = via_def.clone();
                                        let mut physical_keys_sig = physical_keys.clone();
                                        let mut status_sig = status.clone();
                                        let selected_dev = selected_device.read().clone();
                                        let mut devices_sig = devices.clone();
                                        let mut selected_dev_sig = selected_device.clone();
                                        let mut expanded_nodes_sig = expanded_nodes.clone();
                                        let mut lc_sig = layer_count.clone();
                                        let active_l = *active_layer.read();
                                        let mut layer_keycodes_sig = layer_keycodes.clone();
                                        let mut is_loading_layer_sig = is_loading_layer.clone();
                                        let mut layout_source_sig = layout_source.clone();

                                        spawn(async move {
                                            match crate::layout::storage::pick_and_read_json_file().await {
                                                Ok(Some((filename, content))) => {
                                                    match serde_json::from_str::<crate::layout::parser::ViaDefinition>(&content) {
                                                        Ok(def) => {
                                                            let p_keys = crate::layout::kle::parse_kle(&def.layouts.keymap);
                                                            via_def_sig.set(Some(def.clone()));
                                                            layout_source_sig.set(crate::layout::storage::LayoutSource::UserLoaded(def.name.clone()));
                                                            physical_keys_sig.set(p_keys.clone());
                                                            status_sig.set(format!("Loaded {}", filename));

                                                            let vid = if def.vendor_id.starts_with("0x") || def.vendor_id.starts_with("0X") {
                                                                u16::from_str_radix(&def.vendor_id[2..], 16).unwrap_or(0)
                                                            } else {
                                                                def.vendor_id.parse::<u16>().unwrap_or(0)
                                                            };
                                                            let pid = if def.product_id.starts_with("0x") || def.product_id.starts_with("0X") {
                                                                u16::from_str_radix(&def.product_id[2..], 16).unwrap_or(0)
                                                            } else {
                                                                def.product_id.parse::<u16>().unwrap_or(0)
                                                            };

                                                            let dev_info = selected_dev.unwrap_or_else(|| KeyboardInfo {
                                                                vendor_id: vid,
                                                                product_id: pid,
                                                                path: format!("json_{:04X}_{:04X}", vid, pid),
                                                                product_string: def.name.clone(),
                                                            });

                                                            if !devices_sig.read().iter().any(|d| d.path == dev_info.path) {
                                                                devices_sig.write().push(dev_info.clone());
                                                            }
                                                            selected_dev_sig.set(Some(dev_info.clone()));
                                                            expanded_nodes_sig.write().insert(dev_info.path.clone());
                                                            expanded_nodes_sig.write().insert(format!("{}/LAYERS", dev_info.path));

                                                            crate::layout::storage::save_layout(dev_info.vendor_id, dev_info.product_id, &def);
                                                            let dev_path = dev_info.path.clone();
                                                            spawn(async move {
                                                                #[cfg(not(target_arch = "wasm32"))]
                                                                if let Ok(api) = hidapi::HidApi::new() {
                                                                    if let Ok(c_path) = std::ffi::CString::new(dev_path) {
                                                                        if let Ok(device) = api.open_path(&c_path) {
                                                                            let via = crate::hid::via_protocol::ViaKeyboard::new(&device);
                                                                            if let Ok(count) = via.get_layer_count() {
                                                                                if count > 0 && count <= 16 {
                                                                                    lc_sig.set(count);
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            });
                                                            reload_layer(active_l, dev_info.path.clone(), p_keys, layer_keycodes_sig, is_loading_layer_sig, status_sig);
                                                        },
                                                        Err(err) => {
                                                            status_sig.set(format!("Failed to parse via.json: {}", err));
                                                        }
                                                    }
                                                },
                                                Ok(None) => {},
                                                Err(err) => {
                                                    status_sig.set(format!("File load error: {}", err));
                                                }
                                            }
                                        });
                                    },
                                    "Load via.json"
                                }
                            }
                        } else if *active_main_tab.read() == "Keymap" {
                            div {
                                class: "keymap-pane",
                                style: "display: flex; flex-direction: column; width: 100%; height: 100%;",
                                onmouseup: move |_| { is_dragging_keys.set(false); },
                                div { class: "keyboard-layout-container",
                                    div { 
                                        class: "keyboard-layout",
                                        style: {
                                            let keys = physical_keys.read().clone();
                                            let max_w = keys.iter().map(|k| k.x + k.w).fold(0.0_f32, f32::max) * 55.0;
                                            let max_h = keys.iter().map(|k| k.y + k.h).fold(0.0_f32, f32::max) * 55.0;
                                            format!("width: {}px; height: {}px;", max_w, max_h)
                                        },
                                    {
                                        physical_keys.read().clone().into_iter().map(|key| {
                                            let key_down = key.clone();
                                            let key_enter = key.clone();
                                            let key_dragenter = key.clone();
                                            let key_drop = key.clone();
                                            let is_selected = selected_key.read().as_ref() == Some(&key);
                                            let is_hovered = hovered_key.read().as_ref() == Some(&key);
                                            let matrix_row = key.matrix_row;
                                            let matrix_col = key.matrix_col;
                                            let x = key.x;
                                            let y = key.y;
                                            let w = key.w;
                                            let h = key.h;
                                            let r = key.r;
                                            let rx = key.rx;
                                            let ry = key.ry;
                                            
                                            rsx! {
                                                div {
                                                    key: "{matrix_row}-{matrix_col}",
                                                    class: format!("keycap {} {}", if is_selected { "selected" } else { "" }, if is_hovered { "drop-target" } else { "" }),
                                                    style: format!(
                                                        "left: {}px; top: {}px; width: {}px; height: {}px; transform: rotate({}deg); transform-origin: {}px {}px;",
                                                        x * 55.0, y * 55.0, w * 55.0 - 5.0, h * 55.0 - 5.0, r, (rx - x) * 55.0, (ry - y) * 55.0
                                                    ),
                                                    draggable: "false",
                                                    onmousedown: move |_| {
                                                        selected_key.set(Some(key_down.clone()));
                                                        is_dragging_keys.set(true);
                                                    },
                                                    onmouseenter: move |_| {
                                                        if *is_dragging_keys.read() {
                                                            selected_key.set(Some(key_enter.clone()));
                                                        }
                                                        if dragged_keycode.read().is_some() {
                                                            hovered_key.set(Some(key_dragenter.clone()));
                                                        }
                                                    },
                                                    onmouseleave: move |_| {
                                                        if dragged_keycode.read().is_some() {
                                                            hovered_key.set(None);
                                                        }
                                                    },
                                                    onmouseup: move |e| {
                                                        let dragged = dragged_keycode.read().as_ref().cloned();
                                                        if let Some((l, _title, c)) = dragged {
                                                            e.stop_propagation();
                                                            hovered_key.set(None);
                                                            dragged_keycode.set(None);
                                                            mouse_pos.set(None);
                                                            if let Some(dev_info) = selected_device.read().as_ref() {
                                                                let path = dev_info.path.clone();
                                                                let row = key_drop.matrix_row as u8;
                                                                let col = key_drop.matrix_col as u8;
                                                                let layer = *active_layer.read();
                                                                
                                                                stored_keycodes.write().insert((layer, row, col), c);
                                                                layer_keycodes.write().insert((row, col), c);

                                                                let mut status_sig = status.clone();
                                                                
                                                                spawn(async move {
                                                                    #[cfg(not(target_arch = "wasm32"))]
                                                                    if let Ok(api) = hidapi::HidApi::new() {
                                                                        if let Ok(c_path) = std::ffi::CString::new(path) {
                                                                            if let Ok(device) = api.open_path(&c_path) {
                                                                                let via = crate::hid::via_protocol::ViaKeyboard::new(&device);
                                                                                match via.set_keycode(layer, row, col, c) {
                                                                                    Ok(_) => {
                                                                                        let _ = via.custom_save(0, 0);
                                                                                        layer_keycodes.write().insert((row, col), c);
                                                                                        status_sig.set(format!("Assigned {} to {},{}", l, row, col));
                                                                                    },
                                                                                    Err(e) => status_sig.set(format!("Error writing: {}", e)),
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    #[cfg(target_arch = "wasm32")]
                                                                    {
                                                                        let via = crate::hid::via_protocol::ViaKeyboard;
                                                                        match via.set_keycode_async(layer, row, col, c).await {
                                                                            Ok(_) => {
                                                                                gloo_timers::future::TimeoutFuture::new(50).await;
                                                                                let _ = via.custom_save_async(0, 0).await;
                                                                                status_sig.set(format!("Saved keycode 0x{:04X} to {},{} in EEPROM", c, row, col));
                                                                            },
                                                                            Err(e) => status_sig.set(format!("Error writing: {}", e)),
                                                                        }
                                                                    }
                                                                });
                                                            }
                                                        }
                                                    },
                                                    span { class: "key-matrix-corner", "{matrix_row},{matrix_col}" }
                                                    {
                                                        let code_opt = layer_keycodes.read().get(&(matrix_row as u8, matrix_col as u8)).copied();
                                                        let label_text = if let Some(code) = code_opt {
                                                            let l = crate::ui::keycodes::get_keycode_label(code, via_def.read().as_ref());
                                                            if code == 0 {
                                                                String::new()
                                                            } else if !l.is_empty() {
                                                                l
                                                            } else if !key.label.is_empty() {
                                                                key.label.clone()
                                                            } else {
                                                                String::new()
                                                            }
                                                        } else if !key.label.is_empty() {
                                                            key.label.clone()
                                                        } else {
                                                            String::new()
                                                        };

                                                        if let Some(code) = code_opt {
                                                            if let Some(shift_sym) = crate::ui::keycodes::get_shift_symbol(code) {
                                                                rsx! {
                                                                    div { class: "key-labels-dual",
                                                                        span { class: "key-label-shift", "{shift_sym}" }
                                                                        span { class: "key-label-primary", "{label_text}" }
                                                                    }
                                                                }
                                                            } else {
                                                                rsx! {
                                                                    span { class: "key-label-center", "{label_text}" }
                                                                }
                                                            }
                                                        } else {
                                                            rsx! {
                                                                span { class: "key-label-center", "{label_text}" }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        })
                                    }
                                }
                            }
                            
                            div { class: "palette-pane",
                                div { class: "palette-header",
                                    if let Some(key) = selected_key.read().clone() {
                                        div {
                                            h4 { style: "margin: 0 0 5px 0;", "Selected Key" }
                                            p { style: "color: #969696; font-size: 13px; margin: 0;", "Matrix: {key.matrix_row}, {key.matrix_col}" }
                                        }
                                        div { style: "text-align: right;",
                                            p { style: "font-weight: 600; margin: 0;",
                                                "Current Keycode: "
                                                if let Some(code) = layer_keycodes.read().get(&(key.matrix_row as u8, key.matrix_col as u8)) {
                                                    span { style: "font-family: monospace; color: var(--accent);", "{crate::ui::keycodes::get_keycode_label(*code, via_def.read().as_ref())} (0x{code:04X})" }
                                                }
                                            }
                                        }
                                    } else {
                                        div {
                                            h4 { style: "margin: 0 0 5px 0; color: #555;", "No Key Selected" }
                                            p { style: "color: #555; font-size: 13px; margin: 0;", "Click a key on the layout above to map." }
                                        }
                                    }
                                }
                                                        div { class: "palette-tabs",
                                    for tab in ["Basic", "Modifiers", "Media", "Mouse", "Macros", "Layers", "Special", "Lighting", "Custom"].iter() {
                                        button {
                                            class: "tab-btn",
                                            class: if active_tab.read().as_str() == *tab { "active" } else { "" },
                                            onclick: move |_| {
                                                active_tab.set(tab.to_string());
                                            },
                                            "{tab}"
                                        }
                                    }
                                }
                                
                                div { class: "palette-grid",
                                    if active_tab.read().as_str() == "Basic" {
                                        for &(label, title, code) in crate::ui::keycodes::BASIC_KEYCODES.iter() {
                                            KeycodeButton { label: label.to_string(), title: title.to_string(), code, selected_key, physical_keys, selected_device, status, layer_keycodes, active_layer, dragged_keycode, show_any_modal, stored_keycodes, any_custom_text, via_def }
                                        }
                                    } else if active_tab.read().as_str() == "Modifiers" {
                                        for &(label, title, code) in crate::ui::keycodes::MODIFIER_KEYCODES.iter() {
                                            KeycodeButton { label: label.to_string(), title: title.to_string(), code, selected_key, physical_keys, selected_device, status, layer_keycodes, active_layer, dragged_keycode, show_any_modal, stored_keycodes, any_custom_text, via_def }
                                        }
                                    } else if active_tab.read().as_str() == "Media" {
                                        for &(label, title, code) in crate::ui::keycodes::MEDIA_KEYCODES.iter() {
                                            KeycodeButton { label: label.to_string(), title: title.to_string(), code, selected_key, physical_keys, selected_device, status, layer_keycodes, active_layer, dragged_keycode, show_any_modal, stored_keycodes, any_custom_text, via_def }
                                        }
                                    } else if active_tab.read().as_str() == "Mouse" {
                                        for &(label, title, code) in crate::ui::keycodes::MOUSE_KEYCODES.iter() {
                                            KeycodeButton { label: label.to_string(), title: title.to_string(), code, selected_key, physical_keys, selected_device, status, layer_keycodes, active_layer, dragged_keycode, show_any_modal, stored_keycodes, any_custom_text, via_def }
                                        }
                                    } else if active_tab.read().as_str() == "Macros" {
                                        for &(label, title, code) in crate::ui::keycodes::MACRO_KEYCODES.iter() {
                                            KeycodeButton { label: label.to_string(), title: title.to_string(), code, selected_key, physical_keys, selected_device, status, layer_keycodes, active_layer, dragged_keycode, show_any_modal, stored_keycodes, any_custom_text, via_def }
                                        }
                                    } else if active_tab.read().as_str() == "Layers" {
                                        for &(label, title, code) in crate::ui::keycodes::LAYER_KEYCODES.iter() {
                                            KeycodeButton { label: label.to_string(), title: title.to_string(), code, selected_key, physical_keys, selected_device, status, layer_keycodes, active_layer, dragged_keycode, show_any_modal, stored_keycodes, any_custom_text, via_def }
                                        }
                                    } else if active_tab.read().as_str() == "Special" {
                                        for &(label, title, code) in crate::ui::keycodes::SPECIAL_KEYCODES.iter() {
                                            KeycodeButton { label: label.to_string(), title: title.to_string(), code, selected_key, physical_keys, selected_device, status, layer_keycodes, active_layer, dragged_keycode, show_any_modal, stored_keycodes, any_custom_text, via_def }
                                        }
                                    } else if active_tab.read().as_str() == "Lighting" {
                                        for &(label, title, code) in crate::ui::keycodes::LIGHTING_KEYCODES.iter() {
                                            KeycodeButton { label: label.to_string(), title: title.to_string(), code, selected_key, physical_keys, selected_device, status, layer_keycodes, active_layer, dragged_keycode, show_any_modal, stored_keycodes, any_custom_text, via_def }
                                        }
                                    } else if active_tab.read().as_str() == "Custom" {
                                        if let Some(def) = via_def.read().as_ref() {
                                            if def.custom_keycodes.is_empty() {
                                                p { style: "grid-column: 1 / -1; color: #969696;", "No custom keycodes defined for this layout." }
                                            } else {
                                                for (idx, custom) in def.custom_keycodes.iter().enumerate() {
                                                    KeycodeButton { label: custom.short_name.clone(), title: custom.title.clone(), code: 0x7E00 + idx as u16, selected_key, physical_keys, selected_device, status, layer_keycodes, active_layer, dragged_keycode, show_any_modal, stored_keycodes, any_custom_text, via_def }
                                                }
                                            }
                                        }
                                    }
                                }
                                

                            }
                            }
                        } else if active_main_tab.read().starts_with("Macro ") {
                            if let Ok(m_id) = active_main_tab.read().replace("Macro ", "").parse::<u8>() {
                                div { style: "padding: 30px; color: var(--text-main); width: 100%; box-sizing: border-box;",
                                    h2 { style: "margin-top: 0; margin-bottom: 20px;", "Macro {m_id}" }
                                    p { style: "margin-bottom: 20px; line-height: 1.5;", "Type your macro here. Standard text will be typed out. Enclose special keys in braces like {{KC_A}}, {{+KC_LSFT}}, {{-KC_LSFT}}, or {{Delay 100}}." }
                                    
                                    div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px;",
                                        h3 { style: "margin: 0;", "Macro Text" }
                                        button {
                                            class: "primary-btn",
                                            onclick: move |_| {
                                                if selected_device.read().is_some() {
                                                    let mut mb_sig = macro_buffer.clone();
                                                let dm_sig = decoded_macros.clone();
                                                let mc = *macro_count.read();
                                                let mbs = *macro_buffer_size.read();
                                                let prot = *protocol_version.read();
                                                let dev_info = selected_device.read().clone();
                                                let mut status_sig = status.clone();
                                                
                                                spawn(async move {
                                                    if let Some(dev) = dev_info {
                                                        let macros_encoded: Vec<Vec<u8>> = dm_sig.read().iter().map(|s| crate::layout::macro_parser::encode_macro(s, prot)).collect();
                                                        let new_buf = crate::layout::macro_parser::build_macro_buffer(&macros_encoded, mc, mbs);
                                                        mb_sig.set(new_buf.clone());
                                                        
                                                        #[cfg(not(target_arch = "wasm32"))]
                                                        if let Ok(api) = hidapi::HidApi::new() {
                                                            if let Ok(c_path) = std::ffi::CString::new(dev.path) {
                                                                if let Ok(device) = api.open_path(&c_path) {
                                                                    let via = crate::hid::via_protocol::ViaKeyboard::new(&device);
                                                                    let mut success = true;
                                                                    for offset in (0..new_buf.len()).step_by(28) {
                                                                        let chunk_size = std::cmp::min(28, new_buf.len() - offset);
                                                                        let chunk = &new_buf[offset..offset + chunk_size];
                                                                        if via.set_macro_buffer(offset as u16, chunk).is_err() {
                                                                            success = false;
                                                                            break;
                                                                        }
                                                                        std::thread::sleep(std::time::Duration::from_millis(10));
                                                                    }
                                                                    if success {
                                                                        let _ = via.custom_save(0, 0);
                                                                        status_sig.set(format!("Macro {} saved successfully", m_id));
                                                                    } else {
                                                                        status_sig.set(format!("Failed to save Macro {}", m_id));
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        #[cfg(target_arch = "wasm32")]
                                                        {
                                                            let via = crate::hid::via_protocol::ViaKeyboard;
                                                            let mut success = true;
                                                            for offset in (0..new_buf.len()).step_by(28) {
                                                                let chunk_size = std::cmp::min(28, new_buf.len() - offset);
                                                                let chunk = &new_buf[offset..offset + chunk_size];
                                                                if via.set_macro_buffer_async(offset as u16, chunk).await.is_err() {
                                                                    success = false;
                                                                    break;
                                                                }
                                                                gloo_timers::future::TimeoutFuture::new(15).await;
                                                            }
                                                            if success {
                                                                let _ = via.custom_save_async(0, 0).await;
                                                                status_sig.set(format!("Macro {} saved successfully to EEPROM", m_id));
                                                            } else {
                                                                status_sig.set(format!("Failed to save Macro {} via WebHID", m_id));
                                                            }
                                                        }
                                                    }
                                                });
                                                }
                                            },
                                            "Save to Keyboard"
                                        }
                                    }
                                    
                                    crate::ui::macro_builder::MacroBuilder {
                                        m_id,
                                        decoded_macros,
                                    }
                                }
                            }
                        }
                    }
                } else {
                    div {
                        class: "empty-workspace",
                        style: "cursor: move;",
                        onmousedown: move |_| {
                            #[cfg(not(target_arch = "wasm32"))]
                            dioxus::desktop::window().drag();
                        },
                        div { class: "logo-placeholder", "VIA" }
                        p { "Select a device to configure" }
                    }
                }
                
                div { class: "status-bar",
                    div { class: "status-left",
                        span { class: "status-item", "{status}" }
                    }
                    div { class: "status-right",
                        if let Some(def) = via_def.read().as_ref() {
                            match layout_source.read().clone() {
                                crate::layout::storage::LayoutSource::Predefined(_) => rsx! {
                                    span { class: "status-item", style: "color: var(--text-main); font-weight: 500;",
                                        "Predefined Layout: {def.name} ({def.vendor_id}:{def.product_id})"
                                    }
                                },
                                crate::layout::storage::LayoutSource::UserLoaded(_) => rsx! {
                                    span { class: "status-item", style: "color: var(--text-main); font-weight: 500;",
                                        "User Layout: {def.name} ({def.vendor_id}:{def.product_id})"
                                    }
                                },
                                crate::layout::storage::LayoutSource::Cached(_) => rsx! {
                                    span { class: "status-item", style: "color: var(--text-main); font-weight: 500;",
                                        "Cached Layout: {def.name} ({def.vendor_id}:{def.product_id})"
                                    }
                                },
                                crate::layout::storage::LayoutSource::None => rsx! {
                                    span { class: "status-item", style: "color: var(--text-main); font-weight: 500;",
                                        "Layout: {def.name} ({def.vendor_id}:{def.product_id})"
                                    }
                                }
                            }
                        } else if let Some(dev) = selected_device.read().as_ref() {
                            span { class: "status-item", style: "color: #e5c07b;",
                                "Unconfigured Device ({dev.product_string})"
                            }
                        }
                    }
                }
            }
        }
        if let Some(warnings) = import_warnings.read().as_ref() {
            div { class: "modal-overlay",
                div { class: "modal-content",
                    h3 { "Import Completed with Warnings" }
                    p { "The backup was restored, but the following issues occurred:" }
                    div { class: "modal-warning-list",
                        for warning in warnings {
                            div { class: "modal-warning-item", "{warning}" }
                        }
                    }
                    div { style: "display: flex; justify-content: flex-end; margin-top: 20px;",
                        button {
                            class: "primary-btn",
                            onclick: move |_| {
                                import_warnings.set(None);
                            },
                            "Dismiss"
                        }
                    }
                }
            }
        }
        if let Some((mx, my)) = *mouse_pos.read() {
            if let Some((_l, _title, code)) = dragged_keycode.read().as_ref().cloned() {
                div {
                    class: "keycap dragging-preview",
                    style: format!(
                        "position: fixed; left: {}px; top: {}px; width: 50px; height: 50px; pointer-events: none; z-index: 9999; transform: translate(-50%, -50%); opacity: 0.6;",
                        mx, my
                    ),
                    if let Some(shift_sym) = crate::ui::keycodes::get_shift_symbol(code) {
                        div { class: "key-labels-dual",
                            span { class: "key-label-shift", "{shift_sym}" }
                            span { class: "key-label-primary", "{crate::ui::keycodes::get_keycode_label(code, via_def.read().as_ref())}" }
                        }
                    } else {
                        span { class: "key-label-center", "{crate::ui::keycodes::get_keycode_label(code, via_def.read().as_ref())}" }
                    }
                }
            }
        }
        if *show_any_modal.read() {
            div {
                class: "modal-overlay",
                onmousedown: move |e| {
                    e.stop_propagation();
                },
                div {
                    class: "modal-content",
                    style: "width: 500px; padding: 24px; border-radius: 8px; box-shadow: 0 12px 32px rgba(0,0,0,0.6);",
                    {
                        let text = any_custom_text.read().clone();
                        let sense_result = crate::ui::keycodes::parse_custom_keycode(&text, via_def.read().as_ref());
                        let is_valid = sense_result.is_ok();
                        let current_selected = selected_key.read().clone();

                        rsx! {
                            div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;",
                                h3 { style: "margin: 0; display: flex; align-items: center; gap: 8px; font-size: 18px;",
                                    span { style: "background: #7c4dff; color: white; padding: 3px 8px; border-radius: 4px; font-size: 11px; font-weight: 700; letter-spacing: 0.5px;", "ANY" }
                                    "Enter Custom Keycode"
                                }
                                button {
                                    style: "background: none; border: none; color: #888; font-size: 18px; cursor: pointer; padding: 0 4px;",
                                    onclick: move |_| show_any_modal.set(false),
                                    "✕"
                                }
                            }
                            
                            p { style: "color: #b0b0b0; font-size: 13px; margin-bottom: 16px; line-height: 1.5;",
                                "Enter any custom keycode in hex format (0x0000-0xFFFF) or standard QMK alias (e.g. KC_A, MO(1), LCTL(KC_C), LT(1, KC_SPC))."
                            }
                            
                            div { style: "margin-bottom: 16px;",
                                label { style: "display: block; font-size: 12px; font-weight: 600; color: #ccc; margin-bottom: 6px;", "Custom Keycode Expression:" }
                                input {
                                    type: "text",
                                    style: format!(
                                        "width: 100%; padding: 10px 14px; background: #181818; border: 1.5px solid {}; border-radius: 6px; color: #fff; font-family: monospace; font-size: 14px; outline: none; box-sizing: border-box;",
                                        if text.trim().is_empty() { "#444" } else if is_valid { "#4caf50" } else { "#f44336" }
                                    ),
                                    placeholder: "e.g. 0x0004, KC_A, MO(1), LCTL(KC_C)",
                                    value: "{text}",
                                    oninput: move |e| any_custom_text.set(e.value()),
                                }
                            }
                            
                            div {
                                style: format!(
                                    "padding: 14px; border-radius: 6px; margin-bottom: 20px; font-size: 13px; border: 1px solid {}; background: {}; transition: all 0.2s ease;",
                                    if text.trim().is_empty() { "rgba(255,255,255,0.1)" } else if is_valid { "rgba(76, 175, 80, 0.4)" } else { "rgba(244, 67, 54, 0.4)" },
                                    if text.trim().is_empty() { "#222" } else if is_valid { "rgba(76, 175, 80, 0.08)" } else { "rgba(244, 67, 54, 0.08)" }
                                ),
                                if text.trim().is_empty() {
                                    div { style: "color: #777; font-style: italic; font-size: 12px;", "Sense check: Start typing to evaluate keycode validity..." }
                                } else {
                                    match sense_result {
                                        Ok((_code, ref desc)) => rsx! {
                                            div { style: "display: flex; flex-direction: column; gap: 4px;",
                                                div { style: "color: #4caf50; font-weight: 600; display: flex; align-items: center; gap: 6px;",
                                                    span { style: "font-size: 14px;", "✓" }
                                                    "Sense Check Passed"
                                                }
                                                div { style: "font-family: monospace; color: #e0e0e0; font-size: 13px; margin-top: 2px;",
                                                    "Parsed: {desc}"
                                                }
                                            }
                                        },
                                        Err(ref err) => rsx! {
                                            div { style: "display: flex; flex-direction: column; gap: 4px;",
                                                div { style: "color: #f44336; font-weight: 600; display: flex; align-items: center; gap: 6px;",
                                                    span { style: "font-size: 14px;", "✗" }
                                                    "Sense Check Failed"
                                                }
                                                div { style: "color: #ffcdd2; font-size: 12px; margin-top: 2px; line-height: 1.4;",
                                                    "{err}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            
                            div { style: "display: flex; justify-content: space-between; align-items: center; gap: 12px;",
                                if let Some(ref k) = current_selected {
                                    div { style: "color: #aaa; font-size: 12px;",
                                        "Target key: "
                                        span { style: "color: var(--accent); font-weight: 600;", "Row {k.matrix_row}, Col {k.matrix_col}" }
                                    }
                                } else if let Some(first_k) = physical_keys.read().first() {
                                    div { style: "color: #aaa; font-size: 12px;",
                                        "Target key: "
                                        span { style: "color: var(--accent); font-weight: 600;", "Row {first_k.matrix_row}, Col {first_k.matrix_col} (Default)" }
                                    }
                                } else {
                                    div { style: "color: #777; font-size: 12px;", "Target: Active key" }
                                }
                                
                                div { style: "display: flex; gap: 10px;",
                                    button {
                                        class: "secondary-btn",
                                        onclick: move |_| show_any_modal.set(false),
                                        "Cancel"
                                    }
                                    button {
                                        class: if is_valid { "primary-btn" } else { "primary-btn disabled" },
                                        disabled: !is_valid,
                                        style: if !is_valid {
                                            "opacity: 0.4; cursor: not-allowed; background-color: #444444 !important; border: 1px solid #555555 !important;"
                                        } else {
                                            "opacity: 1.0; cursor: pointer; background-color: var(--accent) !important; border: none !important;"
                                        },
                                        onclick: move |_| {
                                            if let Ok((code, label_str)) = crate::ui::keycodes::parse_custom_keycode(&any_custom_text.read(), via_def.read().as_ref()) {
                                                let target_k = selected_key.read().as_ref().cloned().or_else(|| physical_keys.read().first().cloned());
                                                if let Some(k) = target_k {
                                                    show_any_modal.set(false);
                                                    let row = k.matrix_row as u8;
                                                    let col = k.matrix_col as u8;
                                                    let c = code;
                                                    let layer = *active_layer.read();

                                                    stored_keycodes.write().insert((layer, row, col), c);
                                                    layer_keycodes.write().insert((row, col), c);

                                                    if let Some(dev_info) = selected_device.read().as_ref() {
                                                        let path = dev_info.path.clone();
                                                        let mut selected_key_sig = selected_key.clone();
                                                        let physical_keys_sig = physical_keys.clone();
                                                        let mut layer_keycodes_sig = layer_keycodes.clone();
                                                        let mut status_sig = status.clone();
                                                        
                                                        spawn(async move {
                                                            #[cfg(not(target_arch = "wasm32"))]
                                                            if let Ok(api) = hidapi::HidApi::new() {
                                                                if let Ok(c_path) = std::ffi::CString::new(path) {
                                                                    if let Ok(device) = api.open_path(&c_path) {
                                                                        let via = crate::hid::via_protocol::ViaKeyboard::new(&device);
                                                                        match via.set_keycode(layer, row, col, c) {
                                                                            Ok(_) => {
                                                                                let _ = via.custom_save(0, 0);
                                                                                status_sig.set(format!("Saved custom keycode 0x{:04X} ({}) to {},{} in EEPROM", c, label_str, row, col));
                                                                                
                                                                                let p_keys = physical_keys_sig.read().clone();
                                                                                if let Some(pos) = p_keys.iter().position(|pk| pk.matrix_row == k.matrix_row && pk.matrix_col == k.matrix_col) {
                                                                                    let next_pos = pos + 1;
                                                                                    if next_pos < p_keys.len() {
                                                                                        selected_key_sig.set(Some(p_keys[next_pos].clone()));
                                                                                    }
                                                                                }
                                                                            },
                                                                            Err(e) => status_sig.set(format!("Error writing: {}", e)),
                                                                        }
                                                                    }
                                                                }
                                                            }

                                                            #[cfg(target_arch = "wasm32")]
                                                            {
                                                                let via = crate::hid::via_protocol::ViaKeyboard;
                                                                match via.set_keycode_async(layer, row, col, c).await {
                                                                    Ok(_) => {
                                                                        let _ = via.custom_save_async(0, 0).await;
                                                                        status_sig.set(format!("Saved custom keycode 0x{:04X} ({}) to {},{} in EEPROM", c, label_str, row, col));
                                                                        
                                                                        let p_keys = physical_keys_sig.read().clone();
                                                                        if let Some(pos) = p_keys.iter().position(|pk| pk.matrix_row == k.matrix_row && pk.matrix_col == k.matrix_col) {
                                                                            let next_pos = pos + 1;
                                                                            if next_pos < p_keys.len() {
                                                                                selected_key_sig.set(Some(p_keys[next_pos].clone()));
                                                                            }
                                                                        }
                                                                    },
                                                                    Err(e) => status_sig.set(format!("Error writing: {}", e)),
                                                                }
                                                            }
                                                        });
                                                    } else {
                                                        status.set(format!("Set custom keycode 0x{:04X} ({}) to {},{}", code, label_str, k.matrix_row, k.matrix_col));
                                                    }
                                                }
                                            }
                                        },
                                        "Confirm Code"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
