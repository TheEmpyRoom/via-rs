#![allow(non_snake_case)]

mod hid;
mod layout;
mod ui;

#[cfg(not(target_arch = "wasm32"))]
fn load_icon() -> Option<dioxus::desktop::tao::window::Icon> {
    let img_bytes = include_bytes!("ui/assets/logo.png");
    let img = image::load_from_memory_with_format(img_bytes, image::ImageFormat::Png).ok()?;
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    dioxus::desktop::tao::window::Icon::from_rgba(rgba.into_raw(), width, height).ok()
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut window = dioxus::desktop::WindowBuilder::new()
            .with_title("VIA-RS")
            .with_decorations(false)
            .with_inner_size(dioxus::desktop::tao::dpi::LogicalSize::new(1280.0, 800.0))
            .with_min_inner_size(dioxus::desktop::tao::dpi::LogicalSize::new(900.0, 600.0))
            .with_always_on_top(false);
            
        if let Some(icon) = load_icon() {
            window = window.with_window_icon(Some(icon));
        }

        dioxus::LaunchBuilder::desktop()
            .with_cfg(
                dioxus::desktop::Config::new()
                    .with_window(window)
                    .with_menu(None)
            )
            .launch(ui::components::App);
    }

    #[cfg(target_arch = "wasm32")]
    {
        dioxus::LaunchBuilder::web().launch(ui::components::App);
    }
}

