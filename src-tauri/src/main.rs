// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // Disable the DMABuf renderer to run on my 3900x Linux box.
    // Run one of these from within src-tauri/:
    //     - cargo tauri dev
    //     - cargo tauri build -b none
    // The first is "developer mode" and the second is "production mode"
    // in the second case you can run `./target/release/tauri-app` to start the app.
    //
    // If you comment out the line and do one of to get the same results as above:
    //     - WEBKIT_DISABLE_DMABUF_RENDERER=1 cargo tauri dev
    //     - WEBKIT_DISABLE_DMABUF_RENDERER=1 cargo tauri build -b none
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
