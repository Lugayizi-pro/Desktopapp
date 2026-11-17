#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(dead_code, unused_imports)]

mod app;
mod arabic;
mod audio;
mod data;
mod theme;

/// Folder for settings, bookmarks, downloaded translations and audio on desktop.
#[cfg(not(target_os = "android"))]
fn desktop_save_dir() -> std::path::PathBuf {
    use std::path::PathBuf;
    let base = if cfg!(target_os = "windows") {
        std::env::var_os("APPDATA").map(PathBuf::from)
    } else if cfg!(target_os = "macos") {
        std::env::var_os("HOME").map(|h| PathBuf::from(h).join("Library/Application Support"))
    } else {
        std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))
    };
    let dir = base.unwrap_or_else(|| PathBuf::from(".")).join("RustQuran");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

#[cfg(not(target_os = "android"))]
fn main() -> eframe::Result<()> {
    env_logger::init();
    app::set_save_dir(desktop_save_dir());
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("RustQuran")
            .with_inner_size([420.0, 820.0])
            .with_min_inner_size([320.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "RustQuran",
        options,
        Box::new(|cc| Ok(Box::new(app::QuranApp::new(cc)))),
    )
}

// On Android the app starts from `android_main` in lib.rs.
#[cfg(target_os = "android")]
fn main() {}
