#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, Window};
use tracing::{event, Level};
use tracing_subscriber;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) {
    println!("Being greeted: {}", name);
    let payload = format!("Hello, {}! You've been greeted from Rust!", name);
    event!(Level::INFO, payload);
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}
struct MyWriter {
    window: Window,
}
impl std::io::Write for MyWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = String::from_utf8_lossy(buf);
        self.window
            .emit(
                "TRACE",
                Payload {
                    message: s.to_string(),
                },
            )
            .unwrap();
        Ok(s.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            let make_my_writer = move || -> Box<dyn std::io::Write> {
                Box::new(MyWriter {
                    window: main_window.clone(),
                })
            };
            let format = tracing_subscriber::fmt::format().json();
            tracing_subscriber::fmt()
                .event_format(format)
                .with_writer(make_my_writer)
                .init();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
