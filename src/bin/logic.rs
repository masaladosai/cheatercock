use anyhow::Result;
use arboard::Clipboard;
use rdev::{listen, EventType, Key};
use reqwest::blocking::Client;
use serde_json::{json,Value};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use std::fs;
use serde::Deserialize;
use serde_json;
use std::env;
use once_cell::sync::OnceCell;

static PROMPT: OnceCell<String> = OnceCell::new();


#[derive(Debug, Deserialize)]
struct Config {
    api_key: String,
}


fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        PROMPT.set(args[1].clone()).expect("Failed to set prompt");
        println!("global prompt set to: {}", preview(PROMPT.get().unwrap()));
    } else {
        PROMPT.set("you are a coding assistant.".to_string()).expect("Failed to set default prompt");
        println!("No argument received! Using default prompt.");
    }
    let data = fs::read_to_string("config.json")
        .expect("Unable to read config.json");
    // Parse JSON into struct
    let config: Config = serde_json::from_str(&data)
        .expect("JSON was not well-formatted");

    let api_key = config.api_key.clone();
    println!("Using API Key: {}\n", mask_api_key(&api_key));
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
        api_key
    );

  
    let client = Client::new();

    println!("cheatercock is running");

    let saved_text: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    let press: Arc<Mutex<HashSet<Key>>> = Arc::new(Mutex::new(HashSet::new()));
    let triggered = Arc::new(Mutex::new(HashSet::new()));
    let busy = Arc::new(Mutex::new(false));
    let answer: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    // We can clone everything once outside the listen loop
    let saved_text_copy = saved_text.clone();
    let press_copy = press.clone();
    let triggered_copy = triggered.clone();
    let busy_copy = busy.clone();
    let answer_copy = answer.clone();
    listen(move |event| {
        match event.event_type {
            EventType::KeyPress(k) => {
                press_copy.lock().unwrap().insert(k);
            }
            EventType::KeyRelease(k) => {
                press_copy.lock().unwrap().remove(&k);
                triggered_copy.lock().unwrap().remove(&k);
            }
            _ => {}
        }

        let here = |k: &Key| press_copy.lock().unwrap().contains(k);
        let combo = |keys: &[Key]| keys.iter().all(|k| here(k));

        if combo(&[Key::ShiftLeft, Key::ControlLeft, Key::Alt, Key::KeyK]) {
            let mut trig = triggered_copy.lock().unwrap();
            if !trig.contains(&Key::KeyK) {
                trig.insert(Key::KeyK);
                if let Ok(mut cpb) = Clipboard::new() {
                        match cpb.get_text() {
                            Ok(copytext) => {
                                let trimmed_text = copytext.trim().to_string();
                                if !trimmed_text.is_empty() {
                                    *saved_text_copy.lock().unwrap() = Some(trimmed_text.clone());
                                    println!("saved text: {}", &trimmed_text);
                                } else {
                                    println!("clipboard is empty");
                                }
                            }
                            Err(e) => {
                                println!("cant copy from clipboard: {:?}", e);
                            }
                        }
                } else {
                    println!("clipboard not accesable");
                }
            }
        } else if combo(&[Key::ShiftLeft, Key::ControlLeft, Key::Alt, Key::KeyP]) {
            let mut trig = triggered_copy.lock().unwrap();
            if !trig.contains(&Key::KeyP) {
                trig.insert(Key::KeyP);

                let busy_clone = busy_copy.clone();
                let saved_text_clone = saved_text_copy.clone();
                let client_clone = client.clone();
                let url_clone = url.clone();

                thread::spawn(move || {
                    
                    let mut is_busy_guard = busy_clone.lock().unwrap();
                    if *is_busy_guard {
                        println!("Already processing, wait...");
                        return; 
                    }
                    *is_busy_guard = true;
                    drop(is_busy_guard);

                    if let Some(t) = saved_text_clone.lock().unwrap().clone() {
                        println!("current saved text {}", preview(&t));
                        let prompt_text = PROMPT.get().unwrap();
                        let final_prompt = format!("{}\n\n{}", prompt_text, t);
                        let body = json!({
                            "contents": [
                                {"parts": [ { "text": final_prompt }]}]
                        });

                        let res = client_clone.post(&url_clone)
                            .header("Content-Type", "application/json")
                            .json(&body)
                            .send();

                        *busy_clone.lock().unwrap() = false;

                        match res {
                            Ok(response) => {
                                if let Ok(tx) = response.text() {
                                    let v: Value = serde_json::from_str(&tx).unwrap();
                                    if let Some(ans) = v["candidates"][0]["content"]["parts"][0]["text"].as_str() {
                                                println!("Model answer:\n{}", &ans);
                                           
                                    if let Ok(mut cb) = Clipboard::new() {
                                        if cb.set_text(ans.to_string()).is_ok() {
                                            println!("result copied to clipboard.");
                                        } else {
                                            println!("couldnt write to clipboard");
                                        }
                                    } else {
                                        println!("couldnt access clipboard");
                                    }
                                    } else {
                                                println!("Could not extract text");
                                            }
                                        } else {
                                    println!("cant read response txt");
                                }
                            }
                            Err(e) => {
                                println!("req fail to send: {}", e);
                            }
                        }
                    } else {
                        println!("no text copied");
                        *busy_clone.lock().unwrap() = false;
                    }
                });
            }
        } else if combo(&[Key::ShiftLeft, Key::ControlLeft, Key::Alt, Key::KeyQ]) {
            println!("exiting");
            std::process::exit(0);
        }
    })
    .map_err(|e| anyhow::anyhow!("Listen error: {:?}", e))?;

    Ok(())
}

fn preview(s: &str) -> String {
    const MAX: usize = 80;
    if s.len() <= MAX {
        s.to_string()
    } else {
        format!("{}…", &s[..MAX])
    }
}

fn mask_api_key(s: &str) -> String {
    // show first 4 and last 4 characters when long enough, otherwise mask fully
    if s.len() <= 8 {
        "(redacted)".to_string()
    } else {
        let start = &s[..4];
        let end = &s[s.len()-4..];
        format!("{}{}{}", start, "*".repeat(s.len().saturating_sub(8)), end)
    }
}
