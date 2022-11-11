#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] //Hide console window in release builds on Windows, this blocks stdout.

mod app;
mod defines;

use app::RustyAutoClickerApp;
use defines::*;
use eframe::egui;

// When compiling natively
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let native_options = eframe::NativeOptions {
        always_on_top: true,
        decorated: true,
        initial_window_size: Some(egui::vec2(WINDOW_WIDTH, WINDOW_HEIGHT)),
        resizable: false,
        transparent: true,
        icon_data: Some(load_icon()),
        ..Default::default()
    };

    eframe::run_native(
        &format!("{} v{}", APP_NAME, env!("CARGO_PKG_VERSION")),
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Box::new(RustyAutoClickerApp::new(cc))
        }),
    );
}

pub fn load_icon() -> eframe::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let embedded_ico = include_bytes!("../assets/icon-64x64.ico");
        let image = image::load_from_memory(embedded_ico)
            .expect("Failed to open icon path")
            .into_rgba8();

        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
