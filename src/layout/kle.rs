#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalKey {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub r: f32,
    pub rx: f32,
    pub ry: f32,
    pub matrix_row: i32,
    pub matrix_col: i32,
    pub label: String,
    pub default_code: u16,
}

pub fn parse_kle(keymap: &serde_json::Value) -> Vec<PhysicalKey> {
    let mut keys = Vec::new();
    
    let mut current_x = 0.0;
    let mut current_y = 0.0;
    
    let mut current_rx = 0.0;
    let mut current_ry = 0.0;
    let mut current_r = 0.0;

    let keymap_array = if let Some(arr) = keymap.as_array() {
        Some(arr)
    } else if let Some(arr) = keymap.get("keymap").and_then(|v| v.as_array()) {
        Some(arr)
    } else if let Some(arr) = keymap.get("layout").and_then(|v| v.as_array()) {
        Some(arr)
    } else {
        None
    };
    
    if let Some(rows) = keymap_array {
        for row in rows {
            if let Some(items) = row.as_array() {
                let mut current_w = 1.0;
                let mut current_h = 1.0;
                
                for item in items {
                    if let Some(obj) = item.as_object() {
                        if let Some(r) = obj.get("r").and_then(|v| v.as_f64()) {
                            current_r = r as f32;
                        }
                        let mut origin_changed = false;
                        if let Some(rx) = obj.get("rx").and_then(|v| v.as_f64()) {
                            current_rx = rx as f32;
                            origin_changed = true;
                        }
                        if let Some(ry) = obj.get("ry").and_then(|v| v.as_f64()) {
                            current_ry = ry as f32;
                            origin_changed = true;
                        }
                        if origin_changed {
                            current_x = current_rx;
                            current_y = current_ry;
                        }
                        if let Some(x) = obj.get("x").and_then(|v| v.as_f64()) {
                            current_x += x as f32;
                        }
                        if let Some(y) = obj.get("y").and_then(|v| v.as_f64()) {
                            current_y += y as f32;
                        }
                        if let Some(w) = obj.get("w").and_then(|v| v.as_f64()) {
                            current_w = w as f32;
                        }
                        if let Some(h) = obj.get("h").and_then(|v| v.as_f64()) {
                            current_h = h as f32;
                        }
                    } else if let Some(s) = item.as_str() {
                        let mut matrix_row = -1;
                        let mut matrix_col = -1;
                        let mut label = String::new();
                        let mut default_code = 0u16;
                        
                        let lines: Vec<&str> = s.split('\n').collect();
                        for line in &lines {
                            let trimmed = line.trim();
                            let parts: Vec<&str> = trimmed.split(',').collect();
                            if parts.len() == 2 {
                                if let (Ok(r), Ok(c)) = (parts[0].trim().parse::<i32>(), parts[1].trim().parse::<i32>()) {
                                    matrix_row = r;
                                    matrix_col = c;
                                    continue;
                                }
                            }
                            if !trimmed.is_empty() && label.is_empty() {
                                label = trimmed.to_string();
                            }
                            if default_code == 0 && !trimmed.is_empty() {
                                if let Ok((code, _)) = crate::ui::keycodes::parse_custom_keycode(trimmed, None) {
                                    default_code = code;
                                }
                            }
                        }

                        if label.is_empty() && matrix_row >= 0 && matrix_col >= 0 {
                            label = format!("{},{}", matrix_row, matrix_col);
                        }
                        
                        keys.push(PhysicalKey {
                            x: current_x,
                            y: current_y,
                            w: current_w,
                            h: current_h,
                            r: current_r,
                            rx: current_rx,
                            ry: current_ry,
                            matrix_row,
                            matrix_col,
                            label,
                            default_code,
                        });
                        
                        current_x += current_w;
                        current_w = 1.0;
                        current_h = 1.0;
                    }
                }
            }
            current_y += 1.0;
            // A basic simplification: reset x to rx at the end of each row.
            current_x = current_rx;
        }
    }
    
    keys
}
