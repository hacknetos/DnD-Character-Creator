// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use race::{ get_all_known_Races, Race };
use serde::Serialize;
mod race;
mod option;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn invoke_get_all_known_races() -> Result<Vec<Race>, String> {
    let v_option = option::get_race_file_path();
    if v_option.is_err() {
        Err(v_option.unwrap_err())
    } else {
        let all_known_races = get_all_known_Races(v_option.unwrap().as_str());
        if all_known_races.is_err() {
            Err(all_known_races.unwrap_err())
        } else {
            Ok(all_known_races.unwrap().races)
        }
    }
}

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![greet, invoke_get_all_known_races])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
