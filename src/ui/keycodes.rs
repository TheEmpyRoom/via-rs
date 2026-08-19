use crate::layout::parser::ViaDefinition;

pub const BASIC_KEYCODES: &[(&str, &str, u16)] = &[
    ("▽", "Transparent", 0x0001), ("", "None", 0x0000),
    ("A", "A", 0x04), ("B", "B", 0x05), ("C", "C", 0x06), ("D", "D", 0x07),
    ("E", "E", 0x08), ("F", "F", 0x09), ("G", "G", 0x0A), ("H", "H", 0x0B),
    ("I", "I", 0x0C), ("J", "J", 0x0D), ("K", "K", 0x0E), ("L", "L", 0x0F),
    ("M", "M", 0x10), ("N", "N", 0x11), ("O", "O", 0x12), ("P", "P", 0x13),
    ("Q", "Q", 0x14), ("R", "R", 0x15), ("S", "S", 0x16), ("T", "T", 0x17),
    ("U", "U", 0x18), ("V", "V", 0x19), ("W", "W", 0x1A), ("X", "X", 0x1B),
    ("Y", "Y", 0x1C), ("Z", "Z", 0x1D),
    ("1", "1 and !", 0x1E), ("2", "2 and @", 0x1F), ("3", "3 and #", 0x20), 
    ("4", "4 and $", 0x21), ("5", "5 and %", 0x22), ("6", "6 and ^", 0x23), 
    ("7", "7 and &", 0x24), ("8", "8 and *", 0x25), ("9", "9 and (", 0x26), 
    ("0", "0 and )", 0x27),
    ("Ent", "Enter", 0x28), ("Esc", "Escape", 0x29), ("Bspc", "Backspace", 0x2A), 
    ("Tab", "Tab", 0x2B), ("Spc", "Spacebar", 0x2C), ("-", "Minus", 0x2D), 
    ("=", "Equal", 0x2E), ("[", "Left Bracket", 0x2F), ("]", "Right Bracket", 0x30), 
    ("\\", "Backslash", 0x31), ("#", "Non-US Hash", 0x32), (";", "Semicolon", 0x33), 
    ("'", "Quote", 0x34), ("`", "Grave", 0x35), (",", "Comma", 0x36), 
    (".", "Period", 0x37), ("/", "Slash", 0x38), ("Caps", "Caps Lock", 0x39),
    ("F1", "F1", 0x3A), ("F2", "F2", 0x3B), ("F3", "F3", 0x3C), ("F4", "F4", 0x3D),
    ("F5", "F5", 0x3E), ("F6", "F6", 0x3F), ("F7", "F7", 0x40), ("F8", "F8", 0x41),
    ("F9", "F9", 0x42), ("F10", "F10", 0x43), ("F11", "F11", 0x44), ("F12", "F12", 0x45),
    ("PrtSc", "Print Screen", 0x46), ("ScrLk", "Scroll Lock", 0x47), ("Pause", "Pause", 0x48),
    ("Ins", "Insert", 0x49), ("Home", "Home", 0x4A), ("PgUp", "Page Up", 0x4B),
    ("Del", "Delete", 0x4C), ("End", "End", 0x4D), ("PgDn", "Page Down", 0x4E),
    ("→", "Right Arrow", 0x4F), ("←", "Left Arrow", 0x50), ("↓", "Down Arrow", 0x51), ("↑", "Up Arrow", 0x52),
    ("NumLk", "Num Lock", 0x53), ("P/", "Pad Slash", 0x54), ("P*", "Pad Asterisk", 0x55),
    ("P-", "Pad Minus", 0x56), ("P+", "Pad Plus", 0x57), ("PEnt", "Pad Enter", 0x58),
    ("P1", "Pad 1", 0x59), ("P2", "Pad 2", 0x5A), ("P3", "Pad 3", 0x5B), ("P4", "Pad 4", 0x5C),
    ("P5", "Pad 5", 0x5D), ("P6", "Pad 6", 0x5E), ("P7", "Pad 7", 0x5F), ("P8", "Pad 8", 0x60),
    ("P9", "Pad 9", 0x61), ("P0", "Pad 0", 0x62), ("P.", "Pad Period", 0x63),
    ("NonUS\\", "Non-US Backslash", 0x64), ("App", "Application", 0x65), ("Power", "Power", 0x66),
    ("P=", "Pad Equal", 0x67),
    ("F13", "F13", 0x68), ("F14", "F14", 0x69), ("F15", "F15", 0x6A), ("F16", "F16", 0x6B),
    ("F17", "F17", 0x6C), ("F18", "F18", 0x6D), ("F19", "F19", 0x6E), ("F20", "F20", 0x6F),
    ("F21", "F21", 0x70), ("F22", "F22", 0x71), ("F23", "F23", 0x72), ("F24", "F24", 0x73),
];

pub const MODIFIER_KEYCODES: &[(&str, &str, u16)] = &[
    ("LCtrl", "Left Control", 0xE0), ("LShift", "Left Shift", 0xE1), 
    ("LAlt", "Left Alt", 0xE2), ("LGui", "Left GUI", 0xE3),
    ("RCtrl", "Right Control", 0xE4), ("RShift", "Right Shift", 0xE5), 
    ("RAlt", "Right Alt", 0xE6), ("RGui", "Right GUI", 0xE7),
];

pub const MEDIA_KEYCODES: &[(&str, &str, u16)] = &[
    ("Mute", "Mute", 0xA8), ("Vol+", "Volume Up", 0xA9), ("Vol-", "Volume Down", 0xAA),
    ("Next", "Next Track", 0xAB), ("Prev", "Previous Track", 0xAC), 
    ("Stop", "Stop", 0xAD), ("Play", "Play/Pause", 0xAE),
    ("FFwd", "Fast Forward", 0xBB), ("Rwnd", "Rewind", 0xBC),
];

pub const MOUSE_KEYCODES: &[(&str, &str, u16)] = &[
    ("MsUp", "Mouse Up", 0xCD), ("MsDn", "Mouse Down", 0xCE), 
    ("MsL", "Mouse Left", 0xCF), ("MsR", "Mouse Right", 0xD0),
    ("Btn1", "Mouse Button 1", 0xD1), ("Btn2", "Mouse Button 2", 0xD2), 
    ("Btn3", "Mouse Button 3", 0xD3), ("Btn4", "Mouse Button 4", 0xD4),
    ("Btn5", "Mouse Button 5", 0xD5), ("WhlU", "Mouse Wheel Up", 0xD9), 
    ("WhlD", "Mouse Wheel Down", 0xDA), ("WhlL", "Mouse Wheel Left", 0xDB),
    ("WhlR", "Mouse Wheel Right", 0xDC), ("Acl0", "Mouse Accel 0", 0xDD),
    ("Acl1", "Mouse Accel 1", 0xDE), ("Acl2", "Mouse Accel 2", 0xDF),
];

pub const LIGHTING_KEYCODES: &[(&str, &str, u16)] = &[
    ("BL_TOG", "Backlight Toggle", 0x7800), ("BL_STEP", "Backlight Step", 0x7801),
    ("BL_ON", "Backlight On", 0x7802), ("BL_OFF", "Backlight Off", 0x7803),
    ("BL_INC", "Backlight Increase", 0x7804), ("BL_DEC", "Backlight Decrease", 0x7805),
    ("BL_BRTG", "Backlight Breathing", 0x7806),
    ("RGB_TOG", "RGB Toggle", 0x7820), ("RGB_MOD", "RGB Mode", 0x7821),
    ("RGB_RMOD", "RGB Reverse Mode", 0x7822), ("RGB_HUI", "RGB Hue Increase", 0x7823),
    ("RGB_HUD", "RGB Hue Decrease", 0x7824), ("RGB_SAI", "RGB Saturation Increase", 0x7825),
    ("RGB_SAD", "RGB Saturation Decrease", 0x7826), ("RGB_VAI", "RGB Value Increase", 0x7827),
    ("RGB_VAD", "RGB Value Decrease", 0x7828), ("RGB_SPI", "RGB Speed Increase", 0x7829),
    ("RGB_SPD", "RGB Speed Decrease", 0x782A), ("RGB_M_P", "RGB Mode: Plain", 0x782B),
    ("RGB_M_B", "RGB Mode: Breathe", 0x782C), ("RGB_M_R", "RGB Mode: Rainbow", 0x782D),
    ("RGB_M_SW", "RGB Mode: Swirl", 0x782E), ("RGB_M_SN", "RGB Mode: Snake", 0x782F),
    ("RGB_M_K", "RGB Mode: Knight", 0x7830), ("RGB_M_X", "RGB Mode: Christmas", 0x7831),
    ("RGB_M_G", "RGB Mode: Gradient", 0x7832),
];

pub const SPECIAL_KEYCODES: &[(&str, &str, u16)] = &[
    ("Boot", "Bootloader", 0x7C00), ("Debug", "Debug", 0x7C01),
    ("Clear", "Clear EEPROM", 0x7C02), ("Make", "Make", 0x7C03),
    ("Any", "Any", 0x7FFF),
];

pub const MACRO_KEYCODES: &[(&str, &str, u16)] = &[
    ("M0", "Macro 0", 0x7700), ("M1", "Macro 1", 0x7701), ("M2", "Macro 2", 0x7702),
    ("M3", "Macro 3", 0x7703), ("M4", "Macro 4", 0x7704), ("M5", "Macro 5", 0x7705),
    ("M6", "Macro 6", 0x7706), ("M7", "Macro 7", 0x7707), ("M8", "Macro 8", 0x7708),
    ("M9", "Macro 9", 0x7709), ("M10", "Macro 10", 0x770A), ("M11", "Macro 11", 0x770B),
    ("M12", "Macro 12", 0x770C), ("M13", "Macro 13", 0x770D), ("M14", "Macro 14", 0x770E),
    ("M15", "Macro 15", 0x770F),
];

pub const LAYER_KEYCODES: &[(&str, &str, u16)] = &[
    ("MO(0)", "Momentary Layer 0", 0x5220), ("MO(1)", "Momentary Layer 1", 0x5221),
    ("MO(2)", "Momentary Layer 2", 0x5222), ("MO(3)", "Momentary Layer 3", 0x5223),
    ("MO(4)", "Momentary Layer 4", 0x5224), ("MO(5)", "Momentary Layer 5", 0x5225),
    ("MO(6)", "Momentary Layer 6", 0x5226), ("MO(7)", "Momentary Layer 7", 0x5227),
    ("TG(0)", "Toggle Layer 0", 0x5260), ("TG(1)", "Toggle Layer 1", 0x5261),
    ("TG(2)", "Toggle Layer 2", 0x5262), ("TG(3)", "Toggle Layer 3", 0x5263),
    ("TO(0)", "To Layer 0", 0x5200), ("TO(1)", "To Layer 1", 0x5201),
    ("TO(2)", "To Layer 2", 0x5202), ("TO(3)", "To Layer 3", 0x5203),
    ("OSL(0)", "One Shot Layer 0", 0x5280), ("OSL(1)", "One Shot Layer 1", 0x5281),
    ("OSL(2)", "One Shot Layer 2", 0x5282), ("OSL(3)", "One Shot Layer 3", 0x5283),
];

fn get_shifted_label(kc: u16) -> Option<String> {
    match kc {
        0x1E => Some("!".to_string()),
        0x1F => Some("@".to_string()),
        0x20 => Some("#".to_string()),
        0x21 => Some("$".to_string()),
        0x22 => Some("%".to_string()),
        0x23 => Some("^".to_string()),
        0x24 => Some("&".to_string()),
        0x25 => Some("*".to_string()),
        0x26 => Some("(".to_string()),
        0x27 => Some(")".to_string()),
        0x2D => Some("_".to_string()),
        0x2E => Some("+".to_string()),
        0x2F => Some("{".to_string()),
        0x30 => Some("}".to_string()),
        0x31 => Some("|".to_string()),
        0x33 => Some(":".to_string()),
        0x34 => Some("\"".to_string()),
        0x35 => Some("~".to_string()),
        0x36 => Some("<".to_string()),
        0x37 => Some(">".to_string()),
        0x38 => Some("?".to_string()),
        _ => None,
    }
}

fn get_basic_label(code: u16) -> Option<String> {
    for &(label, _, c) in BASIC_KEYCODES.iter() {
        if c == code { return Some(label.to_string()); }
    }
    for &(label, _, c) in MODIFIER_KEYCODES.iter() {
        if c == code { return Some(label.to_string()); }
    }
    for &(label, _, c) in MEDIA_KEYCODES.iter() {
        if c == code { return Some(label.to_string()); }
    }
    for &(label, _, c) in MOUSE_KEYCODES.iter() {
        if c == code { return Some(label.to_string()); }
    }
    for &(label, _, c) in LIGHTING_KEYCODES.iter() {
        if c == code { return Some(label.to_string()); }
    }
    for &(label, _, c) in SPECIAL_KEYCODES.iter() {
        if c == code { return Some(label.to_string()); }
    }
    for &(label, _, c) in MACRO_KEYCODES.iter() {
        if c == code { return Some(label.to_string()); }
    }
    None
}

pub fn get_keycode_label(code: u16, via_def: Option<&ViaDefinition>) -> String {
    if let Some(label) = get_basic_label(code) {
        return label;
    }

    // MODS
    if code >= 0x0100 && code <= 0x1FFF {
        let mods = (code >> 8) & 0x1F;
        let kc = code & 0xFF;
        
        let m = mods & 0x0F;
        
        if m == 0x02 {
            if let Some(shifted) = get_shifted_label(kc) {
                return shifted;
            }
        }
        
        let mut mod_strs = Vec::new();
        if m & 0x01 != 0 { mod_strs.push("C"); }
        if m & 0x02 != 0 { mod_strs.push("S"); }
        if m & 0x04 != 0 { mod_strs.push("A"); }
        if m & 0x08 != 0 { mod_strs.push("G"); }
        
        let kc_label = get_basic_label(kc).unwrap_or(format!("0x{:02X}", kc));
        if mod_strs.is_empty() {
            return kc_label;
        } else {
            return format!("{}({})", mod_strs.join(""), kc_label);
        }
    }
    
    // Mod-Tap
    if code >= 0x2000 && code <= 0x3FFF {
        let kc = code & 0xFF;
        let kc_label = get_basic_label(kc).unwrap_or(format!("0x{:02X}", kc));
        return format!("MT({})", kc_label);
    }
    
    // Layer-Tap
    if code >= 0x4000 && code <= 0x4FFF {
        let layer = (code >> 8) & 0x0F;
        let kc = code & 0xFF;
        let kc_label = get_basic_label(kc).unwrap_or(format!("0x{:02X}", kc));
        return format!("LT({},{})", layer, kc_label);
    }

    // Momentary layer
    if code >= 0x5220 && code <= 0x523F {
        return format!("MO({})", code & 0x1F);
    }
    
    // Toggle layer
    if code >= 0x5260 && code <= 0x527F {
        return format!("TG({})", code & 0x1F);
    }
    
    // TO(layer)
    if code >= 0x5200 && code <= 0x521F {
        return format!("TO({})", code & 0x1F);
    }
    
    // OSL(layer)
    if code >= 0x5280 && code <= 0x529F {
        return format!("OSL({})", code & 0x1F);
    }
    
    // Custom VIA keycodes
    if code >= 0x7E00 && code <= 0x7E3F {
        if let Some(def) = via_def {
            let idx = (code - 0x7E00) as usize;
            if idx < def.custom_keycodes.len() {
                return def.custom_keycodes[idx].short_name.clone();
            }
        }
        return format!("USER{:02}", code - 0x7E00);
    }
    if code >= 0x5C00 && code <= 0x5CFF {
        if let Some(def) = via_def {
            let idx = (code - 0x5C00) as usize;
            if idx < def.custom_keycodes.len() {
                return def.custom_keycodes[idx].short_name.clone();
            }
        }
        return format!("USER{:02}", code - 0x5C00);
    }
    if code >= 0x7E40 && code <= 0x7E7F {
        if let Some(def) = via_def {
            let idx = (code - 0x7E40) as usize;
            if idx < def.custom_keycodes.len() {
                return def.custom_keycodes[idx].short_name.clone();
            }
        }
        return format!("USER{:02}", code - 0x7E40);
    }

    format!("0x{:04X}", code)
}

pub fn format_keycode_for_any_input(code: u16, via_def: Option<&ViaDefinition>) -> String {
    if code == 0 {
        return "".to_string();
    }

    // 1. Basic keycodes in macro_parser (KC_A, KC_1, KC_LCTL, KC_ENT, KC_SPC, etc.)
    if (code & 0xFF00) == 0 {
        if let Some(qmk_name) = crate::layout::macro_parser::get_keycode_name(code as u8) {
            return qmk_name.to_string();
        }
    }

    // 2. Layer functions: MO(layer), TG(layer), TO(layer), OSL(layer), LT(layer, kc)
    if code >= 0x4000 && code <= 0x4FFF {
        let layer = (code >> 8) & 0x0F;
        let kc = code & 0xFF;
        let inner_qmk = format_keycode_for_any_input(kc, via_def);
        return format!("LT({}, {})", layer, if inner_qmk.is_empty() { "KC_NO" } else { &inner_qmk });
    }
    if code >= 0x5220 && code <= 0x523F {
        return format!("MO({})", code & 0x1F);
    }
    if code >= 0x5260 && code <= 0x527F {
        return format!("TG({})", code & 0x1F);
    }
    if code >= 0x5200 && code <= 0x521F {
        return format!("TO({})", code & 0x1F);
    }
    if code >= 0x5280 && code <= 0x529F {
        return format!("OSL({})", code & 0x1F);
    }

    // 3. Modifier combos: LCTL(kc), LSFT(kc), LALT(kc), LGUI(kc), RCTL(kc), RSFT(kc), RALT(kc), RGUI(kc)
    let mod_masks = [
        (0x0100, "LCTL"),
        (0x0200, "LSFT"),
        (0x0400, "LALT"),
        (0x0800, "LGUI"),
        (0x1100, "RCTL"),
        (0x1200, "RSFT"),
        (0x1400, "RALT"),
        (0x1800, "RGUI"),
        (0x0500, "LCA"),
        (0x0700, "MEH"),
        (0x0F00, "HYPR"),
    ];
    for (mask, prefix) in mod_masks {
        if (code & 0xFF00) == mask {
            let inner_kc = code & 0x00FF;
            let inner_qmk = format_keycode_for_any_input(inner_kc, via_def);
            return format!("{}({})", prefix, if inner_qmk.is_empty() { "KC_NO" } else { &inner_qmk });
        }
    }

    // 4. Custom VIA keycodes: USER00, USER01... or short_name
    if code >= 0x7E00 && code <= 0x7E3F {
        if let Some(def) = via_def {
            let idx = (code - 0x7E00) as usize;
            if idx < def.custom_keycodes.len() {
                return def.custom_keycodes[idx].short_name.clone();
            }
        }
        return format!("USER{:02}", code - 0x7E00);
    }
    if code >= 0x5C00 && code <= 0x5CFF {
        if let Some(def) = via_def {
            let idx = (code - 0x5C00) as usize;
            if idx < def.custom_keycodes.len() {
                return def.custom_keycodes[idx].short_name.clone();
            }
        }
        return format!("USER{:02}", code - 0x5C00);
    }
    if code >= 0x7E40 && code <= 0x7E7F {
        if let Some(def) = via_def {
            let idx = (code - 0x7E40) as usize;
            if idx < def.custom_keycodes.len() {
                return def.custom_keycodes[idx].short_name.clone();
            }
        }
        return format!("USER{:02}", code - 0x7E40);
    }

    // 5. Special QMK keycodes
    match code {
        0x7C00 => return "QK_BOOT".to_string(),
        0x7C01 => return "DEBUG".to_string(),
        0x7C02 => return "EE_CLR".to_string(),
        0x0000 => return "KC_NO".to_string(),
        0x0001 => return "KC_TRNS".to_string(),
        _ => {}
    }

    // 6. Lookups in tables (Lighting, Special, etc.)
    let check_tables: &[&[(&str, &str, u16)]] = &[
        LIGHTING_KEYCODES,
        SPECIAL_KEYCODES,
        BASIC_KEYCODES,
        MODIFIER_KEYCODES,
        MEDIA_KEYCODES,
        MOUSE_KEYCODES,
    ];
    for table in check_tables {
        for &(label, _title, c) in table.iter() {
            if c == code {
                if label.starts_with("KC_") || label.starts_with("RGB_") || label.starts_with("BL_") {
                    return label.to_string();
                }
                return format!("KC_{}", label.to_uppercase().replace(" ", "_"));
            }
        }
    }

    // 7. Fallback to 0x{:04X} hex format
    format!("0x{:04X}", code)
}

pub fn get_shift_symbol(code: u16) -> Option<&'static str> {
    match code {
        0x1E => Some("!"),
        0x1F => Some("@"),
        0x20 => Some("#"),
        0x21 => Some("$"),
        0x22 => Some("%"),
        0x23 => Some("^"),
        0x24 => Some("&"),
        0x25 => Some("*"),
        0x26 => Some("("),
        0x27 => Some(")"),
        0x2D => Some("_"),
        0x2E => Some("+"),
        0x2F => Some("{"),
        0x30 => Some("}"),
        0x31 => Some("|"),
        0x33 => Some(":"),
        0x34 => Some("\""),
        0x35 => Some("~"),
        0x36 => Some("<"),
        0x37 => Some(">"),
        0x38 => Some("?"),
        _ => None,
    }
}

pub fn parse_custom_keycode(input: &str, via_def: Option<&ViaDefinition>) -> Result<(u16, String), String> {
    let s = input.trim();
    if s.is_empty() {
        return Err("Enter a hex keycode (e.g. 0x0004) or QMK alias (e.g. KC_A, MO(1), LCTL(KC_C))".to_string());
    }

    // 1. Hex parsing with 0x / 0X prefix (e.g. 0x0004, 0x5C01)
    if let Some(hex_str) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        if hex_str.is_empty() || hex_str.len() > 4 {
            return Err("Hex keycode must be 1 to 4 hex digits (0x0000 to 0xFFFF)".to_string());
        }
        match u16::from_str_radix(hex_str, 16) {
            Ok(code) => {
                let label = get_keycode_label(code, via_def);
                return Ok((code, format!("Hex 0x{:04X} -> {}", code, label)));
            }
            Err(_) => return Err("Invalid hex string. Expected digits 0-9, A-F".to_string()),
        }
    }

    let upper = s.to_uppercase();

    // 2. Layer functions: MO(layer), TG(layer), TO(layer), OSL(layer)
    if let Some(inner) = extract_fn_arg(&upper, "MO") {
        if let Ok(layer) = inner.parse::<u8>() {
            if layer <= 31 {
                let code = 0x5220 + layer as u16;
                return Ok((code, format!("MO({}) - Momentary Layer {}", layer, layer)));
            }
        }
        return Err("MO layer index must be between 0 and 31 (e.g. MO(1))".to_string());
    }
    if let Some(inner) = extract_fn_arg(&upper, "TG") {
        if let Ok(layer) = inner.parse::<u8>() {
            if layer <= 31 {
                let code = 0x5260 + layer as u16;
                return Ok((code, format!("TG({}) - Toggle Layer {}", layer, layer)));
            }
        }
        return Err("TG layer index must be between 0 and 31 (e.g. TG(1))".to_string());
    }
    if let Some(inner) = extract_fn_arg(&upper, "TO") {
        if let Ok(layer) = inner.parse::<u8>() {
            if layer <= 31 {
                let code = 0x5200 + layer as u16;
                return Ok((code, format!("TO({}) - Switch To Layer {}", layer, layer)));
            }
        }
        return Err("TO layer index must be between 0 and 31 (e.g. TO(0))".to_string());
    }
    if let Some(inner) = extract_fn_arg(&upper, "OSL") {
        if let Ok(layer) = inner.parse::<u8>() {
            if layer <= 31 {
                let code = 0x5280 + layer as u16;
                return Ok((code, format!("OSL({}) - One Shot Layer {}", layer, layer)));
            }
        }
        return Err("OSL layer index must be between 0 and 31 (e.g. OSL(1))".to_string());
    }

    // 3. Layer-Tap: LT(layer, kc)
    if let Some(inner) = extract_fn_arg(&upper, "LT") {
        if let Some((l_str, kc_str)) = inner.split_once(',') {
            if let Ok(layer) = l_str.trim().parse::<u8>() {
                if layer <= 15 {
                    match parse_custom_keycode(kc_str.trim(), via_def) {
                        Ok((inner_kc, inner_label)) => {
                            let code = 0x4000 | ((layer as u16 & 0x0F) << 8) | (inner_kc & 0xFF);
                            return Ok((code, format!("LT({}, {}) - Layer-Tap", layer, inner_label)));
                        }
                        Err(e) => return Err(format!("Invalid keycode inside LT: {}", e)),
                    }
                }
            }
            return Err("LT layer index must be 0 to 15 (e.g. LT(1, KC_SPC))".to_string());
        }
    }

    // 4. Modifier Mod-Key combinations: LCTL(kc), LSFT(kc)...
    let mods = [
        ("LCTL", 0x0100), ("C", 0x0100), ("KC_LCTL", 0x0100), ("KC_LCTRL", 0x0100),
        ("LSFT", 0x0200), ("S", 0x0200), ("KC_LSFT", 0x0200), ("KC_LSHIFT", 0x0200),
        ("LALT", 0x0400), ("A", 0x0400), ("KC_LALT", 0x0400),
        ("LGUI", 0x0800), ("G", 0x0800), ("KC_LGUI", 0x0800), ("CMD", 0x0800), ("WIN", 0x0800),
        ("RCTL", 0x1100), ("KC_RCTL", 0x1100),
        ("RSFT", 0x1200), ("KC_RSFT", 0x1200),
        ("RALT", 0x1400), ("ALGR", 0x1400), ("KC_RALT", 0x1400),
        ("RGUI", 0x1800), ("KC_RGUI", 0x1800),
        ("LCA", 0x0500), ("MEH", 0x0700), ("HYPR", 0x0F00),
    ];
    for (prefix, mask) in mods {
        if let Some(inner) = extract_fn_arg(&upper, prefix) {
            match parse_custom_keycode(inner, via_def) {
                Ok((inner_kc, _)) => {
                    let code = mask | (inner_kc & 0xFF);
                    let label = get_keycode_label(code, via_def);
                    return Ok((code, format!("{}({})", prefix, label)));
                }
                Err(e) => return Err(format!("Invalid inner keycode for {}: {}", prefix, e)),
            }
        }
    }

    // 5. Lookups in QMK macro parser names (KC_A, KC_ENTER, KC_SPACE, etc.)
    if let Some(b) = crate::layout::macro_parser::get_name_keycode(&upper) {
        let code = b as u16;
        let label = get_keycode_label(code, via_def);
        return Ok((code, format!("{} (0x{:04X})", label, code)));
    }

    // 6. Lookups across keycode tables in keycodes.rs
    if let Some(code) = find_keycode_by_alias(&upper, via_def) {
        let label = get_keycode_label(code, via_def);
        return Ok((code, format!("{} (0x{:04X})", label, code)));
    }

    // 7. Pure hex digits: if 4-digit hex like 5C01 or decimal integer
    if s.len() == 4 && s.chars().all(|c| c.is_ascii_hexdigit()) {
        if let Ok(code) = u16::from_str_radix(s, 16) {
            let label = get_keycode_label(code, via_def);
            return Ok((code, format!("Hex 0x{:04X} -> {}", code, label)));
        }
    }
    if s.chars().all(|c| c.is_ascii_digit()) {
        if let Ok(code) = s.parse::<u16>() {
            let label = get_keycode_label(code, via_def);
            return Ok((code, format!("Decimal {} (0x{:04X}) -> {}", code, code, label)));
        }
    }

    let upper = s.to_uppercase();

    // 3. Layer functions: MO(layer), TG(layer), TO(layer), OSL(layer)
    if let Some(inner) = extract_fn_arg(&upper, "MO") {
        if let Ok(layer) = inner.parse::<u8>() {
            if layer <= 31 {
                let code = 0x5220 + layer as u16;
                return Ok((code, format!("MO({}) - Momentary Layer {}", layer, layer)));
            }
        }
        return Err("MO layer index must be between 0 and 31 (e.g. MO(1))".to_string());
    }
    if let Some(inner) = extract_fn_arg(&upper, "TG") {
        if let Ok(layer) = inner.parse::<u8>() {
            if layer <= 31 {
                let code = 0x5260 + layer as u16;
                return Ok((code, format!("TG({}) - Toggle Layer {}", layer, layer)));
            }
        }
        return Err("TG layer index must be between 0 and 31 (e.g. TG(1))".to_string());
    }
    if let Some(inner) = extract_fn_arg(&upper, "TO") {
        if let Ok(layer) = inner.parse::<u8>() {
            if layer <= 31 {
                let code = 0x5200 + layer as u16;
                return Ok((code, format!("TO({}) - Switch To Layer {}", layer, layer)));
            }
        }
        return Err("TO layer index must be between 0 and 31 (e.g. TO(0))".to_string());
    }
    if let Some(inner) = extract_fn_arg(&upper, "OSL") {
        if let Ok(layer) = inner.parse::<u8>() {
            if layer <= 31 {
                let code = 0x5280 + layer as u16;
                return Ok((code, format!("OSL({}) - One Shot Layer {}", layer, layer)));
            }
        }
        return Err("OSL layer index must be between 0 and 31 (e.g. OSL(1))".to_string());
    }

    // 4. Layer-Tap: LT(layer, kc)
    if let Some(inner) = extract_fn_arg(&upper, "LT") {
        if let Some((l_str, kc_str)) = inner.split_once(',') {
            if let Ok(layer) = l_str.trim().parse::<u8>() {
                if layer <= 15 {
                    match parse_custom_keycode(kc_str.trim(), via_def) {
                        Ok((inner_kc, inner_label)) => {
                            let code = 0x4000 | ((layer as u16 & 0x0F) << 8) | (inner_kc & 0xFF);
                            return Ok((code, format!("LT({}, {}) - Layer-Tap", layer, inner_label)));
                        }
                        Err(e) => return Err(format!("Invalid keycode inside LT: {}", e)),
                    }
                }
            }
            return Err("LT layer index must be 0 to 15 (e.g. LT(1, KC_SPC))".to_string());
        }
    }

    // 5. Modifier Mod-Key combinations: LCTL(kc), LSFT(kc), LALT(kc), LGUI(kc), RCTL(kc), RSFT(kc), RALT(kc), RGUI(kc), C(kc), S(kc), A(kc), G(kc), LCA(kc), MEH(kc), HYPR(kc)
    let mods = [
        ("LCTL", 0x0100), ("C", 0x0100), ("KC_LCTL", 0x0100), ("KC_LCTRL", 0x0100),
        ("LSFT", 0x0200), ("S", 0x0200), ("KC_LSFT", 0x0200), ("KC_LSHIFT", 0x0200),
        ("LALT", 0x0400), ("A", 0x0400), ("KC_LALT", 0x0400),
        ("LGUI", 0x0800), ("G", 0x0800), ("KC_LGUI", 0x0800), ("CMD", 0x0800), ("WIN", 0x0800),
        ("RCTL", 0x1100), ("KC_RCTL", 0x1100),
        ("RSFT", 0x1200), ("KC_RSFT", 0x1200),
        ("RALT", 0x1400), ("ALGR", 0x1400), ("KC_RALT", 0x1400),
        ("RGUI", 0x1800), ("KC_RGUI", 0x1800),
        ("LCA", 0x0500), ("MEH", 0x0700), ("HYPR", 0x0F00),
    ];
    for (prefix, mask) in mods {
        if let Some(inner) = extract_fn_arg(&upper, prefix) {
            match parse_custom_keycode(inner, via_def) {
                Ok((inner_kc, _)) => {
                    let code = mask | (inner_kc & 0xFF);
                    let label = get_keycode_label(code, via_def);
                    return Ok((code, format!("{}({})", prefix, label)));
                }
                Err(e) => return Err(format!("Invalid inner keycode for {}: {}", prefix, e)),
            }
        }
    }

    // 6. Lookups in QMK macro parser names (e.g. KC_A, KC_ENTER, etc.)
    if let Some(b) = crate::layout::macro_parser::get_name_keycode(&upper) {
        let code = b as u16;
        let label = get_keycode_label(code, via_def);
        return Ok((code, format!("{} (0x{:04X})", label, code)));
    }

    // 7. Lookups across keycode tables in keycodes.rs
    if let Some(code) = find_keycode_by_alias(&upper, via_def) {
        let label = get_keycode_label(code, via_def);
        return Ok((code, format!("{} (0x{:04X})", label, code)));
    }

    Err(format!("Unknown keycode format '{}'. Use hex (e.g. 0x0004), layer function (e.g. MO(1)), or QMK alias (e.g. KC_A, LCTL(KC_C))", s))
}

fn extract_fn_arg<'a>(input: &'a str, fn_name: &str) -> Option<&'a str> {
    let prefix = format!("{}(", fn_name);
    if input.starts_with(&prefix) && input.ends_with(')') {
        Some(&input[prefix.len()..input.len() - 1])
    } else {
        None
    }
}

fn find_keycode_by_alias(upper: &str, via_def: Option<&ViaDefinition>) -> Option<u16> {
    let check_tables: &[&[(&str, &str, u16)]] = &[
        BASIC_KEYCODES,
        MODIFIER_KEYCODES,
        MEDIA_KEYCODES,
        MOUSE_KEYCODES,
        LIGHTING_KEYCODES,
        SPECIAL_KEYCODES,
        MACRO_KEYCODES,
        LAYER_KEYCODES,
    ];

    for table in check_tables {
        for &(label, title, code) in table.iter() {
            if label.to_uppercase() == upper || title.to_uppercase() == upper {
                return Some(code);
            }
            let kc_label = format!("KC_{}", label.to_uppercase());
            let kc_title = format!("KC_{}", title.to_uppercase().replace(" ", "_"));
            if kc_label == upper || kc_title == upper {
                return Some(code);
            }
        }
    }

    match upper {
        "RESET" | "QK_BOOT" | "BOOTLOADER" => return Some(0x7C00),
        "DEBUG" => return Some(0x7C01),
        "EE_CLR" | "CLEAR_EEPROM" => return Some(0x7C02),
        "KC_NO" | "NONE" => return Some(0x0000),
        "KC_TRNS" | "TRANSPARENT" => return Some(0x0001),
        _ => {}
    }

    if upper.starts_with("USER") && upper.len() >= 6 {
        if let Ok(idx) = upper[4..].parse::<u16>() {
            if idx <= 31 {
                return Some(0x7E00 + idx);
            }
        }
    }

    if let Some(def) = via_def {
        for (idx, custom) in def.custom_keycodes.iter().enumerate() {
            if custom.short_name.to_uppercase() == upper || custom.title.to_uppercase() == upper {
                return Some(0x7E00 + idx as u16);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex() {
        assert_eq!(parse_custom_keycode("0x0004", None).unwrap().0, 0x0004);
        assert_eq!(parse_custom_keycode("0x5C01", None).unwrap().0, 0x5C01);
        assert_eq!(parse_custom_keycode("0x5220", None).unwrap().0, 0x5220);
        assert_eq!(parse_custom_keycode("5C01", None).unwrap().0, 0x5C01);
        assert!(parse_custom_keycode("0x10000", None).is_err());
        assert!(parse_custom_keycode("0xXYZ", None).is_err());
    }

    #[test]
    fn test_parse_layer_functions() {
        assert_eq!(parse_custom_keycode("MO(1)", None).unwrap().0, 0x5221);
        assert_eq!(parse_custom_keycode("TG(2)", None).unwrap().0, 0x5262);
        assert_eq!(parse_custom_keycode("TO(0)", None).unwrap().0, 0x5200);
        assert_eq!(parse_custom_keycode("OSL(3)", None).unwrap().0, 0x5283);
        assert!(parse_custom_keycode("MO(35)", None).is_err());
    }

    #[test]
    fn test_parse_modifiers() {
        assert_eq!(parse_custom_keycode("LCTL(KC_C)", None).unwrap().0, 0x0106);
        assert_eq!(parse_custom_keycode("C(KC_C)", None).unwrap().0, 0x0106);
        assert_eq!(parse_custom_keycode("LSFT(KC_A)", None).unwrap().0, 0x0204);
        assert_eq!(parse_custom_keycode("LALT(KC_TAB)", None).unwrap().0, 0x042B);
    }

    #[test]
    fn test_parse_aliases() {
        assert_eq!(parse_custom_keycode("KC_A", None).unwrap().0, 0x0004);
        assert_eq!(parse_custom_keycode("KC_ENTER", None).unwrap().0, 0x0028);
        assert_eq!(parse_custom_keycode("KC_TRNS", None).unwrap().0, 0x0001);
        assert_eq!(parse_custom_keycode("RESET", None).unwrap().0, 0x7C00);
        assert_eq!(parse_custom_keycode("USER01", None).unwrap().0, 0x5C01);
    }

    #[test]
    fn test_parse_invalid() {
        assert!(parse_custom_keycode("INVALID_CODE_123", None).is_err());
        assert!(parse_custom_keycode("", None).is_err());
    }
}


