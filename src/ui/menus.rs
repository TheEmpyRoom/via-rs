use dioxus::prelude::*;
use crate::hid::manager::KeyboardInfo;
use serde_json::Value;

pub fn json_to_u64(val: &serde_json::Value) -> Option<u64> {
    if let Some(n) = val.as_u64() {
        Some(n)
    } else if let Some(f) = val.as_f64() {
        Some(f.round() as u64)
    } else if let Some(i) = val.as_i64() {
        Some(i as u64)
    } else if let Some(s) = val.as_str() {
        s.parse::<u64>().ok().or_else(|| s.parse::<f64>().ok().map(|f| f.round() as u64))
    } else {
        None
    }
}

pub fn json_to_f64(val: &serde_json::Value) -> Option<f64> {
    if let Some(f) = val.as_f64() {
        Some(f)
    } else if let Some(u) = val.as_u64() {
        Some(u as f64)
    } else if let Some(i) = val.as_i64() {
        Some(i as f64)
    } else if let Some(s) = val.as_str() {
        s.parse::<f64>().ok()
    } else {
        None
    }
}

fn extract_any_feature_value(resp: &[u8], start_idx: usize, min_val: f64, max_val: f64) -> f64 {
    if resp.len() <= start_idx {
        return min_val;
    }
    let b0 = resp[start_idx];
    let b1 = if resp.len() > start_idx + 1 { resp[start_idx + 1] } else { 0 };
    let b2 = if resp.len() > start_idx + 2 { resp[start_idx + 2] } else { 0 };
    let b3 = if resp.len() > start_idx + 3 { resp[start_idx + 3] } else { 0 };

    let v0 = b0 as f64;
    let v1 = b1 as f64;
    let min_u16 = min_val.round() as u16;
    let max_u16 = max_val.round() as u16;

    if v0 >= min_val && v0 <= max_val {
        if b1 == 0 || max_val <= 255.0 {
            return v0;
        }
    }

    let be16 = ((b0 as u16) << 8) | (b1 as u16);
    if be16 >= min_u16 && be16 <= max_u16 {
        return be16 as f64;
    }

    let le16 = ((b1 as u16) << 8) | (b0 as u16);
    if le16 >= min_u16 && le16 <= max_u16 {
        return le16 as f64;
    }

    if v1 >= min_val && v1 <= max_val {
        return v1;
    }

    let f_be = f32::from_be_bytes([b0, b1, b2, b3]) as f64;
    if f_be.is_finite() && f_be >= min_val && f_be <= max_val {
        return f_be;
    }

    let f_le = f32::from_le_bytes([b0, b1, b2, b3]) as f64;
    if f_le.is_finite() && f_le >= min_val && f_le <= max_val {
        return f_le;
    }

    v0.clamp(min_val, max_val)
}

#[component]
pub fn MenuNode(
    node: Value,
    selected_device: Signal<Option<KeyboardInfo>>,
    status: Signal<String>,
    level: u8,
    parent_id: String,
    expanded_nodes: Signal<std::collections::HashSet<String>>,
    features_version: Signal<u64>
) -> Element {
    let label = node.get("label").and_then(|l| l.as_str()).unwrap_or("").to_string();
    let ctype = node.get("type").and_then(|t| t.as_str()).unwrap_or("").to_string();
    let node_id = format!("{}/{}", parent_id, label);

    if !ctype.is_empty() {
        rsx! {
            MenuControl { control: node.clone(), selected_device, status, level, features_version }
        }
    } else if let Some(content) = node.get("content").and_then(|c| c.as_array()) {
        rsx! {
            div { class: "device-tree-children",
                if !label.is_empty() {
                    div {
                        class: "tree-item parent-node",
                        style: "padding-left: {level * 20}px;",
                        onclick: {
                            let id = node_id.clone();
                            move |_| {
                                let mut nodes = expanded_nodes.write();
                                if nodes.contains(&id) { nodes.remove(&id); } else { nodes.insert(id.clone()); }
                            }
                        },
                        span { class: if expanded_nodes.read().contains(&node_id) { "chevron expanded" } else { "chevron collapsed" }, "▶" }
                        span { class: "tree-label", "{label}" }
                    }
                }
                
                if label.is_empty() || expanded_nodes.read().contains(&node_id) {
                    for child in content.iter() {
                        MenuNode { 
                            node: child.clone(), 
                            selected_device, 
                            status, 
                            level: if label.is_empty() { level } else { level + 1 }, 
                            parent_id: node_id.clone(), 
                            expanded_nodes: expanded_nodes.clone(),
                            features_version
                        }
                    }
                }
            }
        }
    } else {
        rsx! { span {} }
    }
}

#[component]
pub fn MenuControl(
    control: Value,
    selected_device: Signal<Option<KeyboardInfo>>,
    status: Signal<String>,
    level: u8,
    features_version: Signal<u64>
) -> Element {
    let label = control.get("label").and_then(|l| l.as_str()).unwrap_or("Unknown").to_string();
    let ctype = control.get("type").and_then(|t| t.as_str()).unwrap_or("").to_string();
    
    // Parse mapping [command, channel/value_id, offset]
    let (cmd_id, val_or_ch, offset) = if let Some(content) = control.get("content").and_then(|c| c.as_array()) {
        match content.len() {
            1 => {
                let val_id = json_to_u64(&content[0]).unwrap_or(0) as u8;
                (3, val_id, 0)
            }
            2 => {
                let ch = json_to_u64(&content[0]).unwrap_or(0) as u8;
                let off = json_to_u64(&content[1]).unwrap_or(0) as u8;
                (7, ch, off)
            }
            _ => {
                let cmd = json_to_u64(&content[0]).unwrap_or(7) as u8;
                let ch = json_to_u64(&content[1]).unwrap_or(0) as u8;
                let off = json_to_u64(&content[2]).unwrap_or(0) as u8;
                (cmd, ch, off)
            }
        }
    } else {
        (7, 0, 0)
    };

    let mut current_val = use_signal(|| 0.0f64);
    let is_loading = use_signal(|| true);

    let dropdown_options: Vec<(String, f64)> = if let Some(options) = control.get("options").and_then(|o| o.as_array()) {
        options.iter().enumerate().filter_map(|(idx, opt)| {
            if let Some(arr) = opt.as_array() {
                if arr.len() >= 2 {
                    let lbl = arr[0].as_str().unwrap_or("").to_string();
                    let val = json_to_f64(&arr[1]).unwrap_or(idx as f64);
                    Some((lbl, val))
                } else if arr.len() == 1 {
                    let lbl = arr[0].as_str().unwrap_or("").to_string();
                    Some((lbl, idx as f64))
                } else {
                    None
                }
            } else if let Some(str_val) = opt.as_str() {
                Some((str_val.to_string(), idx as f64))
            } else if let Some(num_val) = json_to_f64(opt) {
                Some((format!("{}", num_val), num_val))
            } else {
                None
            }
        }).collect()
    } else {
        Vec::new()
    };

    let (mut min_val, mut max_val, mut step_val) = (0.0f64, 255.0f64, 1.0f64);
    if let Some(options) = control.get("options").and_then(|o| o.as_array()) {
        if ctype == "range" {
            if options.len() >= 2 {
                min_val = json_to_f64(&options[0]).unwrap_or(0.0);
                max_val = json_to_f64(&options[1]).unwrap_or(255.0);
            }
            if options.len() >= 3 {
                step_val = json_to_f64(&options[2]).unwrap_or(1.0);
            }
        } else if ctype == "dropdown" && !dropdown_options.is_empty() {
            min_val = dropdown_options.iter().map(|(_, v)| *v).fold(f64::INFINITY, f64::min);
            max_val = dropdown_options.iter().map(|(_, v)| *v).fold(f64::NEG_INFINITY, f64::max);
        }
    }

    let is_float_control = if let Some(options) = control.get("options").and_then(|o| o.as_array()) {
        if options.len() >= 3 {
            let step = json_to_f64(&options[2]).unwrap_or(1.0);
            step < 1.0 || step.fract() != 0.0
        } else if options.len() >= 2 {
            let min_f = json_to_f64(&options[0]).unwrap_or(0.0);
            let max_f = json_to_f64(&options[1]).unwrap_or(255.0);
            min_f.fract() != 0.0 || max_f.fract() != 0.0
        } else {
            false
        }
    } else {
        false
    };

    // Initial fetch
    let dev_sig = selected_device.clone();
    use_effect(move || {
        let _ = features_version.read();
        let _ = dev_sig.read();
        if let Some(dev_info) = dev_sig.read().as_ref() {
            let path = dev_info.path.clone();
            let mut val_sig = current_val.clone();
            let mut is_loading_sig = is_loading.clone();
            spawn(async move {
                is_loading_sig.set(true);
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let delay_ms = 120 + (val_or_ch as u64 * 40) + (offset as u64 * 30);
                    tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
                    if let Ok(api) = hidapi::HidApi::new() {
                        if let Ok(c_path) = std::ffi::CString::new(path) {
                            if let Ok(dev) = api.open_path(&c_path) {
                                let via = crate::hid::via_protocol::ViaKeyboard::new(&dev);
                                let mut val = if cmd_id == 2 || cmd_id == 3 {
                                    via.get_keyboard_value_raw(val_or_ch).ok().map(|resp| {
                                        extract_any_feature_value(&resp, 2, min_val, max_val)
                                    })
                                } else {
                                    via.get_custom_value_raw(val_or_ch, offset).ok().map(|resp| {
                                        extract_any_feature_value(&resp, 3, min_val, max_val)
                                    })
                                };
                                if val.is_none() {
                                    std::thread::sleep(std::time::Duration::from_millis(40));
                                    val = if cmd_id == 2 || cmd_id == 3 {
                                        via.get_keyboard_value_raw(val_or_ch).ok().map(|resp| {
                                            extract_any_feature_value(&resp, 2, min_val, max_val)
                                        })
                                    } else {
                                        via.get_custom_value_raw(val_or_ch, offset).ok().map(|resp| {
                                            extract_any_feature_value(&resp, 3, min_val, max_val)
                                        })
                                    };
                                }
                                val_sig.set(val.unwrap_or(min_val));
                            }
                        }
                    }
                }

                #[cfg(target_arch = "wasm32")]
                {
                    let delay_ms = 120 + (val_or_ch as u32 * 40) + (offset as u32 * 30);
                    gloo_timers::future::TimeoutFuture::new(delay_ms).await;
                    let via = crate::hid::via_protocol::ViaKeyboard;
                    let mut val = if cmd_id == 2 || cmd_id == 3 {
                        if let Ok(resp) = via.get_keyboard_value_raw_async(val_or_ch).await {
                            Some(extract_any_feature_value(&resp, 2, min_val, max_val))
                        } else {
                            None
                        }
                    } else {
                        if let Ok(resp) = via.get_custom_value_raw_async(val_or_ch, offset).await {
                            Some(extract_any_feature_value(&resp, 3, min_val, max_val))
                        } else {
                            None
                        }
                    };

                    if val.is_none() {
                        gloo_timers::future::TimeoutFuture::new(40).await;
                        val = if cmd_id == 2 || cmd_id == 3 {
                            if let Ok(resp) = via.get_keyboard_value_raw_async(val_or_ch).await {
                                Some(extract_any_feature_value(&resp, 2, min_val, max_val))
                            } else {
                                None
                            }
                        } else {
                            if let Ok(resp) = via.get_custom_value_raw_async(val_or_ch, offset).await {
                                Some(extract_any_feature_value(&resp, 3, min_val, max_val))
                            } else {
                                None
                            }
                        };
                    }
                    val_sig.set(val.unwrap_or(min_val));
                }
                is_loading_sig.set(false);
            });
        }
    });

    let mut status_sig = status.clone();
    let label_clone = label.clone();
    let on_change = move |new_val: f64| {
        current_val.set(new_val);
        if let Some(dev_info) = selected_device.read().as_ref() {
            let path = dev_info.path.clone();
            let lbl = label_clone.clone();
            spawn(async move {
                #[cfg(not(target_arch = "wasm32"))]
                if let Ok(api) = hidapi::HidApi::new() {
                    if let Ok(c_path) = std::ffi::CString::new(path) {
                        if let Ok(dev) = api.open_path(&c_path) {
                            let via = crate::hid::via_protocol::ViaKeyboard::new(&dev);
                            if cmd_id == 2 || cmd_id == 3 {
                                let u32_val = if is_float_control {
                                    u32::from_be_bytes((new_val as f32).to_be_bytes())
                                } else {
                                    new_val.round() as u32
                                };
                                let set_res = via.set_keyboard_value(val_or_ch, u32_val);
                                std::thread::sleep(std::time::Duration::from_millis(50));
                                let _ = via.custom_save(val_or_ch, 0);
                                if let Err(e) = set_res {
                                    status_sig.set(format!("Failed to set {}: {}", lbl, e));
                                } else {
                                    status_sig.set(format!("Saved {} = {} to EEPROM", lbl, new_val));
                                }
                            } else {
                                let custom_bytes: [u8; 2] = if is_float_control {
                                    if max_val <= 10.0 {
                                        [(new_val * 100.0).round() as u8, 0]
                                    } else {
                                        [new_val.round() as u8, 0]
                                    }
                                } else if max_val > 255.0 {
                                    (new_val.round() as u16).to_be_bytes()
                                } else {
                                    [new_val.round() as u8, 0]
                                };
                                let set_res = via.set_custom_value(val_or_ch, offset, custom_bytes);
                                std::thread::sleep(std::time::Duration::from_millis(50));
                                let save_res = via.custom_save(val_or_ch, offset);
                                if let Err(e) = set_res {
                                    status_sig.set(format!("Failed to set {}: {}", lbl, e));
                                } else if let Err(e) = save_res {
                                    status_sig.set(format!("Failed to save {} to EEPROM: {}", lbl, e));
                                } else {
                                    status_sig.set(format!("Saved {} = {} to EEPROM", lbl, new_val));
                                }
                            }
                        }
                    }
                }

                #[cfg(target_arch = "wasm32")]
                {
                    let via = crate::hid::via_protocol::ViaKeyboard;
                    if cmd_id == 2 || cmd_id == 3 {
                        let u32_val = if is_float_control {
                            u32::from_be_bytes((new_val as f32).to_be_bytes())
                        } else {
                            new_val.round() as u32
                        };
                        let set_res = via.set_keyboard_value_async(val_or_ch, u32_val).await;
                        gloo_timers::future::TimeoutFuture::new(50).await;
                        let _ = via.custom_save_async(val_or_ch, 0).await;
                        if let Err(e) = set_res {
                            status_sig.set(format!("Failed to set {}: {}", lbl, e));
                        } else {
                            status_sig.set(format!("Saved {} = {} to EEPROM", lbl, new_val));
                        }
                    } else {
                        let custom_bytes: [u8; 2] = if is_float_control {
                            if max_val <= 10.0 {
                                [(new_val * 100.0).round() as u8, 0]
                            } else {
                                [new_val.round() as u8, 0]
                            }
                        } else if max_val > 255.0 {
                            (new_val.round() as u16).to_be_bytes()
                        } else {
                            [new_val.round() as u8, 0]
                        };
                        let set_res = via.set_custom_value_async(val_or_ch, offset, custom_bytes).await;
                        gloo_timers::future::TimeoutFuture::new(50).await;
                        let save_res = via.custom_save_async(val_or_ch, offset).await;
                        if let Err(e) = set_res {
                            status_sig.set(format!("Failed to set {}: {}", lbl, e));
                        } else if let Err(e) = save_res {
                            status_sig.set(format!("Failed to save {} to EEPROM: {}", lbl, e));
                        } else {
                            status_sig.set(format!("Saved {} = {} to EEPROM", lbl, new_val));
                        }
                    }
                }
            });
        }
    };

    let on_change = std::rc::Rc::new(std::cell::RefCell::new(on_change));
    let on_change_toggle = on_change.clone();
    let on_change_dropdown = on_change.clone();
    let on_change_number = on_change.clone();
    let on_change_range = on_change.clone();

    let display_val_str = if is_float_control {
        format!("{:.2}", *current_val.read())
    } else {
        format!("{}", current_val.read().round() as i64)
    };

    rsx! {
        if ctype == "toggle" {
            div { class: "menu-control tree-item leaf-node", style: "display: flex; align-items: center; justify-content: space-between; padding: 6px 15px 6px {level * 20}px; height: auto; gap: 10px;",
                span { style: "font-size: 12px; color: var(--text-main); white-space: normal; line-height: 1.4; flex: 1; display: flex; align-items: center; gap: 6px;",
                    "{label}"
                    if *is_loading.read() {
                        span { class: "feature-loader", title: "Loading..." }
                    }
                }
                input {
                    r#type: "checkbox",
                    checked: *current_val.read() != 0.0,
                    style: "cursor: pointer; margin: 0;",
                    onchange: move |e| on_change_toggle.borrow_mut()(if e.value() == "true" { 1.0 } else { 0.0 }),
                }
            }
        } else if ctype == "dropdown" {
            div { class: "menu-control tree-item leaf-node", style: "display: flex; align-items: center; justify-content: space-between; padding: 6px 15px 6px {level * 20}px; height: auto; gap: 10px;",
                span { style: "font-size: 12px; color: var(--text-main); white-space: normal; line-height: 1.4; flex: 1; display: flex; align-items: center; gap: 6px;",
                    "{label}"
                    if *is_loading.read() {
                        span { class: "feature-loader", title: "Loading..." }
                    }
                }
                select {
                    style: "width: auto; min-width: 90px; text-align: right; background-color: var(--bg-dark); color: var(--text-bright); border: 1px solid var(--border); padding: 4px; border-radius: 4px; cursor: pointer;",
                    onchange: move |e| {
                        if let Ok(v) = e.value().parse::<f64>() {
                            on_change_dropdown.borrow_mut()(v);
                        }
                    },
                    for (opt_lbl, opt_val) in dropdown_options.iter() {
                        option {
                            value: "{opt_val}",
                            selected: (*current_val.read() - *opt_val).abs() < 0.001,
                            "{opt_lbl}"
                        }
                    }
                }
            }
        } else if ctype == "range" {
            div { class: "menu-control tree-item leaf-node", style: "display: flex; flex-direction: column; align-items: stretch; padding: 8px 15px 8px {level * 20}px; height: auto; gap: 6px;",
                span { style: "font-size: 12px; color: var(--text-main); white-space: normal; line-height: 1.4; width: 100%; display: flex; align-items: center; gap: 6px;",
                    "{label}"
                    if *is_loading.read() {
                        span { class: "feature-loader", title: "Loading..." }
                    }
                }
                div { style: "display: flex; align-items: center; justify-content: space-between; width: 100%; gap: 10px;",
                    input {
                        "type": "range",
                        min: "{min_val}",
                        max: "{max_val}",
                        step: "{step_val}",
                        value: "{current_val.read()}",
                        style: "flex: 1; margin: 0; min-width: 0;",
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<f64>() {
                                current_val.set(v);
                            }
                        },
                        onchange: move |e| {
                            if let Ok(v) = e.value().parse::<f64>() {
                                on_change_range.borrow_mut()(v);
                            }
                        }
                    }
                    input {
                        r#type: "number",
                        min: "{min_val}",
                        max: "{max_val}",
                        step: "{step_val}",
                        value: "{display_val_str}",
                        style: "font-family: monospace; font-size: 11px; color: var(--text-bright); background: var(--bg-dark); border: 1px solid var(--border); border-radius: 4px; width: 55px; padding: 2px 4px; text-align: right; outline: none; flex-shrink: 0;",
                        onchange: move |e| {
                            if let Ok(v) = e.value().parse::<f64>() {
                                let clamped = v.clamp(min_val, max_val);
                                on_change_number.borrow_mut()(clamped);
                            }
                        }
                    }
                }
            }
        } else {
            div { class: "menu-control tree-item leaf-node", style: "display: flex; align-items: center; padding: 6px 15px 6px {level * 20}px; height: auto;",
                span { style: "color: #ff5555;", "Unsupported control: {ctype}" }
            }
        }
    }
}
