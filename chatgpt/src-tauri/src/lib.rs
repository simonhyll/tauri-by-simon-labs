use log::LevelFilter;
use tauri_plugin_log::{Target, TargetKind};
use tauri::{AppHandle, Manager};

#[tauri::command]
async fn prompt(app: AppHandle, message: String) -> Result<(), ()> {
  log::info!("Received message!");
  tauri::async_runtime::spawn(async move {
    let window = app.clone().get_webview_window("main").unwrap();
    let _ = window.emit("message", "Hello!");
  });
  Ok(())
}

#[tauri::command]
async fn connect(app: AppHandle, token: String) -> Result<(), ()> {
  log::info!("Connecting!");
  Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(LevelFilter::Info)
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![prompt, connect])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
