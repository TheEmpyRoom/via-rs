use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ChainAction {
    Tap(String),
    Down(String),
    Up(String),
    Delay(u32),
    Text(String),
    AltCode(String),
    UCode(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChainItem {
    pub id: usize,
    pub action: ChainAction,
}

fn numpad_to_digit(kc: &str) -> Option<char> {
    match kc {
        "KC_P0" | "0x62" | "P0" => Some('0'),
        "KC_P1" | "0x59" | "P1" => Some('1'),
        "KC_P2" | "0x5A" | "P2" => Some('2'),
        "KC_P3" | "0x5B" | "P3" => Some('3'),
        "KC_P4" | "0x5C" | "P4" => Some('4'),
        "KC_P5" | "0x5D" | "P5" => Some('5'),
        "KC_P6" | "0x5E" | "P6" => Some('6'),
        "KC_P7" | "0x5F" | "P7" => Some('7'),
        "KC_P8" | "0x60" | "P8" => Some('8'),
        "KC_P9" | "0x61" | "P9" => Some('9'),
        _ => None,
    }
}

fn hex_keycode_to_char(kc: &str) -> Option<char> {
    match kc {
        "KC_0" | "0x27" | "KC_P0" | "0x62" | "P0" => Some('0'),
        "KC_1" | "0x1E" | "KC_P1" | "0x59" | "P1" => Some('1'),
        "KC_2" | "0x1F" | "KC_P2" | "0x5A" | "P2" => Some('2'),
        "KC_3" | "0x20" | "KC_P3" | "0x5B" | "P3" => Some('3'),
        "KC_4" | "0x21" | "KC_P4" | "0x5C" | "P4" => Some('4'),
        "KC_5" | "0x22" | "KC_P5" | "0x5D" | "P5" => Some('5'),
        "KC_6" | "0x23" | "KC_P6" | "0x5E" | "P6" => Some('6'),
        "KC_7" | "0x24" | "KC_P7" | "0x5F" | "P7" => Some('7'),
        "KC_8" | "0x25" | "KC_P8" | "0x60" | "P8" => Some('8'),
        "KC_9" | "0x26" | "KC_P9" | "0x61" | "P9" => Some('9'),
        "KC_A" | "0x04" => Some('A'),
        "KC_B" | "0x05" => Some('B'),
        "KC_C" | "0x06" => Some('C'),
        "KC_D" | "0x07" => Some('D'),
        "KC_E" | "0x08" => Some('E'),
        "KC_F" | "0x09" => Some('F'),
        _ => None,
    }
}

pub fn parse_macro_chain(text: &str) -> Vec<ChainItem> {
    let mut chain = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    let mut current_text = String::new();

    let flush_text = |chain: &mut Vec<ChainItem>, text_buf: &mut String| {
        if !text_buf.is_empty() {
            chain.push(ChainItem {
                id: chain.len(),
                action: ChainAction::Text(text_buf.clone()),
            });
            text_buf.clear();
        }
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
                flush_text(&mut chain, &mut current_text);
                
                if content.starts_with('+') {
                    chain.push(ChainItem {
                        id: chain.len(),
                        action: ChainAction::Down(content[1..].to_string()),
                    });
                } else if content.starts_with('-') {
                    chain.push(ChainItem {
                        id: chain.len(),
                        action: ChainAction::Up(content[1..].to_string()),
                    });
                } else if content.starts_with("Delay ") {
                    let ms = content[6..].parse::<u32>().unwrap_or(100);
                    chain.push(ChainItem {
                        id: chain.len(),
                        action: ChainAction::Delay(ms),
                    });
                } else {
                    chain.push(ChainItem {
                        id: chain.len(),
                        action: ChainAction::Tap(content.clone()),
                    });
                }
                i = j + 1;
                continue;
            }
        }
        
        current_text.push(chars[i]);
        i += 1;
    }
    flush_text(&mut chain, &mut current_text);

    // Condense Alt Numpad codes and U-Codes ({+KC_LALT}{KC_PPLS}...{-KC_LALT})
    let mut condensed = Vec::new();
    let mut idx = 0;
    while idx < chain.len() {
        let is_alt_start = match &chain[idx].action {
            ChainAction::Down(kc) => kc == "KC_LALT" || kc == "0xE2" || kc == "LALT",
            _ => false,
        };

        if is_alt_start {
            let mut j = idx + 1;
            if j < chain.len() {
                let is_ucode_start = match &chain[j].action {
                    ChainAction::Tap(kc) => kc == "KC_PPLS" || kc == "KC_P+" || kc == "0x57" || kc == "PPLS" || kc == "P+",
                    _ => false,
                };

                if is_ucode_start {
                    // This is a U-Code (Unicode Hex Numpad sequence)
                    j += 1;
                    let mut hex_digits = String::new();
                    let mut valid_hex = true;
                    while j < chain.len() {
                        match &chain[j].action {
                            ChainAction::Tap(kc) => {
                                if let Some(ch) = hex_keycode_to_char(kc) {
                                    hex_digits.push(ch);
                                    j += 1;
                                } else {
                                    valid_hex = false;
                                    break;
                                }
                            }
                            ChainAction::Up(kc) => {
                                if kc == "KC_LALT" || kc == "0xE2" || kc == "LALT" {
                                    break;
                                } else {
                                    valid_hex = false;
                                    break;
                                }
                            }
                            _ => {
                                valid_hex = false;
                                break;
                            }
                        }
                    }

                    if valid_hex && !hex_digits.is_empty() && j < chain.len() {
                        if let ChainAction::Up(kc) = &chain[j].action {
                            if kc == "KC_LALT" || kc == "0xE2" || kc == "LALT" {
                                condensed.push(ChainItem {
                                    id: condensed.len(),
                                    action: ChainAction::UCode(hex_digits),
                                });
                                idx = j + 1;
                                continue;
                            }
                        }
                    }
                } else {
                    // This is a standard Alt Numpad decimal sequence
                    let mut digits = String::new();
                    let mut valid_numpad = true;
                    while j < chain.len() {
                        match &chain[j].action {
                            ChainAction::Tap(kc) => {
                                if let Some(ch) = numpad_to_digit(kc) {
                                    digits.push(ch);
                                    j += 1;
                                } else {
                                    valid_numpad = false;
                                    break;
                                }
                            }
                            ChainAction::Up(kc) => {
                                if kc == "KC_LALT" || kc == "0xE2" || kc == "LALT" {
                                    break;
                                } else {
                                    valid_numpad = false;
                                    break;
                                }
                            }
                            _ => {
                                valid_numpad = false;
                                break;
                            }
                        }
                    }

                    if valid_numpad && !digits.is_empty() && j < chain.len() {
                        if let ChainAction::Up(kc) = &chain[j].action {
                            if kc == "KC_LALT" || kc == "0xE2" || kc == "LALT" {
                                condensed.push(ChainItem {
                                    id: condensed.len(),
                                    action: ChainAction::AltCode(digits),
                                });
                                idx = j + 1;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        let mut item = chain[idx].clone();
        item.id = condensed.len();
        condensed.push(item);
        idx += 1;
    }

    condensed
}

pub fn chain_to_string(chain: &[ChainItem]) -> String {
    let mut res = String::new();
    for item in chain {
        match &item.action {
            ChainAction::Tap(kc) => res.push_str(&format!("{{{}}}", kc)),
            ChainAction::Down(kc) => res.push_str(&format!("{{+{}}}", kc)),
            ChainAction::Up(kc) => res.push_str(&format!("{{-{}}}", kc)),
            ChainAction::Delay(ms) => res.push_str(&format!("{{Delay {}}}", ms)),
            ChainAction::Text(t) => res.push_str(t),
            ChainAction::AltCode(digits) => {
                res.push_str("{+KC_LALT}");
                for ch in digits.chars() {
                    if let Some(d) = ch.to_digit(10) {
                        res.push_str(&format!("{{KC_P{}}}", d));
                    }
                }
                res.push_str("{-KC_LALT}");
            }
            ChainAction::UCode(hex) => {
                res.push_str("{+KC_LALT}{KC_PPLS}");
                for ch in hex.chars() {
                    let kc = match ch.to_ascii_uppercase() {
                        '0' => "KC_0", '1' => "KC_1", '2' => "KC_2", '3' => "KC_3", '4' => "KC_4",
                        '5' => "KC_5", '6' => "KC_6", '7' => "KC_7", '8' => "KC_8", '9' => "KC_9",
                        'A' => "KC_A", 'B' => "KC_B", 'C' => "KC_C", 'D' => "KC_D", 'E' => "KC_E", 'F' => "KC_F",
                        _ => continue,
                    };
                    res.push_str(&format!("{{{}}}", kc));
                }
                res.push_str("{-KC_LALT}");
            }
        }
    }
    res
}

fn append_to_macro(mut decoded_macros: Signal<Vec<String>>, m_id: u8, act: ChainAction) {
    let mut new_chain = parse_macro_chain(&decoded_macros.read().get(m_id as usize).cloned().unwrap_or_default());
    new_chain.push(ChainItem {
        id: new_chain.len(),
        action: act,
    });
    let new_str = chain_to_string(&new_chain);
    if (m_id as usize) < decoded_macros.read().len() {
        decoded_macros.write()[m_id as usize] = new_str;
    }
}

fn remove_from_macro(mut decoded_macros: Signal<Vec<String>>, m_id: u8, idx: usize) {
    let mut new_chain = parse_macro_chain(&decoded_macros.read().get(m_id as usize).cloned().unwrap_or_default());
    if idx < new_chain.len() {
        new_chain.remove(idx);
        let new_str = chain_to_string(&new_chain);
        if (m_id as usize) < decoded_macros.read().len() {
            decoded_macros.write()[m_id as usize] = new_str;
        }
    }
}

fn move_in_macro(mut decoded_macros: Signal<Vec<String>>, m_id: u8, idx: usize, dir: i32) {
    let mut new_chain = parse_macro_chain(&decoded_macros.read().get(m_id as usize).cloned().unwrap_or_default());
    let new_idx = (idx as i32 + dir) as usize;
    if idx < new_chain.len() && new_idx < new_chain.len() {
        let item = new_chain.remove(idx);
        new_chain.insert(new_idx, item);
        let new_str = chain_to_string(&new_chain);
        if (m_id as usize) < decoded_macros.read().len() {
            decoded_macros.write()[m_id as usize] = new_str;
        }
    }
}

#[component]
pub fn MacroBuilder(
    m_id: u8,
    mut decoded_macros: Signal<Vec<String>>,
) -> Element {
    let mut active_tab = use_signal(|| "letters".to_string());
    let mut modifier_mode = use_signal(|| "tap".to_string()); // "tap", "down", "up"
    let mut delay_val = use_signal(|| "100".to_string());
    let mut text_val = use_signal(|| "".to_string());
    let mut alt_code_val = use_signal(|| "0176".to_string());
    let mut ucode_val = use_signal(|| "00B0".to_string());
    let mut dragged_pallet_action = use_signal(|| Option::<ChainAction>::None);
    let mut dragged_chain_idx = use_signal(|| Option::<usize>::None);

    let current_macro_str = decoded_macros.read().get(m_id as usize).cloned().unwrap_or_default();
    let chain = parse_macro_chain(&current_macro_str);

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 20px; width: 100%;",
            
            // --- 1. VISUAL CHAIN TIMELINE ---
            div { style: "background: rgba(20, 24, 33, 0.85); border: 1px solid rgba(255, 255, 255, 0.1); border-radius: 12px; padding: 20px; box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);",
                div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 15px;",
                    div { style: "display: flex; align-items: center; gap: 10px;",
                        span { style: "font-size: 20px;", "🔗" }
                        h3 { style: "margin: 0; font-size: 18px; color: var(--text-bright); font-weight: 600;", "Visual Macro Chain" }
                        span { style: "font-size: 12px; color: var(--text-muted); background: rgba(255, 255, 255, 0.05); padding: 3px 8px; border-radius: 12px;", "Drag keys here or click to append" }
                    }
                    if !chain.is_empty() {
                        button {
                            style: "background: rgba(239, 68, 68, 0.15); color: #ef4444; border: 1px solid rgba(239, 68, 68, 0.3); padding: 6px 14px; border-radius: 6px; cursor: pointer; font-size: 13px; font-weight: 600; transition: all 0.2s ease;",
                            onclick: move |_| {
                                if (m_id as usize) < decoded_macros.read().len() {
                                    decoded_macros.write()[m_id as usize] = String::new();
                                }
                            },
                            "🗑 Clear Chain"
                        }
                    }
                }

                div {
                    style: "background: linear-gradient(135deg, rgba(13, 16, 23, 0.9), rgba(18, 22, 30, 0.95)); border: 2px dashed rgba(100, 120, 255, 0.25); border-radius: 10px; padding: 20px; min-height: 100px; display: flex; flex-wrap: wrap; align-items: center; gap: 10px; transition: border-color 0.2s ease;",
                    ondragover: move |e| { e.prevent_default(); },
                    ondrop: move |e| {
                        e.prevent_default();
                        let from_idx_opt = *dragged_chain_idx.read();
                        let pallet_act_opt = dragged_pallet_action.read().clone();
                        if let Some(_from_idx) = from_idx_opt {
                            dragged_chain_idx.set(None);
                        } else if let Some(act) = pallet_act_opt {
                            append_to_macro(decoded_macros, m_id, act);
                            dragged_pallet_action.set(None);
                        }
                    },
                    if chain.is_empty() {
                        div { style: "width: 100%; text-align: center; color: rgba(255, 255, 255, 0.3); font-style: italic; padding: 20px 0; pointer-events: none;",
                            "✨ Chain is empty. Drag key cards from the palette below or click them to build your sequence!"
                        }
                    } else {
                        {chain.iter().enumerate().map(|(idx, item)| {
                            let (bg, border, text_col, icon, label) = match &item.action {
                                ChainAction::AltCode(digits) => {
                                    let lbl = match digits.as_str() {
                                        "0176" | "248" => format!("Alt+{} (° Degree)", digits),
                                        "0169" => format!("Alt+{} (© Copyright)", digits),
                                        "0174" => format!("Alt+{} (® Registered)", digits),
                                        "0153" => format!("Alt+{} (™ Trademark)", digits),
                                        "0177" | "241" => format!("Alt+{} (± Plus-Minus)", digits),
                                        "0128" => format!("Alt+{} (€ Euro)", digits),
                                        "0163" => format!("Alt+{} (£ Pound)", digits),
                                        "0162" => format!("Alt+{} (¢ Cent)", digits),
                                        "0149" | "7" => format!("Alt+{} (• Bullet)", digits),
                                        _ => format!("Alt+{} (Symbol)", digits),
                                    };
                                    (
                                        "linear-gradient(135deg, rgba(236, 72, 153, 0.25), rgba(244, 63, 94, 0.25))",
                                        "rgba(236, 72, 153, 0.6)",
                                        "#f43f5e",
                                        "🌟",
                                        lbl,
                                    )
                                },
                                ChainAction::UCode(hex) => {
                                    let lbl = match hex.as_str() {
                                        "00B0" => format!("U+{} (° Degree)", hex),
                                        "20AC" => format!("U+{} (€ Euro)", hex),
                                        "2192" => format!("U+{} (→ Right Arrow)", hex),
                                        "2190" => format!("U+{} (← Left Arrow)", hex),
                                        "2191" => format!("U+{} (↑ Up Arrow)", hex),
                                        "2193" => format!("U+{} (↓ Down Arrow)", hex),
                                        "2605" => format!("U+{} (★ Star)", hex),
                                        "2665" => format!("U+{} (♥ Heart)", hex),
                                        "2713" => format!("U+{} (✓ Check)", hex),
                                        "2318" => format!("U+{} (⌘ Cmd)", hex),
                                        "00A9" => format!("U+{} (© Copyright)", hex),
                                        "00AE" => format!("U+{} (® Registered)", hex),
                                        "2122" => format!("U+{} (™ Trademark)", hex),
                                        _ => format!("U+{} (Unicode Hex)", hex),
                                    };
                                    (
                                        "linear-gradient(135deg, rgba(139, 92, 246, 0.25), rgba(99, 102, 241, 0.25))",
                                        "rgba(139, 92, 246, 0.6)",
                                        "#a855f7",
                                        "✨",
                                        lbl,
                                    )
                                },
                                ChainAction::Tap(kc) => (
                                    "linear-gradient(135deg, rgba(6, 182, 212, 0.2), rgba(59, 130, 246, 0.2))",
                                    "rgba(6, 182, 212, 0.5)",
                                    "#06b6d4",
                                    "⌨",
                                    kc.clone(),
                                ),
                                ChainAction::Down(kc) => (
                                    "linear-gradient(135deg, rgba(16, 185, 129, 0.2), rgba(5, 150, 105, 0.2))",
                                    "rgba(16, 185, 129, 0.5)",
                                    "#10b981",
                                    "⬇",
                                    format!("Hold {}", kc),
                                ),
                                ChainAction::Up(kc) => (
                                    "linear-gradient(135deg, rgba(245, 158, 11, 0.2), rgba(217, 119, 6, 0.2))",
                                    "rgba(245, 158, 11, 0.5)",
                                    "#f59e0b",
                                    "⬆",
                                    format!("Release {}", kc),
                                ),
                                ChainAction::Delay(ms) => (
                                    "linear-gradient(135deg, rgba(139, 92, 246, 0.2), rgba(109, 40, 217, 0.2))",
                                    "rgba(139, 92, 246, 0.5)",
                                    "#8b5cf6",
                                    "⏱",
                                    format!("{} ms", ms),
                                ),
                                ChainAction::Text(t) => (
                                    "linear-gradient(135deg, rgba(99, 102, 241, 0.2), rgba(79, 70, 229, 0.2))",
                                    "rgba(99, 102, 241, 0.5)",
                                    "#818cf8",
                                    "💬",
                                    format!("\"{}\"", t),
                                ),
                            };

                            rsx! {
                                div {
                                    key: "{idx}",
                                    style: "display: flex; align-items: center; gap: 8px;",
                                    if idx > 0 {
                                        span { style: "color: rgba(255, 255, 255, 0.2); font-size: 14px; user-select: none;", "➔" }
                                    }
                                    div {
                                        style: "background: {bg}; border: 1px solid {border}; color: {text_col}; padding: 8px 12px; border-radius: 8px; display: flex; align-items: center; gap: 8px; font-size: 13px; font-weight: 600; box-shadow: 0 4px 12px rgba(0,0,0,0.2); cursor: grab; user-select: none; transition: transform 0.15s ease, box-shadow 0.15s ease;",
                                        draggable: "true",
                                        ondragstart: move |_| {
                                            dragged_chain_idx.set(Some(idx));
                                            dragged_pallet_action.set(None);
                                        },
                                        ondragover: move |e| { e.prevent_default(); },
                                        ondrop: move |e| {
                                            e.prevent_default();
                                            let from_idx_opt = *dragged_chain_idx.read();
                                            let pallet_act_opt = dragged_pallet_action.read().clone();
                                            if let Some(from_idx) = from_idx_opt {
                                                if from_idx != idx {
                                                    move_in_macro(decoded_macros, m_id, from_idx, (idx as i32) - (from_idx as i32));
                                                }
                                            } else if let Some(act) = pallet_act_opt {
                                                append_to_macro(decoded_macros, m_id, act);
                                                dragged_pallet_action.set(None);
                                            }
                                            dragged_chain_idx.set(None);
                                        },

                                        span { "{icon}" }
                                        span { "{label}" }

                                        div { style: "display: flex; align-items: center; gap: 4px; margin-left: 4px; border-left: 1px solid rgba(255, 255, 255, 0.15); padding-left: 6px;",
                                            if idx > 0 {
                                                button {
                                                    style: "background: none; border: none; color: inherit; opacity: 0.6; cursor: pointer; padding: 0 2px; font-size: 12px;",
                                                    title: "Move Left",
                                                    onclick: move |_| move_in_macro(decoded_macros, m_id, idx, -1),
                                                    "◀"
                                                }
                                            }
                                            if idx + 1 < chain.len() {
                                                button {
                                                    style: "background: none; border: none; color: inherit; opacity: 0.6; cursor: pointer; padding: 0 2px; font-size: 12px;",
                                                    title: "Move Right",
                                                    onclick: move |_| move_in_macro(decoded_macros, m_id, idx, 1),
                                                    "▶"
                                                }
                                            }
                                            button {
                                                style: "background: none; border: none; color: #ef4444; opacity: 0.8; cursor: pointer; padding: 0 2px; font-size: 14px; font-weight: bold;",
                                                title: "Delete",
                                                onclick: move |_| remove_from_macro(decoded_macros, m_id, idx),
                                                "×"
                                            }
                                        }
                                    }
                                }
                            }
                        })}
                    }
                }
            }

            // --- 2. KEY PALETTE ---
            div { style: "background: rgba(20, 24, 33, 0.85); border: 1px solid rgba(255, 255, 255, 0.1); border-radius: 12px; padding: 20px; box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);",
                div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 15px; border-bottom: 1px solid rgba(255, 255, 255, 0.08); padding-bottom: 12px;",
                    div { style: "display: flex; gap: 8px;",
                        {["letters", "numbers", "modifiers", "numpad", "tools"].iter().map(|&tab| {
                            let label = match tab {
                                "letters" => "🔤 Letters",
                                "numbers" => "🔢 Numbers & Nav",
                                "modifiers" => "🎛️ Modifiers",
                                "numpad" => "🌟 Numpad, Alt & U-Codes",
                                "tools" => "⏱️ Delays & Text",
                                _ => tab,
                            };
                            let is_active = *active_tab.read() == tab;
                            let bg = if is_active { "rgba(0, 240, 255, 0.15)" } else { "transparent" };
                            let col = if is_active { "#00f0ff" } else { "var(--text-muted)" };
                            let border = if is_active { "1px solid rgba(0, 240, 255, 0.4)" } else { "1px solid transparent" };
                            rsx! {
                                button {
                                    key: "{tab}",
                                    style: "background: {bg}; color: {col}; border: {border}; padding: 8px 16px; border-radius: 6px; cursor: pointer; font-size: 13px; font-weight: 600; transition: all 0.15s ease;",
                                    onclick: move |_| active_tab.set(tab.to_string()),
                                    "{label}"
                                }
                            }
                        })}
                    }
                    
                    if *active_tab.read() == "modifiers" {
                        div { style: "display: flex; align-items: center; gap: 8px; background: rgba(0, 0, 0, 0.3); padding: 4px 10px; border-radius: 8px; border: 1px solid rgba(255, 255, 255, 0.08);",
                            span { style: "font-size: 12px; color: var(--text-muted); font-weight: 600;", "Action Mode:" }
                            {["tap", "down", "up"].iter().map(|&mode| {
                                let lbl = match mode {
                                    "tap" => "Tap",
                                    "down" => "Hold (+)",
                                    "up" => "Release (-)",
                                    _ => mode,
                                };
                                let is_sel = *modifier_mode.read() == mode;
                                let m_bg = if is_sel { "#3b82f6" } else { "transparent" };
                                let m_col = if is_sel { "#ffffff" } else { "var(--text-muted)" };
                                rsx! {
                                    button {
                                        key: "{mode}",
                                        style: "background: {m_bg}; color: {m_col}; border: none; padding: 4px 10px; border-radius: 4px; font-size: 12px; cursor: pointer; font-weight: 600; transition: all 0.15s ease;",
                                        onclick: move |_| modifier_mode.set(mode.to_string()),
                                        "{lbl}"
                                    }
                                }
                            })}
                        }
                    }
                }

                div { style: "min-height: 140px; max-height: 320px; overflow-y: auto; padding-right: 6px;",
                    match active_tab.read().as_str() {
                        "letters" => rsx! {
                            div { style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(70px, 1fr)); gap: 8px;",
                                {["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"].iter().map(|&char_str| {
                                    let kc = format!("KC_{}", char_str);
                                    let act_drag = ChainAction::Tap(kc.clone());
                                    let act_click = ChainAction::Tap(kc.clone());
                                    rsx! {
                                        div {
                                            key: "{char_str}",
                                            style: "background: #1e2330; border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 6px; padding: 12px 6px; text-align: center; color: var(--text-bright); font-size: 13px; font-weight: 600; cursor: pointer; transition: all 0.15s ease; user-select: none;",
                                            draggable: "true",
                                            ondragstart: move |_| {
                                                dragged_pallet_action.set(Some(act_drag.clone()));
                                                dragged_chain_idx.set(None);
                                            },
                                            onclick: move |_| append_to_macro(decoded_macros, m_id, act_click.clone()),
                                            "{char_str}"
                                        }
                                    }
                                })}
                            }
                        },
                        "numbers" => rsx! {
                            div { style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(90px, 1fr)); gap: 8px;",
                                {["1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "ENT", "ESC", "BSPC", "TAB", "SPC", "UP", "DOWN", "LEFT", "RGHT", "MINS", "EQL", "LBRC", "RBRC", "BSLS", "SCLN", "QUOT", "COMM", "DOT", "SLSH"].iter().map(|&k_str| {
                                    let kc = format!("KC_{}", k_str);
                                    let act_drag = ChainAction::Tap(kc.clone());
                                    let act_click = ChainAction::Tap(kc.clone());
                                    rsx! {
                                        div {
                                            key: "{k_str}",
                                            style: "background: #1e2330; border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 6px; padding: 12px 6px; text-align: center; color: var(--text-bright); font-size: 13px; font-weight: 600; cursor: pointer; transition: all 0.15s ease; user-select: none;",
                                            draggable: "true",
                                            ondragstart: move |_| {
                                                dragged_pallet_action.set(Some(act_drag.clone()));
                                                dragged_chain_idx.set(None);
                                            },
                                            onclick: move |_| append_to_macro(decoded_macros, m_id, act_click.clone()),
                                            "{k_str}"
                                        }
                                    }
                                })}
                            }
                        },
                        "modifiers" => rsx! {
                            div { style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(110px, 1fr)); gap: 10px;",
                                {["LCTL", "LSFT", "LALT", "LGUI", "RCTL", "RSFT", "RALT", "RGUI"].iter().map(|&m_str| {
                                    let kc = format!("KC_{}", m_str);
                                    let mode = modifier_mode.read().clone();
                                    let act = match mode.as_str() {
                                        "down" => ChainAction::Down(kc.clone()),
                                        "up" => ChainAction::Up(kc.clone()),
                                        _ => ChainAction::Tap(kc.clone()),
                                    };
                                    let act_drag = act.clone();
                                    let act_click = act.clone();
                                    let prefix = match mode.as_str() {
                                        "down" => "⬇ Hold ",
                                        "up" => "⬆ Release ",
                                        _ => "⌨ Tap ",
                                    };
                                    rsx! {
                                        div {
                                            key: "{m_str}",
                                            style: "background: #1e2330; border: 1px solid rgba(16, 185, 129, 0.3); border-radius: 8px; padding: 14px 10px; text-align: center; color: #10b981; font-size: 13px; font-weight: 600; cursor: pointer; transition: all 0.15s ease; user-select: none;",
                                            draggable: "true",
                                            ondragstart: move |_| {
                                                dragged_pallet_action.set(Some(act_drag.clone()));
                                                dragged_chain_idx.set(None);
                                            },
                                            onclick: move |_| append_to_macro(decoded_macros, m_id, act_click.clone()),
                                            "{prefix}{m_str}"
                                        }
                                    }
                                })}
                            }
                        },
                        "numpad" => rsx! {
                            div { style: "display: flex; flex-direction: column; gap: 15px;",
                                // CUSTOM ALT NUMPAD SYMBOL BUILDER (Decimal)
                                div {
                                    style: "display: flex; flex-direction: column; gap: 12px; background: linear-gradient(135deg, rgba(236, 72, 153, 0.15), rgba(244, 63, 94, 0.15)); border: 2px solid rgba(236, 72, 153, 0.5); border-radius: 10px; padding: 15px; box-shadow: 0 4px 15px rgba(236, 72, 153, 0.15);",
                                    
                                    div { style: "display: flex; justify-content: space-between; align-items: center;",
                                        div { style: "display: flex; align-items: center; gap: 8px;",
                                            span { style: "font-size: 18px;", "🌟" }
                                            span { style: "font-size: 14px; font-weight: bold; color: #f43f5e;", "Custom Alt Numpad Symbol Builder (Decimal)" }
                                        }
                                        span { style: "font-size: 11px; color: var(--text-muted);", "Generates {{+KC_LALT}}{{KC_P0}}...{{-KC_LALT}} sequence" }
                                    }

                                    div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 8px;",
                                        span { style: "font-size: 12px; color: var(--text-bright); font-weight: 600;", "Quick Presets:" }
                                        {[("0176", "° Degree"), ("0169", "© Copyright"), ("0153", "™ TM"), ("0177", "± Plus/Minus"), ("0128", "€ Euro"), ("0149", "• Bullet")].iter().map(|&(code, label)| {
                                            let is_sel = *alt_code_val.read() == code;
                                            let btn_bg = if is_sel { "rgba(236, 72, 153, 0.5)" } else { "rgba(0, 0, 0, 0.3)" };
                                            let btn_col = if is_sel { "#ffffff" } else { "var(--text-bright)" };
                                            rsx! {
                                                button {
                                                    key: "{code}",
                                                    style: "background: {btn_bg}; color: {btn_col}; border: 1px solid rgba(236, 72, 153, 0.4); padding: 6px 10px; border-radius: 6px; font-size: 12px; font-weight: 600; cursor: pointer; transition: all 0.15s ease;",
                                                    onclick: move |_| alt_code_val.set(code.to_string()),
                                                    "{label} ({code})"
                                                }
                                            }
                                        })}
                                    }

                                    div { style: "display: flex; align-items: center; gap: 10px; margin-top: 2px;",
                                        span { style: "font-size: 13px; color: var(--text-bright); font-weight: 600;", "Alt +" }
                                        input {
                                            style: "width: 110px; background: #131722; border: 1px solid rgba(236, 72, 153, 0.6); border-radius: 6px; padding: 8px 12px; color: #f43f5e; font-size: 14px; font-weight: bold; text-align: center; outline: none;",
                                            type: "text",
                                            placeholder: "e.g. 0176",
                                            value: "{alt_code_val}",
                                            oninput: move |e| {
                                                let digits: String = e.value().chars().filter(|c| c.is_ascii_digit()).collect();
                                                alt_code_val.set(digits);
                                            },
                                        }
                                        button {
                                            style: "background: linear-gradient(135deg, #ec4899, #f43f5e); color: white; border: none; padding: 8px 18px; border-radius: 6px; font-weight: 700; cursor: pointer; transition: all 0.15s ease; box-shadow: 0 2px 8px rgba(236, 72, 153, 0.4); display: flex; align-items: center; gap: 6px;",
                                            onclick: move |_| {
                                                let digits = alt_code_val.read().clone();
                                                if !digits.is_empty() {
                                                    append_to_macro(decoded_macros, m_id, ChainAction::AltCode(digits));
                                                }
                                            },
                                            span { "➕ Add Alt Symbol to Chain" }
                                        }
                                    }
                                }

                                // CUSTOM UNICODE (U-CODE) HEX BUILDER
                                div {
                                    style: "display: flex; flex-direction: column; gap: 12px; background: linear-gradient(135deg, rgba(139, 92, 246, 0.15), rgba(99, 102, 241, 0.15)); border: 2px solid rgba(139, 92, 246, 0.5); border-radius: 10px; padding: 15px; box-shadow: 0 4px 15px rgba(139, 92, 246, 0.15);",
                                    
                                    div { style: "display: flex; justify-content: space-between; align-items: center;",
                                        div { style: "display: flex; align-items: center; gap: 8px;",
                                            span { style: "font-size: 18px;", "✨" }
                                            span { style: "font-size: 14px; font-weight: bold; color: #a855f7;", "Custom Unicode (U-Code) Hex Builder" }
                                        }
                                        span { style: "font-size: 11px; color: var(--text-muted);", "Generates {{+KC_LALT}}{{KC_PPLS}}...{{-KC_LALT}} sequence" }
                                    }

                                    div { style: "display: flex; flex-wrap: wrap; align-items: center; gap: 8px;",
                                        span { style: "font-size: 12px; color: var(--text-bright); font-weight: 600;", "Hex Presets:" }
                                        {[("00B0", "° Degree"), ("20AC", "€ Euro"), ("2192", "→ Arrow"), ("2605", "★ Star"), ("2665", "♥ Heart"), ("2713", "✓ Check")].iter().map(|&(code, label)| {
                                            let is_sel = *ucode_val.read() == code;
                                            let btn_bg = if is_sel { "rgba(139, 92, 246, 0.5)" } else { "rgba(0, 0, 0, 0.3)" };
                                            let btn_col = if is_sel { "#ffffff" } else { "var(--text-bright)" };
                                            rsx! {
                                                button {
                                                    key: "{code}",
                                                    style: "background: {btn_bg}; color: {btn_col}; border: 1px solid rgba(139, 92, 246, 0.4); padding: 6px 10px; border-radius: 6px; font-size: 12px; font-weight: 600; cursor: pointer; transition: all 0.15s ease;",
                                                    onclick: move |_| ucode_val.set(code.to_string()),
                                                    "{label} ({code})"
                                                }
                                            }
                                        })}
                                    }

                                    div { style: "display: flex; align-items: center; gap: 10px; margin-top: 2px;",
                                        span { style: "font-size: 13px; color: var(--text-bright); font-weight: 600;", "U+ Hex" }
                                        input {
                                            style: "width: 110px; background: #131722; border: 1px solid rgba(139, 92, 246, 0.6); border-radius: 6px; padding: 8px 12px; color: #a855f7; font-size: 14px; font-weight: bold; text-align: center; outline: none;",
                                            type: "text",
                                            placeholder: "e.g. 00B0",
                                            value: "{ucode_val}",
                                            oninput: move |e| {
                                                let hex_chars: String = e.value().chars()
                                                    .filter(|c| c.is_ascii_hexdigit())
                                                    .map(|c| c.to_ascii_uppercase())
                                                    .collect();
                                                ucode_val.set(hex_chars);
                                            },
                                        }
                                        button {
                                            style: "background: linear-gradient(135deg, #8b5cf6, #6366f1); color: white; border: none; padding: 8px 18px; border-radius: 6px; font-weight: 700; cursor: pointer; transition: all 0.15s ease; box-shadow: 0 2px 8px rgba(139, 92, 246, 0.4); display: flex; align-items: center; gap: 6px;",
                                            onclick: move |_| {
                                                let hex = ucode_val.read().clone();
                                                if !hex.is_empty() {
                                                    append_to_macro(decoded_macros, m_id, ChainAction::UCode(hex));
                                                }
                                            },
                                            span { "➕ Add U-Code to Chain" }
                                        }
                                    }
                                }

                                // STANDARD NUMPAD KEYS
                                div { style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(80px, 1fr)); gap: 8px;",
                                    {["P1", "P2", "P3", "P4", "P5", "P6", "P7", "P8", "P9", "P0", "PDOT", "PENT", "PPLS", "PMNS", "PAST", "PSLS", "NLCK"].iter().map(|&np_str| {
                                        let kc = format!("KC_{}", np_str);
                                        let act_drag = ChainAction::Tap(kc.clone());
                                        let act_click = ChainAction::Tap(kc.clone());
                                        rsx! {
                                            div {
                                                key: "{np_str}",
                                                style: "background: #1e2330; border: 1px solid rgba(255, 255, 255, 0.08); border-radius: 6px; padding: 12px 6px; text-align: center; color: var(--text-bright); font-size: 13px; font-weight: 600; cursor: pointer; transition: all 0.15s ease; user-select: none;",
                                                draggable: "true",
                                                ondragstart: move |_| {
                                                    dragged_pallet_action.set(Some(act_drag.clone()));
                                                    dragged_chain_idx.set(None);
                                                },
                                                onclick: move |_| append_to_macro(decoded_macros, m_id, act_click.clone()),
                                                "{np_str}"
                                            }
                                        }
                                    })}
                                }
                            }
                        },
                        "tools" => rsx! {
                            div { style: "display: flex; flex-direction: column; gap: 15px; padding: 10px 0;",
                                // DELAY TOOL
                                div { style: "display: flex; align-items: center; gap: 12px; background: rgba(0, 0, 0, 0.25); padding: 15px; border-radius: 8px; border: 1px solid rgba(255, 255, 255, 0.05);",
                                    span { style: "font-size: 20px;", "⏱" }
                                    div { style: "display: flex; flex-direction: column; flex: 1;",
                                        span { style: "font-size: 13px; font-weight: 600; color: var(--text-bright);", "Insert Custom Delay" }
                                        span { style: "font-size: 11px; color: var(--text-muted);", "Pause execution between key strokes in milliseconds" }
                                    }
                                    input {
                                        style: "width: 80px; background: #181c26; border: 1px solid rgba(255, 255, 255, 0.15); border-radius: 6px; padding: 8px 12px; color: var(--text-bright); font-size: 14px; text-align: center;",
                                        type: "number",
                                        value: "{delay_val}",
                                        oninput: move |e| delay_val.set(e.value()),
                                    }
                                    span { style: "color: var(--text-muted); font-size: 13px;", "ms" }
                                    button {
                                        style: "background: linear-gradient(135deg, #8b5cf6, #6d28d9); color: white; border: none; padding: 8px 16px; border-radius: 6px; font-weight: 600; cursor: pointer; transition: all 0.15s ease;",
                                        onclick: move |_| {
                                            if let Ok(ms) = delay_val.read().parse::<u32>() {
                                                append_to_macro(decoded_macros, m_id, ChainAction::Delay(ms));
                                            }
                                        },
                                        "+ Add Delay"
                                    }
                                }

                                // TEXT BLOCK TOOL
                                div { style: "display: flex; align-items: center; gap: 12px; background: rgba(0, 0, 0, 0.25); padding: 15px; border-radius: 8px; border: 1px solid rgba(255, 255, 255, 0.05);",
                                    span { style: "font-size: 20px;", "💬" }
                                    div { style: "display: flex; flex-direction: column; flex: 1;",
                                        span { style: "font-size: 13px; font-weight: 600; color: var(--text-bright);", "Insert Text Block" }
                                        span { style: "font-size: 11px; color: var(--text-muted);", "Automatically type out a sequence of ASCII characters" }
                                    }
                                    input {
                                        style: "flex: 2; background: #181c26; border: 1px solid rgba(255, 255, 255, 0.15); border-radius: 6px; padding: 8px 12px; color: var(--text-bright); font-size: 14px;",
                                        type: "text",
                                        placeholder: "Type text (e.g. hello world)...",
                                        value: "{text_val}",
                                        oninput: move |e| text_val.set(e.value()),
                                    }
                                    button {
                                        style: "background: linear-gradient(135deg, #3b82f6, #2563eb); color: white; border: none; padding: 8px 16px; border-radius: 6px; font-weight: 600; cursor: pointer; transition: all 0.15s ease;",
                                        onclick: move |_| {
                                            if !text_val.read().is_empty() {
                                                append_to_macro(decoded_macros, m_id, ChainAction::Text(text_val.read().clone()));
                                                text_val.set("".to_string());
                                            }
                                        },
                                        "+ Add Text"
                                    }
                                }
                            }
                        },
                        _ => rsx! { div {} }
                    }
                }
            }

            // --- 3. RAW MACRO TEXTAREA (Synchronized) ---
            div { style: "background: rgba(20, 24, 33, 0.85); border: 1px solid rgba(255, 255, 255, 0.1); border-radius: 12px; padding: 20px; box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);",
                div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px;",
                    h3 { style: "margin: 0; font-size: 16px; color: var(--text-bright); font-weight: 600;", "📝 Raw Macro Syntax" }
                    span { style: "font-size: 12px; color: var(--text-muted);", "Edits here sync instantly with the chain builder above" }
                }
                textarea {
                    style: "width: 100%; height: 120px; font-family: 'Fira Code', monospace; background: #131722; color: #00f0ff; border: 1px solid rgba(255, 255, 255, 0.15); padding: 15px; border-radius: 8px; resize: vertical; box-sizing: border-box; font-size: 14px; line-height: 1.5; outline: none; transition: border-color 0.2s ease;",
                    value: current_macro_str,
                    oninput: move |e| {
                        if (m_id as usize) < decoded_macros.read().len() {
                            decoded_macros.write()[m_id as usize] = e.value();
                        }
                    }
                }
            }
        }
    }
}
