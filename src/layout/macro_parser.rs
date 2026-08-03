pub fn get_keycode_name(code: u8) -> Option<&'static str> {
    match code {
        0x04 => Some("KC_A"), 0x05 => Some("KC_B"), 0x06 => Some("KC_C"), 0x07 => Some("KC_D"),
        0x08 => Some("KC_E"), 0x09 => Some("KC_F"), 0x0A => Some("KC_G"), 0x0B => Some("KC_H"),
        0x0C => Some("KC_I"), 0x0D => Some("KC_J"), 0x0E => Some("KC_K"), 0x0F => Some("KC_L"),
        0x10 => Some("KC_M"), 0x11 => Some("KC_N"), 0x12 => Some("KC_O"), 0x13 => Some("KC_P"),
        0x14 => Some("KC_Q"), 0x15 => Some("KC_R"), 0x16 => Some("KC_S"), 0x17 => Some("KC_T"),
        0x18 => Some("KC_U"), 0x19 => Some("KC_V"), 0x1A => Some("KC_W"), 0x1B => Some("KC_X"),
        0x1C => Some("KC_Y"), 0x1D => Some("KC_Z"),
        0x1E => Some("KC_1"), 0x1F => Some("KC_2"), 0x20 => Some("KC_3"), 0x21 => Some("KC_4"),
        0x22 => Some("KC_5"), 0x23 => Some("KC_6"), 0x24 => Some("KC_7"), 0x25 => Some("KC_8"),
        0x26 => Some("KC_9"), 0x27 => Some("KC_0"),
        0x28 => Some("KC_ENT"), 0x29 => Some("KC_ESC"), 0x2A => Some("KC_BSPC"), 0x2B => Some("KC_TAB"),
        0x2C => Some("KC_SPC"), 0x2D => Some("KC_MINS"), 0x2E => Some("KC_EQL"), 0x2F => Some("KC_LBRC"),
        0x30 => Some("KC_RBRC"), 0x31 => Some("KC_BSLS"), 0x33 => Some("KC_SCLN"), 0x34 => Some("KC_QUOT"),
        0x35 => Some("KC_GRV"), 0x36 => Some("KC_COMM"), 0x37 => Some("KC_DOT"), 0x38 => Some("KC_SLSH"),
        0x39 => Some("KC_CAPS"),
        0x3A => Some("KC_F1"), 0x3B => Some("KC_F2"), 0x3C => Some("KC_F3"), 0x3D => Some("KC_F4"),
        0x3E => Some("KC_F5"), 0x3F => Some("KC_F6"), 0x40 => Some("KC_F7"), 0x41 => Some("KC_F8"),
        0x42 => Some("KC_F9"), 0x43 => Some("KC_F10"), 0x44 => Some("KC_F11"), 0x45 => Some("KC_F12"),
        0x46 => Some("KC_PSCR"), 0x47 => Some("KC_SLCK"), 0x48 => Some("KC_PAUS"),
        0x49 => Some("KC_INS"), 0x4A => Some("KC_HOME"), 0x4B => Some("KC_PGUP"), 0x4C => Some("KC_DEL"),
        0x4D => Some("KC_END"), 0x4E => Some("KC_PGDN"),
        0x4F => Some("KC_RGHT"), 0x50 => Some("KC_LEFT"), 0x51 => Some("KC_DOWN"), 0x52 => Some("KC_UP"),
        0x53 => Some("KC_NLCK"), 0x54 => Some("KC_PSLS"), 0x55 => Some("KC_PAST"), 0x56 => Some("KC_PMNS"), 0x57 => Some("KC_PPLS"), 0x58 => Some("KC_PENT"),
        0x59 => Some("KC_P1"), 0x5A => Some("KC_P2"), 0x5B => Some("KC_P3"), 0x5C => Some("KC_P4"), 0x5D => Some("KC_P5"), 0x5E => Some("KC_P6"), 0x5F => Some("KC_P7"), 0x60 => Some("KC_P8"), 0x61 => Some("KC_P9"), 0x62 => Some("KC_P0"), 0x63 => Some("KC_PDOT"),
        0xE0 => Some("KC_LCTL"), 0xE1 => Some("KC_LSFT"), 0xE2 => Some("KC_LALT"), 0xE3 => Some("KC_LGUI"),
        0xE4 => Some("KC_RCTL"), 0xE5 => Some("KC_RSFT"), 0xE6 => Some("KC_RALT"), 0xE7 => Some("KC_RGUI"),
        _ => None
    }
}

pub fn get_name_keycode(name: &str) -> Option<u8> {
    match name {
        "KC_A" => Some(0x04), "KC_B" => Some(0x05), "KC_C" => Some(0x06), "KC_D" => Some(0x07),
        "KC_E" => Some(0x08), "KC_F" => Some(0x09), "KC_G" => Some(0x0A), "KC_H" => Some(0x0B),
        "KC_I" => Some(0x0C), "KC_J" => Some(0x0D), "KC_K" => Some(0x0E), "KC_L" => Some(0x0F),
        "KC_M" => Some(0x10), "KC_N" => Some(0x11), "KC_O" => Some(0x12), "KC_P" => Some(0x13),
        "KC_Q" => Some(0x14), "KC_R" => Some(0x15), "KC_S" => Some(0x16), "KC_T" => Some(0x17),
        "KC_U" => Some(0x18), "KC_V" => Some(0x19), "KC_W" => Some(0x1A), "KC_X" => Some(0x1B),
        "KC_Y" => Some(0x1C), "KC_Z" => Some(0x1D),
        "KC_1" => Some(0x1E), "KC_2" => Some(0x1F), "KC_3" => Some(0x20), "KC_4" => Some(0x21),
        "KC_5" => Some(0x22), "KC_6" => Some(0x23), "KC_7" => Some(0x24), "KC_8" => Some(0x25),
        "KC_9" => Some(0x26), "KC_0" => Some(0x27),
        "KC_ENT" | "KC_ENTER" => Some(0x28), "KC_ESC" | "KC_ESCAPE" => Some(0x29), "KC_BSPC" | "KC_BACKSPACE" => Some(0x2A), "KC_TAB" => Some(0x2B),
        "KC_SPC" | "KC_SPACE" => Some(0x2C), "KC_MINS" | "KC_MINUS" => Some(0x2D), "KC_EQL" | "KC_EQUAL" => Some(0x2E), "KC_LBRC" => Some(0x2F),
        "KC_RBRC" => Some(0x30), "KC_BSLS" | "KC_BSLASH" => Some(0x31), "KC_SCLN" | "KC_SCOLON" => Some(0x33), "KC_QUOT" | "KC_QUOTE" => Some(0x34),
        "KC_GRV" | "KC_GRAVE" => Some(0x35), "KC_COMM" | "KC_COMMA" => Some(0x36), "KC_DOT" => Some(0x37), "KC_SLSH" | "KC_SLASH" => Some(0x38),
        "KC_CAPS" | "KC_CAPSLOCK" => Some(0x39),
        "KC_F1" => Some(0x3A), "KC_F2" => Some(0x3B), "KC_F3" => Some(0x3C), "KC_F4" => Some(0x3D),
        "KC_F5" => Some(0x3E), "KC_F6" => Some(0x3F), "KC_F7" => Some(0x40), "KC_F8" => Some(0x41),
        "KC_F9" => Some(0x42), "KC_F10" => Some(0x43), "KC_F11" => Some(0x44), "KC_F12" => Some(0x45),
        "KC_PSCR" => Some(0x46), "KC_SLCK" => Some(0x47), "KC_PAUS" => Some(0x48),
        "KC_INS" | "KC_INSERT" => Some(0x49), "KC_HOME" => Some(0x4A), "KC_PGUP" => Some(0x4B), "KC_DEL" | "KC_DELETE" => Some(0x4C),
        "KC_END" => Some(0x4D), "KC_PGDN" => Some(0x4E),
        "KC_RGHT" | "KC_RIGHT" => Some(0x4F), "KC_LEFT" => Some(0x50), "KC_DOWN" => Some(0x51), "KC_UP" => Some(0x52),
        "KC_NLCK" | "KC_NUMLOCK" => Some(0x53), "KC_PSLS" | "KC_PSLASH" | "KC_P/" => Some(0x54), "KC_PAST" | "KC_PASTERISK" | "KC_P*" => Some(0x55), "KC_PMNS" | "KC_PMINUS" | "KC_P-" => Some(0x56), "KC_PPLS" | "KC_PPLUS" | "KC_P+" => Some(0x57), "KC_PENT" | "KC_PENTER" => Some(0x58),
        "KC_P1" => Some(0x59), "KC_P2" => Some(0x5A), "KC_P3" => Some(0x5B), "KC_P4" => Some(0x5C), "KC_P5" => Some(0x5D), "KC_P6" => Some(0x5E), "KC_P7" => Some(0x5F), "KC_P8" => Some(0x60), "KC_P9" => Some(0x61), "KC_P0" => Some(0x62), "KC_PDOT" | "KC_P." => Some(0x63),
        "KC_LCTL" | "KC_LCTRL" => Some(0xE0), "KC_LSFT" | "KC_LSHIFT" => Some(0xE1), "KC_LALT" => Some(0xE2), "KC_LGUI" => Some(0xE3),
        "KC_RCTL" | "KC_RCTRL" => Some(0xE4), "KC_RSFT" | "KC_RSHIFT" => Some(0xE5), "KC_RALT" => Some(0xE6), "KC_RGUI" => Some(0xE7),
        _ => None
    }
}

/// Splits the entire raw macro buffer into a vector of individual macro buffers.
pub fn split_macro_buffer(buffer: &[u8], count: u8, _max_size: u16) -> Vec<Vec<u8>> {
    let mut macros = Vec::new();
    if buffer.is_empty() || count == 0 {
        return macros;
    }

    let mut current_macro = Vec::new();
    let mut macro_id = 0;
    
    for &b in buffer {
        if macro_id >= count {
            break;
        }
        if b == 0 {
            macros.push(current_macro.clone());
            current_macro.clear();
            macro_id += 1;
        } else {
            current_macro.push(b);
        }
    }
    
    // Pad out the remaining macros if the buffer didn't contain enough
    while (macros.len() as u8) < count {
        macros.push(Vec::new());
    }
    
    macros
}

/// Reconstructs the macro buffer from individual macros, padding to max_size
pub fn build_macro_buffer(macros: &[Vec<u8>], count: u8, max_size: u16) -> Vec<u8> {
    let mut buffer = Vec::new();
    
    for i in 0..count as usize {
        let mac: &[u8] = if i < macros.len() { &macros[i] } else { &[] };
        buffer.extend_from_slice(mac);
        buffer.push(0); // null terminator
    }
    
    // Pad to max_size
    if buffer.len() < max_size as usize {
        buffer.resize(max_size as usize, 0);
    } else {
        // Truncate if somehow larger (should warn user)
        buffer.truncate(max_size as usize);
    }
    
    buffer
}

/// Decodes a single macro byte array into a human-readable VIA string
pub fn decode_macro(buffer: &[u8], protocol_version: u16) -> String {
    let mut decoded = String::new();
    let mut i = 0;
    
    // In VIA Protocol 11+, the key actions are prefixed with 0x01. If version is 0 (unknown), default to 11.
    let eff_version = if protocol_version == 0 { 11 } else { protocol_version };
    let uses_prefix = eff_version >= 11;
    
    while i < buffer.len() {
        let b = buffer[i];
        if b == 0 {
            break; // terminator
        }
        
        let mut process_action = |action_type: u8, keycode: u8| {
            if let Some(name) = get_keycode_name(keycode) {
                match action_type {
                    1 => decoded.push_str(&format!("{{{}}}", name)), // SS_TAP
                    2 => decoded.push_str(&format!("{{+{}}}", name)), // SS_DOWN
                    3 => decoded.push_str(&format!("{{-{}}}", name)), // SS_UP
                    _ => decoded.push_str(&format!("{{UnknownAction {}}}", action_type)),
                }
            } else {
                match action_type {
                    1 => decoded.push_str(&format!("{{0x{:02X}}}", keycode)),
                    2 => decoded.push_str(&format!("{{+0x{:02X}}}", keycode)),
                    3 => decoded.push_str(&format!("{{-0x{:02X}}}", keycode)),
                    _ => decoded.push_str(&format!("{{Unknown {} 0x{:02X}}}", action_type, keycode)),
                }
            }
        };

        if uses_prefix {
            if b == 1 { // KeyActionPrefix
                i += 1;
                if i < buffer.len() {
                    let action = buffer[i];
                    if action == 4 { // Delay
                        i += 1;
                        let mut delay_str = String::new();
                        while i < buffer.len() && buffer[i] != 124 && buffer[i] != 0 { // 124 is DelayTerminator '|'
                            delay_str.push(buffer[i] as char);
                            i += 1;
                        }
                        decoded.push_str(&format!("{{Delay {}}}", delay_str));
                    } else {
                        i += 1;
                        if i < buffer.len() {
                            let keycode = buffer[i];
                            process_action(action, keycode);
                        }
                    }
                }
            } else {
                // Character stream
                if b >= 0x20 && b <= 0x7E {
                    decoded.push(b as char);
                } else {
                    decoded.push_str(&format!("\\x{:02X}", b));
                }
            }
        } else {
            // Protocol < 11
            if b == 1 || b == 2 || b == 3 {
                i += 1;
                if i < buffer.len() {
                    let keycode = buffer[i];
                    process_action(b, keycode);
                }
            } else if b == 4 { // Delay (not really supported in v10 but just in case)
                i += 1;
                if i < buffer.len() {
                    let delay = buffer[i];
                    decoded.push_str(&format!("{{Delay {}}}", delay));
                }
            } else {
                // Character stream
                if b >= 0x20 && b <= 0x7E {
                    decoded.push(b as char);
                } else {
                    decoded.push_str(&format!("\\x{:02X}", b));
                }
            }
        }
        
        i += 1;
    }
    
    decoded
}

/// Encodes a human-readable VIA string into a macro byte array
pub fn encode_macro(text: &str, protocol_version: u16) -> Vec<u8> {
    let mut bytes = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    
    let uses_prefix = protocol_version >= 11;
    
    let push_action = |b: &mut Vec<u8>, action_type: u8, keycode: u8| {
        if uses_prefix {
            b.push(1); // KeyActionPrefix
        }
        b.push(action_type);
        b.push(keycode);
    };

    while i < chars.len() {
        if chars[i] == '{' {
            let mut j = i + 1;
            let mut content = String::new();
            while j < chars.len() && chars[j] != '}' {
                content.push(chars[j]);
                j += 1;
            }
            if j < chars.len() {
                // Parse content
                if content.starts_with('+') {
                    let kc = &content[1..];
                    let formatted_kc = if kc.starts_with("KC_") { kc.to_string() } else { format!("KC_{}", kc) };
                    if let Some(code) = get_name_keycode(&formatted_kc).or_else(|| get_name_keycode(kc)) {
                        push_action(&mut bytes, 2, code); // SS_DOWN
                    } else if let Ok(hex) = u8::from_str_radix(kc.trim_start_matches("0x"), 16) {
                        push_action(&mut bytes, 2, hex);
                    }
                } else if content.starts_with('-') {
                    let kc = &content[1..];
                    let formatted_kc = if kc.starts_with("KC_") { kc.to_string() } else { format!("KC_{}", kc) };
                    if let Some(code) = get_name_keycode(&formatted_kc).or_else(|| get_name_keycode(kc)) {
                        push_action(&mut bytes, 3, code); // SS_UP
                    } else if let Ok(hex) = u8::from_str_radix(kc.trim_start_matches("0x"), 16) {
                        push_action(&mut bytes, 3, hex);
                    }
                } else if content.starts_with("Delay ") {
                    let delay_str = &content[6..];
                    if uses_prefix {
                        bytes.push(1); // KeyActionPrefix
                        bytes.push(4); // SS_DELAY
                        for c in delay_str.chars() {
                            bytes.push(c as u8);
                        }
                        bytes.push(124); // DelayTerminator '|'
                    } else {
                        if let Ok(delay) = delay_str.parse::<u8>() {
                            bytes.push(4); // SS_DELAY
                            bytes.push(delay);
                        }
                    }
                } else {
                    let formatted_kc = if content.starts_with("KC_") { content.clone() } else { format!("KC_{}", content) };
                    if let Some(code) = get_name_keycode(&formatted_kc).or_else(|| get_name_keycode(&content)) {
                        push_action(&mut bytes, 1, code); // SS_TAP
                    } else if let Ok(hex) = u8::from_str_radix(content.trim_start_matches("0x"), 16) {
                        push_action(&mut bytes, 1, hex);
                    }
                }
                i = j + 1;
                continue;
            }
        } else if chars[i] == '\\' && i + 3 < chars.len() && chars[i+1] == 'x' {
            let hex: String = chars[i+2..i+4].iter().collect();
            if let Ok(val) = u8::from_str_radix(&hex, 16) {
                bytes.push(val);
                i += 4;
                continue;
            }
        } else if chars[i] == '\\' && i + 5 < chars.len() && chars[i+1] == 'u' {
            let hex_str: String = chars[i+2..i+6].iter().collect();
            if hex_str.chars().all(|c| c.is_ascii_hexdigit()) {
                push_action(&mut bytes, 2, 0xE2); // SS_DOWN KC_LALT
                push_action(&mut bytes, 1, 0x57); // SS_TAP KC_PPLS
                for ch in hex_str.chars() {
                    let kc = match ch.to_ascii_uppercase() {
                        '0' => 0x27, '1' => 0x1E, '2' => 0x1F, '3' => 0x20, '4' => 0x21,
                        '5' => 0x22, '6' => 0x23, '7' => 0x24, '8' => 0x25, '9' => 0x26,
                        'A' => 0x04, 'B' => 0x05, 'C' => 0x06, 'D' => 0x07, 'E' => 0x08, 'F' => 0x09,
                        _ => continue,
                    };
                    push_action(&mut bytes, 1, kc); // SS_TAP
                }
                push_action(&mut bytes, 3, 0xE2); // SS_UP KC_LALT
                i += 6;
                continue;
            }
        }
        
        // Literal character
        bytes.push(chars[i] as u8);
        i += 1;
    }
    
    bytes
}
