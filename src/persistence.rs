use std::path::PathBuf;

use crate::Pet;

/// Creates and returns the save path.
pub fn initialize() -> PathBuf {
    let save_path = dirs::config_dir()
        .unwrap()
        .join("petfetch")
        .join("pet.json");

    if let Err(e) = std::fs::create_dir_all(save_path.parent().unwrap()) {
        eprintln!("Warning: failed to create petfetch directory: {}", e);
    }

    return save_path;
}

/// Saves your pet as a JSON in the specified path.
pub fn save_pet(pet: &Pet, path: &str) -> std::io::Result<()> {
    let json = serde_json::to_string(&pet).unwrap();
    std::fs::write(path, json)
}

/// Loads your pet's JSON from the specified path.
pub fn load_pet(path: &str) -> std::io::Result<Pet> {
    let json = std::fs::read_to_string(path)?;
    let pet: Pet = serde_json::from_str(&json).unwrap();
    Ok(pet)
}
