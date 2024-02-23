use std::{ fs::File, io::Write, path::Path, error::Error };
use std::io::BufReader;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Options {
    race_file_path: String,
    class_file_path: String,
    character_files_dir: String,
}
const OPTION_PATH: &str = "C:\\DnDCharacterGen\\Option.JSON";

pub fn create_Options() {
    let mut file = File::create(OPTION_PATH);
    if file.is_err() {
        std::println!("{}", file.unwrap_err());
    } else {
        file.unwrap().write_all(
            b"{\n\"race_file_path\n:\"\",\n\"class_file_path\": \"\",\"character_files_dir\": \"\"\n}"
        );
    }
}

pub fn exists_Options() -> bool {
    if !Path::new(OPTION_PATH).exists() && !Path::new(OPTION_PATH).is_file() {
        return false;
    } else {
        return true;
    }
}

pub fn read_Options() -> Result<Options, Box<dyn Error>> {
    if !exists_Options() {
        create_Options();
    }

    let file = File::open(OPTION_PATH)?;
    let reader = BufReader::new(file);
    let json = serde_json::from_reader(reader)?;
    Ok(json)
}
pub fn get_race_file_path() -> Result<String, String> {
    let optin_json = read_Options();
    if optin_json.is_err() {
        Err(optin_json.unwrap_err().to_string())
    } else {
        Ok(optin_json.unwrap().race_file_path)
    }
}
pub fn get_class_file_path() -> Result<String, String> {
    let optin_json = read_Options();
    if optin_json.is_err() {
        Err(optin_json.unwrap_err().to_string())
    } else {
        Ok(optin_json.unwrap().class_file_path)
    }
}
pub fn get_character_files_dir() -> Result<String, String> {
    let optin_json = read_Options();
    if optin_json.is_err() {
        Err(optin_json.unwrap_err().to_string())
    } else {
        Ok(optin_json.unwrap().character_files_dir)
    }
}
