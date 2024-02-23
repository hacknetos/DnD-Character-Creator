use std::{ fs::File, io::{ BufReader, Read }, path::Path };
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct Race_Bonus {
    pub name: String,
    pub bonus: u8,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Under_Species {
    pub name: String,
    pub description: String,
    pub under_race_bonus: Option<Vec<Race_Bonus>>,
    pub under_race_skills: Option<Vec<Race_Skills>>,
    pub under_race_resistance: Option<Vec<String>>,
    pub under_race_lang: Option<Vec<String>>,
    pub under_race_size: Option<Race_size>,
    pub under_race_age: Option<(u8, u8)>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Race_Skills {
    pub name: String,
    pub description: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Race_size {
    pub min: u16,
    pub max: u16,
    pub description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Race {
    pub name: String,
    pub description: String,
    pub speed: u16,
    pub bonuses: Vec<Race_Bonus>,
    pub standard_size: Race_size,
    pub standard_age: (u16, u16),
    pub standard_weight: u16,
    pub languages: Option<Vec<String>>,
    pub resistance: Option<Vec<String>>,
    pub race_skills: Option<Vec<Race_Skills>>,
    pub under_species: Option<Vec<Under_Species>>,
    pub hand_book: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RaceWrapper {
    pub races: Vec<Race>,
}

pub fn check_Race_file(race_file_path: &str) -> Result<bool, bool> {
    if Path::new(race_file_path).is_file() == false { Err(false) } else { Ok(true) }
}
pub fn Read_RaceFile(race_file_path: &str) -> Result<RaceWrapper, String> {
    if check_Race_file(race_file_path).is_err() {
        Err("File Dont Exist".to_string())
    } else {
        let file = File::open(race_file_path);

        if file.is_err() {
            std::println!("{:?}", file.unwrap_err().to_string());
            Err("An error has occurred while reading".to_string())
        } else {
            let reader = BufReader::new(file.unwrap());
            let json = serde_json::from_reader(reader);
            if json.is_err() {
                println!("{:?}", json.unwrap_err());
                Err(String::from("Error acurt"))
            } else {
                Ok(json.unwrap())
            }
        }
    }
}
pub fn get_all_known_Races(race_file_path: &str) -> Result<RaceWrapper, String> {
    let all_known_Races = Read_RaceFile(race_file_path);
    if all_known_Races.is_err() {
        std::println!("{:?}", all_known_Races.unwrap_err());
        let err = String::from("A Error Acurt Sorry");
        Err(err)
    } else {
        Ok(all_known_Races.unwrap())
    }
}
