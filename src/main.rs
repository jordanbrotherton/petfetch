mod config;
mod persistence;
mod pet;
mod pet_skin;

use crate::pet::Pet;
use clap::{Parser, Subcommand};
use std::{
    io::{self, Write},
    path::{Path, PathBuf},
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<PetCommand>,
}

#[derive(Subcommand)]
enum PetCommand {
    /// Feed your pet whenever it's hungry
    Feed,
    /// Play a guessing game with your pet
    Play {
        /// The direction to guess, either 'left' or 'right'
        direction: Option<String>,
    },
    /// Let your pet relieve its bladder
    Toilet,
    /// Heal your pet when it is ill
    Medicate,
}

fn main() {
    let save_path: PathBuf = persistence::initialize();
    let loaded_pet = persistence::load_pet(&save_path.to_string_lossy())
        .ok()
        .or_else(adopt_pet);

    if let Some(pet) = loaded_pet {
        interact_pet(pet, &save_path);
    } else {
        return;
    }
}

/// Adopts a new pet to inhabit your terminal.
fn adopt_pet() -> std::option::Option<Pet> {
    print!("You have no pet. Want to adopt one? [Y/n]: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim().eq_ignore_ascii_case("y") {
        print!("What is your pet's name? [Pet]: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trim = input.trim();

        let name = if trim.is_empty() {
            "Pet".to_string()
        } else {
            trim.to_string()
        };

        Some(Pet::new(name, pet_skin::PetSkin::Blob))
    } else {
        None
    }
}

/// Handles the main interaction with a valid pet.
fn interact_pet(mut pet: Pet, save_path: &Path) {
    pet.update(); // Updates the pet's stats before we start.

    // Send off the command to their handler.
    let cli = Cli::parse();
    match cli.command {
        Some(PetCommand::Feed) => {
            feed_pet(&mut pet);
        }
        Some(PetCommand::Play { direction }) => {
            play_pet(&mut pet, direction);
        }
        Some(PetCommand::Toilet) => {
            bathe_pet(&mut pet);
        }
        Some(PetCommand::Medicate) => {
            medicate_pet(&mut pet);
        }
        None => {
            pet.check();
        }
    }

    // Save the updated pet.
    if let Err(e) = persistence::save_pet(&pet, &save_path.to_string_lossy()) {
        eprintln!("Warning: failed to save pet: {}", e);
    }
}

/// Feeds your pet.
fn feed_pet(pet: &mut Pet) {
    if pet.feed() {
        println!("{} is fed!", pet.name);
    } else {
        println!("{} is full...", pet.name);
    }
}

/// Plays a guessing game with your pet.
fn play_pet(pet: &mut Pet, direction: Option<String>) {
    let result: pet::PlayResult;
    let direction_bool: bool;
    let direction = direction.unwrap_or_default().to_lowercase();
    match direction.as_str() {
        "left" => {
            direction_bool = false;
        }
        "right" => {
            direction_bool = true;
        }
        _ => {
            println!(
                "{} is confused at your move... (left/right only!)",
                pet.name
            );
            return;
        }
    }

    result = pet.play(direction_bool);
    match result {
        pet::PlayResult::DeniedBladder => {
            println!("{} has to go, not play right now...", pet.name);
        }
        pet::PlayResult::DeniedHungry => {
            println!("{} is too hungry to play right now...", pet.name);
        }
        pet::PlayResult::DeniedIll => {
            println!("{} is too sick to play...", pet.name);
        }
        pet::PlayResult::Won => {
            println!(
                "You look {}!\n{} looks {} too!",
                direction, pet.name, direction
            );
            println!("You win! {} is very happy at your victory!", pet.name);
        }
        pet::PlayResult::Lost => {
            println!(
                "You look {}!\n{} looks {}...",
                direction, pet.name, direction
            );
            println!("You lose. {} is slightly happy at its victory.", pet.name);
        }
    }
}

/// Relieves your pet's bladder.
fn bathe_pet(pet: &mut Pet) {
    if pet.toilet() {
        println!("{} is refreshed!", pet.name);
    } else {
        println!("{} doesn't need to go.", pet.name);
    }
}

/// Heal your sick pet.
fn medicate_pet(pet: &mut Pet) {
    match pet.medicate() {
        pet::MedicateResult::DeniedHungry => {
            println!("{} is hungry... Can't heal an empty stomach!", pet.name);
        }
        pet::MedicateResult::DeniedNotIll => {
            println!("{} is healthy. No need to heal!", pet.name);
        }
        pet::MedicateResult::Healed => {
            println!("{} is feeling better!", pet.name);
        }
    }
}
