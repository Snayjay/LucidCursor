// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use tauri::Manager;
use std::thread;
use std::time::Duration;
use rdev::{listen, Event, EventType};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            
            // 1. Set ignore cursor events so clicks pass through
            // Note: On Windows/macOS this allows clicking what's behind.
            let _ = window.set_ignore_cursor_events(true);

            // 2. Start Mouse Follower
            // We use rdev to listen to global mouse events
            let w_handle = window.clone();
            thread::spawn(move || {
                if let Err(error) = listen(move |event| {
                    match event.event_type {
                        EventType::MouseMove { x, y } => {
                             // Center the 300x300 window on the cursor
                             // Position is top-left, so subtract 150
                             let new_pos = tauri::Position::Physical(tauri::PhysicalPosition {
                                 x: (x as i32) - 150,
                                 y: (y as i32) - 150,
                             });
                             let _ = w_handle.set_position(new_pos);
                        }
                         _ => {}
                    }
                }) {
                    println!("Error: {:?}", error);
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
