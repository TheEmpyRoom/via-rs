#![allow(dead_code)]

#[cfg(not(target_arch = "wasm32"))]
use hidapi::HidDevice;

pub const VIA_REPORT_SIZE: usize = 32;

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum ViaCommand {
    GetProtocolVersion = 0x01,
    GetKeyboardValue = 0x02,
    SetKeyboardValue = 0x03,
    DynamicKeymapGetKeycode = 0x04,
    DynamicKeymapSetKeycode = 0x05,
    DynamicKeymapReset = 0x06,
    CustomSetValue = 0x07,
    CustomGetValue = 0x08,
    CustomSave = 0x09,
    EepromReset = 0x0A,
    BootloaderJump = 0x0B,
    MacroGetCount = 0x0C,
    MacroGetBufferSize = 0x0D,
    MacroGetBuffer = 0x0E,
    MacroSetBuffer = 0x0F,
    MacroReset = 0x10,
    GetLayerCount = 0x11,
    GetBuffer = 0x12,
    SetBuffer = 0x13,
}

#[cfg(not(target_arch = "wasm32"))]
pub struct ViaKeyboard<'a> {
    device: &'a HidDevice,
}

#[cfg(not(target_arch = "wasm32"))]
impl<'a> ViaKeyboard<'a> {
    pub fn new(device: &'a HidDevice) -> Self {
        Self { device }
    }

    pub fn send_command(&self, command: ViaCommand, data: &[u8]) -> Result<[u8; VIA_REPORT_SIZE], String> {
        let mut report = [0u8; VIA_REPORT_SIZE + 1];
        report[0] = 0x00; // Report ID
        report[1] = command as u8;
        
        let copy_len = data.len().min(VIA_REPORT_SIZE - 1);
        report[2..2 + copy_len].copy_from_slice(&data[..copy_len]);

        self.device.write(&report).map_err(|e| format!("Failed to write: {}", e))?;

        let mut read_buf = [0u8; VIA_REPORT_SIZE + 1];
        let bytes_read = self.device.read_timeout(&mut read_buf, 1000).map_err(|e| format!("Failed to read: {}", e))?;
        
        if bytes_read == 0 {
            return Err("Timeout waiting for response from keyboard".to_string());
        }

        let mut response = [0u8; VIA_REPORT_SIZE];
        
        let offset = if bytes_read == VIA_REPORT_SIZE + 1 { 1 } else { 0 };
        response.copy_from_slice(&read_buf[offset..offset + VIA_REPORT_SIZE]);
        
        Ok(response)
    }

    pub fn get_protocol_version(&self) -> Result<u16, String> {
        let resp = self.send_command(ViaCommand::GetProtocolVersion, &[])?;
        let version = u16::from_be_bytes([resp[1], resp[2]]);
        Ok(version)
    }

    pub fn get_keyboard_value(&self, value_id: u8) -> Result<u32, String> {
        let resp = self.send_command(ViaCommand::GetKeyboardValue, &[value_id])?;
        Ok(u32::from_be_bytes([resp[2], resp[3], resp[4], resp[5]]))
    }

    pub fn set_keyboard_value(&self, value_id: u8, value: u32) -> Result<(), String> {
        let bytes = value.to_be_bytes();
        self.send_command(ViaCommand::SetKeyboardValue, &[value_id, bytes[0], bytes[1], bytes[2], bytes[3]])?;
        Ok(())
    }

    pub fn get_layer_count(&self) -> Result<u8, String> {
        let resp = self.send_command(ViaCommand::GetLayerCount, &[])?;
        Ok(resp[1])
    }

    pub fn get_keycode(&self, layer: u8, row: u8, col: u8) -> Result<u16, String> {
        let resp = self.send_command(ViaCommand::DynamicKeymapGetKeycode, &[layer, row, col])?;
        Ok(u16::from_be_bytes([resp[4], resp[5]]))
    }

    pub fn set_keycode(&self, layer: u8, row: u8, col: u8, keycode: u16) -> Result<(), String> {
        let bytes = keycode.to_be_bytes();
        self.send_command(ViaCommand::DynamicKeymapSetKeycode, &[layer, row, col, bytes[0], bytes[1]])?;
        Ok(())
    }

    pub fn get_keyboard_value_raw(&self, value_id: u8) -> Result<[u8; VIA_REPORT_SIZE], String> {
        self.send_command(ViaCommand::GetKeyboardValue, &[value_id])
    }

    pub fn get_custom_value_raw(&self, channel: u8, offset: u8) -> Result<[u8; VIA_REPORT_SIZE], String> {
        self.send_command(ViaCommand::CustomGetValue, &[channel, offset])
    }

    pub fn get_custom_value(&self, channel: u8, offset: u8) -> Result<[u8; 2], String> {
        let resp = self.send_command(ViaCommand::CustomGetValue, &[channel, offset])?;
        Ok([resp[3], resp[4]])
    }

    pub fn set_custom_value(&self, channel: u8, offset: u8, value_bytes: [u8; 2]) -> Result<(), String> {
        self.send_command(ViaCommand::CustomSetValue, &[channel, offset, value_bytes[0], value_bytes[1]])?;
        Ok(())
    }

    pub fn custom_save(&self, channel: u8, offset: u8) -> Result<(), String> {
        self.send_command(ViaCommand::CustomSave, &[channel, offset])?;
        Ok(())
    }

    pub fn get_macro_count(&self) -> Result<u8, String> {
        let resp = self.send_command(ViaCommand::MacroGetCount, &[])?;
        Ok(resp[1])
    }

    pub fn get_macro_buffer_size(&self) -> Result<u16, String> {
        let resp = self.send_command(ViaCommand::MacroGetBufferSize, &[])?;
        Ok(u16::from_be_bytes([resp[1], resp[2]]))
    }

    pub fn get_macro_buffer(&self, offset: u16, size: u8) -> Result<Vec<u8>, String> {
        let offset_bytes = offset.to_be_bytes();
        let resp = self.send_command(ViaCommand::MacroGetBuffer, &[offset_bytes[0], offset_bytes[1], size])?;
        Ok(resp[4..4 + size as usize].to_vec())
    }

    pub fn set_macro_buffer(&self, offset: u16, data: &[u8]) -> Result<(), String> {
        let offset_bytes = offset.to_be_bytes();
        let size = data.len() as u8;
        let mut payload = vec![offset_bytes[0], offset_bytes[1], size];
        payload.extend_from_slice(data);
        self.send_command(ViaCommand::MacroSetBuffer, &payload)?;
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
pub struct ViaKeyboard;

#[cfg(target_arch = "wasm32")]
impl ViaKeyboard {
    pub fn new<T>(_device: &T) -> Self { Self }

    pub async fn send_command_async(&self, command: ViaCommand, data: &[u8]) -> Result<[u8; VIA_REPORT_SIZE], String> {
        use wasm_bindgen_futures::JsFuture;

        let data_json = serde_json::to_string(&data).unwrap_or_else(|_| "[]".to_string());
        let eval_code = format!("window._webhid_via_send({}, {})", command as u8, data_json);

        let promise_js = js_sys::eval(&eval_code).map_err(|e| format!("JS eval error: {:?}", e))?;
        let promise = js_sys::Promise::from(promise_js);
        let res = JsFuture::from(promise).await.map_err(|e| {
            if let Some(s) = e.as_string() { s } else { "WebHID report error".to_string() }
        })?;

        let arr = js_sys::Array::from(&res);
        let mut response = [0u8; VIA_REPORT_SIZE];
        for i in 0..VIA_REPORT_SIZE.min(arr.length() as usize) {
            response[i] = arr.get(i as u32).as_f64().unwrap_or(0.0) as u8;
        }
        Ok(response)
    }

    pub async fn get_protocol_version_async(&self) -> Result<u16, String> {
        let resp = self.send_command_async(ViaCommand::GetProtocolVersion, &[]).await?;
        Ok(u16::from_be_bytes([resp[1], resp[2]]))
    }

    pub async fn get_keyboard_value_async(&self, value_id: u8) -> Result<u32, String> {
        let resp = self.send_command_async(ViaCommand::GetKeyboardValue, &[value_id]).await?;
        Ok(u32::from_be_bytes([resp[2], resp[3], resp[4], resp[5]]))
    }

    pub async fn set_keyboard_value_async(&self, value_id: u8, value: u32) -> Result<(), String> {
        let bytes = value.to_be_bytes();
        self.send_command_async(ViaCommand::SetKeyboardValue, &[value_id, bytes[0], bytes[1], bytes[2], bytes[3]]).await?;
        Ok(())
    }

    pub async fn get_layer_count_async(&self) -> Result<u8, String> {
        let resp = self.send_command_async(ViaCommand::GetLayerCount, &[]).await?;
        Ok(resp[1])
    }

    pub async fn get_keycode_async(&self, layer: u8, row: u8, col: u8) -> Result<u16, String> {
        let resp = self.send_command_async(ViaCommand::DynamicKeymapGetKeycode, &[layer, row, col]).await?;
        Ok(u16::from_be_bytes([resp[4], resp[5]]))
    }

    pub async fn set_keycode_async(&self, layer: u8, row: u8, col: u8, keycode: u16) -> Result<(), String> {
        let bytes = keycode.to_be_bytes();
        self.send_command_async(ViaCommand::DynamicKeymapSetKeycode, &[layer, row, col, bytes[0], bytes[1]]).await?;
        Ok(())
    }

    pub async fn get_keyboard_value_raw_async(&self, value_id: u8) -> Result<[u8; VIA_REPORT_SIZE], String> {
        self.send_command_async(ViaCommand::GetKeyboardValue, &[value_id]).await
    }

    pub async fn get_custom_value_raw_async(&self, channel: u8, offset: u8) -> Result<[u8; VIA_REPORT_SIZE], String> {
        self.send_command_async(ViaCommand::CustomGetValue, &[channel, offset]).await
    }

    pub async fn get_custom_value_async(&self, channel: u8, offset: u8) -> Result<[u8; 2], String> {
        let resp = self.send_command_async(ViaCommand::CustomGetValue, &[channel, offset]).await?;
        Ok([resp[3], resp[4]])
    }

    pub async fn set_custom_value_async(&self, channel: u8, offset: u8, value_bytes: [u8; 2]) -> Result<(), String> {
        self.send_command_async(ViaCommand::CustomSetValue, &[channel, offset, value_bytes[0], value_bytes[1]]).await?;
        Ok(())
    }

    pub async fn custom_save_async(&self, channel: u8, offset: u8) -> Result<(), String> {
        self.send_command_async(ViaCommand::CustomSave, &[channel, offset]).await?;
        Ok(())
    }

    pub async fn get_macro_count_async(&self) -> Result<u8, String> {
        let resp = self.send_command_async(ViaCommand::MacroGetCount, &[]).await?;
        Ok(resp[1])
    }

    pub async fn get_macro_buffer_size_async(&self) -> Result<u16, String> {
        let resp = self.send_command_async(ViaCommand::MacroGetBufferSize, &[]).await?;
        Ok(u16::from_be_bytes([resp[1], resp[2]]))
    }

    pub async fn get_macro_buffer_async(&self, offset: u16, size: u8) -> Result<Vec<u8>, String> {
        let offset_bytes = offset.to_be_bytes();
        let resp = self.send_command_async(ViaCommand::MacroGetBuffer, &[offset_bytes[0], offset_bytes[1], size]).await?;
        Ok(resp[4..4 + size as usize].to_vec())
    }

    pub async fn set_macro_buffer_async(&self, offset: u16, data: &[u8]) -> Result<(), String> {
        let offset_bytes = offset.to_be_bytes();
        let size = data.len() as u8;
        let mut payload = vec![offset_bytes[0], offset_bytes[1], size];
        payload.extend_from_slice(data);
        self.send_command_async(ViaCommand::MacroSetBuffer, &payload).await?;
        Ok(())
    }
}

