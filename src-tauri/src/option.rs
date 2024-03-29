use std::{ fs::File, io::Write, path::Path, error::Error };
use std::io::BufReader;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Options {
    race_file_path: String,
    class_file_path: String,
    character_files_dir: String,
}
//INFO All Posebile Static Paths on The Main OS es
const OPTION_PATH_WIN: &str = "C:\\DnDCharacterGen\\Option.JSON";
const OPTION_PATH_UNI: &str = "/home/$USER/Option.JSON";
const OPTION_PATH_MAC: &str = ""; //FIXME I Dont Fucking Know how the Paths hear lol
pub fn create_Options() {
    let mut file = File::create(OPTION_PATH_WIN);
    if cfg!(unix) {
        file = File::create(OPTION_PATH_UNI);
    } else if cfg!(target_os = "macos") {
        file = File::create(OPTION_PATH_MAC);
    }
    if file.is_err() {
        std::println!("{}", file.unwrap_err());
    } else {
        file.unwrap().write_all(
            b"{\n\"race_file_path\n:\"\",\n\"class_file_path\": \"\",\"character_files_dir\": \"\"\n}"
        );
    }
}

pub fn exists_Options() -> bool {
    if cfg!(windows) {
        if !Path::new(OPTION_PATH_WIN).exists() && !Path::new(OPTION_PATH_WIN).is_file() {
            println!("Option WIN Parh Dont exist");
            return false;
        }
    } else if cfg!(unix) {
        println!("Option unix Parh Dont exist");
        if !Path::new(OPTION_PATH_UNI).exists() && !Path::new(OPTION_PATH_UNI).is_file() {
            return false;
        }
    } else if cfg!(target_os = "macos") {
        println!("Option MAC Parh Dont exist");
        if !Path::new(OPTION_PATH_MAC).exists() && !Path::new(OPTION_PATH_MAC).is_file() {
            return false;
        }
    }
    println!("Path exist");
    return true;
}

pub fn read_Options() -> Result<Options, Box<dyn Error>> {
    if !exists_Options() {
        create_Options();
    }

    let mut file = File::open(OPTION_PATH_WIN)?;
    if cfg!(unix) {
        file = File::open(OPTION_PATH_UNI)?;
    } else if cfg!(target_os = "macos") {
        file = File::open(OPTION_PATH_MAC)?;
    }
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
