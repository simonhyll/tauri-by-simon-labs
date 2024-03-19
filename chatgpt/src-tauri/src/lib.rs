use log::LevelFilter;
use tauri_plugin_log::{Target, TargetKind};
use tauri::{AppHandle, Manager};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use std::collections::HashMap;
use reqwest::Client;

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConversationRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Default, Serialize, Deserialize)]
struct ChatGPT {
  #[serde(skip_serializing)]
  token: String,
  discussions: HashMap<u64, Vec<Message>>,
}

#[tauri::command]
fn prompt(app: AppHandle, discussion: u64, model: String, message: String) -> Result<(), ()> {
  log::info!("Received message: {}", message);
  tauri::async_runtime::block_on(async move {
    let window = app.clone().get_webview_window("main").unwrap();
    let state = app.state::<Mutex<ChatGPT>>();
    let mut lock = state.lock().unwrap();
    lock.discussions.get_mut(&discussion).unwrap().push(Message{role:"user".to_string(), content:message});
    let token = lock.token.clone();
    let messages = lock.discussions.get(&discussion).unwrap().clone();
    drop(lock);
    let conversation_api_url = "https://api.openai.com/v1/chat/completions";
    let conversation_request = ConversationRequest {
        model: model,
        messages: messages,
    };
    let client = Client::new();
    let response = client
        .post(conversation_api_url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&conversation_request)
        .send()
        .await.expect("request failed");
    if response.status().is_success() {
      let response_body = response.text().await.expect("could not get response body as text");
      let json: serde_json::Value = serde_json::from_str(&response_body)
          .expect("Failed to parse JSON");
  
      if let Some(first_choice_content) = json["choices"][0]["message"]["content"].as_str() {
          log::info!("Response: {}", first_choice_content);
          let message = Message{
            role:"system".to_string(),
            content:first_choice_content.to_string()
          };
          let mut lock = state.lock().unwrap();
          lock.discussions.get_mut(&discussion).unwrap().push(message.clone());
          let _ = window.emit("message", message);
      } else {
          log::error!("Could not find the expected data in the response");
      }
    } else {
        log::error!("Error: {}", response.status());
    }
  });
  Ok(())
}

#[tauri::command]
async fn connect(app: AppHandle, token: String) -> Result<(), ()> {
  log::info!("Connecting!");
  let state = app.state::<Mutex<ChatGPT>>();
  let mut lock = state.lock().unwrap();
  lock.token = token;
  Ok(())
}

#[tauri::command]
async fn get_messages(app: AppHandle, discussion: u64) -> Result<Vec<Message>, ()> {
  log::info!("Getting messages!");
  let state = app.state::<Mutex<ChatGPT>>();
  let lock = state.lock().unwrap();
  Ok(lock.discussions.get(&discussion).unwrap().clone())
}

#[tauri::command]
async fn create_discussion(app: AppHandle) -> Result<u64, ()> {
  log::info!("Discussion created!");
  let state = app.state::<Mutex<ChatGPT>>();
  let mut lock = state.lock().unwrap();
  let mut key: u64 = 1;
  while lock.discussions.contains_key(&key) {
    key += 1
  }
  lock.discussions.insert(key, vec![]);
  Ok(key)
}


#[tauri::command]
async fn get_discussions(app: AppHandle) -> Result<Vec<u64>, ()> {
  log::info!("Getting discussions!");
  let state = app.state::<Mutex<ChatGPT>>();
  let lock = state.lock().unwrap();
  Ok(lock.discussions.keys().cloned().collect())
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
        .manage(Mutex::new(ChatGPT::default()))
        .invoke_handler(tauri::generate_handler![prompt, connect, get_messages, create_discussion, get_discussions])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
