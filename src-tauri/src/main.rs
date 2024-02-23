// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use race::{ get_all_known_Races, Race, RaceWrapper };
use tauri::{ Manager, Window };
use serde::Serialize;
mod race;
mod option;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_race() -> Result<Vec<race::Race>, String> {
    let opt_path = option::get_race_file_path();
    if opt_path.is_err() {
        Err(String::from("Option File Error"))
    } else {
        let v_race = race::get_all_known_Races(opt_path.unwrap().as_str());
        if v_race.is_err() {
            Err(String::from("Error on Get The Races"))
        } else {
            Ok(v_race.unwrap().races)
        }
    }
}

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![greet, get_race])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
