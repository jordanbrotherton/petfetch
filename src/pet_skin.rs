use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use rand::seq::IndexedRandom;
use serde::Deserialize;

use crate::pet::PetMoods;

#[derive(Debug, Clone, Deserialize)]
pub struct EvolutionConfig {
    pub target_skin: String,
    pub min_age_hours: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PetData {
    #[serde(default)]
    pub sprites: HashMap<PetMoods, Vec<String>>,
    #[serde(default)]
    pub quotes: HashMap<PetMoods, Vec<String>>,
    pub evolves_to: Option<EvolutionConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PetStore {
    pub pets: HashMap<String, PetData>,
}

impl PetStore {
    /// Creates the PetStore and loads in the default pets.
    pub fn load() -> Self {
        const DEFAULT_JSON: &str = include_str!("default_pets.json");
        let store: PetStore =
            serde_json::from_str(DEFAULT_JSON).expect("Default Pets is malformed!");
        return store;
    }

    /// Loads in the custom pets inside of the 'petfetch/skins.json' file.
    pub fn load_custom(&mut self) {
        let custom_path = dirs::config_dir()
            .unwrap()
            .join("petfetch")
            .join("skins.json");

        let Ok(file) = File::open(custom_path) else {
            return;
        };

        let reader = BufReader::new(file);
        let store: PetStore =
            serde_json::from_reader(reader).expect("Could not read 'skins.json'.");

        // Overlaying default pets.
        for (name, custom_pet) in store.pets {
            self.pets
                .entry(name)
                .and_modify(|base| {
                    base.sprites.extend(custom_pet.sprites.clone());
                    base.quotes.extend(custom_pet.quotes.clone());
                })
                .or_insert(custom_pet);
        }
    }

    /// Obtains the sprite for the specified pet's mood.
    pub fn get_sprite(&self, skin_name: &str, mood: PetMoods) -> &[String] {
        self.pets
            .get(skin_name)
            .and_then(|pet| {
                pet.sprites
                    .get(&mood)
                    .or_else(|| pet.sprites.get(&PetMoods::Normal))
            })
            .or_else(|| {
                eprintln!(
                    "Could not find sprite for mood {:?} in {}!",
                    mood, skin_name
                );
                self.pets.get("Blob").and_then(|blob| {
                    blob.sprites
                        .get(&mood)
                        .or_else(|| blob.sprites.get(&PetMoods::Normal))
                })
            })
            .map(|lines| lines.as_slice())
            .unwrap_or(&[])
    }

    /// Obtains a random quote for the specified pet's mood.
    pub fn get_quote(&self, skin_name: &str, mood: PetMoods) -> &str {
        let quotes = self
            .pets
            .get(skin_name)
            .and_then(|pet| {
                pet.quotes
                    .get(&mood)
                    .filter(|lines| !lines.is_empty())
                    .or_else(|| {
                        pet.quotes
                            .get(&PetMoods::Normal)
                            .filter(|lines| !lines.is_empty())
                    })
            })
            .or_else(|| {
                eprintln!(
                    "Could not find quotes for mood {:?} in {}!",
                    mood, skin_name
                );
                self.pets.get("Blob").and_then(|blob| {
                    blob.quotes
                        .get(&mood)
                        .filter(|lines| !lines.is_empty())
                        .or_else(|| {
                            blob.quotes
                                .get(&PetMoods::Normal)
                                .filter(|lines| !lines.is_empty())
                        })
                })
            })
            .map(|lines| lines.as_slice())
            .unwrap_or(&[]);
        quotes
            .choose(&mut rand::rng())
            .map(|s| s.as_str())
            .unwrap_or("")
    }
}
