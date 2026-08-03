#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CustomKeycode {
    pub name: String,
    pub title: String,
    pub short_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ViaDefinition {
    pub name: String,
    pub vendor_id: String,
    pub product_id: String,
    pub matrix: MatrixDef,
    pub layouts: LayoutsDef,
    #[serde(default)]
    pub custom_keycodes: Vec<CustomKeycode>,
    #[serde(default)]
    pub menus: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MatrixDef {
    pub rows: u8,
    pub cols: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LayoutsDef {
    // KLE format is a mix of objects and strings, so we store it as a generic JSON Value for now.
    // In a fully featured app, this would be parsed into a structured representation of keys with x, y, width, height, etc.
    pub keymap: serde_json::Value,
}
