use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Result;
use std::io::Write; 
use std::path::Path; 
use crate::entities::player::Player;

const FILE_PATH: &str = "player.json";

pub fn get_player() -> Player {
    let file_path = Path::new(FILE_PATH);

    if file_path.exists() {
        if let Ok(file) = File::open(file_path) {
            if let Ok(player) = serde_json::from_reader(file) {
                return player; 
            }
        }
    }
 
    // If file doesn't exist (or fails to parse), make a default Player
    let default_player = Player::new(0, 0, 0);

    if let Ok(json) = serde_json::to_string_pretty(&default_player) {
        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true) // overwrite if it exists
            .open(file_path)
        {
            let _ = file.write_all(json.as_bytes());
        }
    }   
    default_player
}

pub fn update_exp(value: u32)-> Result<()> {
    let mut json_file = File::open(FILE_PATH)?;
    let mut contents = String::new();
    json_file.read_to_string(&mut contents)?;
        let mut player: Player = serde_json::from_str(&contents).unwrap();

    // Step 4: Update the struct
    player.set_exp(value);

    // Step 5: Serialize back to JSON
    let new_json = serde_json::to_string_pretty(&player).unwrap();

    // Step 6: Overwrite file with new JSON
    let mut file = OpenOptions::new().write(true).truncate(true).open(FILE_PATH)?;
    file.write_all(new_json.as_bytes())?;
    Ok(())
}


pub fn update_highscore(value: u32)-> Result<()> {
    let mut json_file = File::open(FILE_PATH)?;
    let mut contents = String::new();
    json_file.read_to_string(&mut contents)?;
        let mut player: Player = serde_json::from_str(&contents).unwrap();

    // Step 4: Update the struct
    player.set_highscore(value);

    // Step 5: Serialize back to JSON
    let new_json = serde_json::to_string_pretty(&player).unwrap();

    // Step 6: Overwrite file with new JSON
    let mut file = OpenOptions::new().write(true).truncate(true).open(FILE_PATH)?;
    file.write_all(new_json.as_bytes())?;
    Ok(())
}

pub fn update_revive(value: u32)-> Result<()> {
    let mut json_file = File::open(FILE_PATH)?;
    let mut contents = String::new();
    json_file.read_to_string(&mut contents)?;
        let mut player: Player = serde_json::from_str(&contents).unwrap();

    // Step 4: Update the struct
    player.set_revive(value);

    // Step 5: Serialize back to JSON
    let new_json = serde_json::to_string_pretty(&player).unwrap();

    // Step 6: Overwrite file with new JSON
    let mut file = OpenOptions::new().write(true).truncate(true).open(FILE_PATH)?;
    file.write_all(new_json.as_bytes())?;
    Ok(())
}