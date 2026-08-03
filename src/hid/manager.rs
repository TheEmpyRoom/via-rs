const VIA_USAGE_PAGE: u16 = 0xFF60;
const VIA_USAGE: u16 = 0x0061;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyboardInfo {
    pub vendor_id: u16,
    pub product_id: u16,
    pub path: String,
    pub product_string: String,
}

pub fn scan_for_keyboards() -> Result<Vec<KeyboardInfo>, String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let api = hidapi::HidApi::new().map_err(|e| format!("Failed to init HID api: {}", e))?;
        let mut keyboards = Vec::new();

        for device in api.device_list() {
            if device.usage_page() == VIA_USAGE_PAGE && device.usage() == VIA_USAGE {
                keyboards.push(KeyboardInfo {
                    vendor_id: device.vendor_id(),
                    product_id: device.product_id(),
                    path: device.path().to_string_lossy().to_string(),
                    product_string: device.product_string().unwrap_or("Unknown Keyboard").to_string(),
                });
            }
        }

        Ok(keyboards)
    }

    #[cfg(target_arch = "wasm32")]
    {
        Ok(Vec::new())
    }
}

#[cfg(target_arch = "wasm32")]
pub async fn request_webhid_device() -> Result<Option<KeyboardInfo>, String> {
    use wasm_bindgen_futures::JsFuture;

    let eval_code = r#"
        (async () => {
            if (!navigator.hid) {
                throw new Error("WebHID is not supported in this browser. Please use Chrome, Edge, Opera, or Brave.");
            }
            const devices = await navigator.hid.requestDevice({ filters: [] });
            if (!devices || devices.length === 0) return null;
            const dev = devices[0];
            if (!dev.opened) {
                await dev.open();
            }
            window._webhid_device = dev;
            window._webhid_via_send = async function(cmdByte, dataArray) {
                if (!window._webhid_device) throw new Error("No WebHID device connected");
                if (!window._webhid_device.opened) await window._webhid_device.open();

                return new Promise((resolve, reject) => {
                    const timeout = setTimeout(() => {
                        window._webhid_device.oninputreport = null;
                        reject(new Error("WebHID timeout waiting for command " + cmdByte));
                    }, 1500);

                    window._webhid_device.oninputreport = (e) => {
                        const bytes = new Uint8Array(e.data.buffer, e.data.byteOffset, e.data.byteLength);
                        if (bytes.length > 0) {
                            let start = -1;
                            if (bytes[0] === cmdByte) {
                                start = 0;
                            } else if (bytes.length > 1 && bytes[0] === 0 && bytes[1] === cmdByte) {
                                start = 1;
                            }
                            if (start >= 0) {
                                clearTimeout(timeout);
                                window._webhid_device.oninputreport = null;
                                const arr = new Uint8Array(32);
                                arr.set(bytes.subarray(start, Math.min(bytes.length, start + 32)));
                                resolve(Array.from(arr));
                            }
                        }
                    };

                    const report = new Uint8Array(32);
                    report[0] = cmdByte;
                    if (dataArray && dataArray.length > 0) {
                        for (let i = 0; i < Math.min(dataArray.length, 31); i++) {
                            report[1 + i] = dataArray[i];
                        }
                    }

                    window._webhid_device.sendReport(0, report).catch(err => {
                        clearTimeout(timeout);
                        window._webhid_device.oninputreport = null;
                        reject(err);
                    });
                });
            };

            return {
                vendorId: dev.vendorId,
                productId: dev.productId,
                productName: dev.productName || "WebHID Keyboard"
            };
        })()
    "#;

    let promise_js = js_sys::eval(eval_code).map_err(|e| format!("JS eval error: {:?}", e))?;
    let promise = js_sys::Promise::from(promise_js);
    let res = JsFuture::from(promise).await.map_err(|e| {
        if let Some(s) = e.as_string() {
            s
        } else {
            "WebHID error or user closed prompt".to_string()
        }
    })?;

    if res.is_null() || res.is_undefined() {
        return Ok(None);
    }

    let vid = js_sys::Reflect::get(&res, &"vendorId".into())
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0) as u16;

    let pid = js_sys::Reflect::get(&res, &"productId".into())
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0) as u16;

    let name = js_sys::Reflect::get(&res, &"productName".into())
        .ok()
        .and_then(|v| v.as_string())
        .unwrap_or_else(|| "WebHID Keyboard".to_string());

    Ok(Some(KeyboardInfo {
        vendor_id: vid,
        product_id: pid,
        path: format!("webhid_{}_{}", vid, pid),
        product_string: name,
    }))
}




