use std::{
    io::{self, Write},
    path::Path,
};

use clap::{Parser, Subcommand};

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

use crate::pet::Pet;

mod config;
mod pet;
mod pet_skin;

fn main() {
    let save_path = dirs::config_dir()
        .unwrap()
        .join("petfetch")
        .join("pet.json");

    if let Err(e) = std::fs::create_dir_all(save_path.parent().unwrap()) {
        eprintln!("Warning: failed to create petfetch directory: {}", e);
    }

    let loaded_pet = pet::Pet::load_pet(&save_path.to_string_lossy());

    if let Ok(pet) = loaded_pet {
        interact_pet(pet, &save_path);
    } else {
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

            let pet = Pet::new(name, pet_skin::PetSkin::Blob);
            interact_pet(pet, &save_path);
        } else {
            return;
        }
    }
}

fn interact_pet(mut pet: Pet, save_path: &Path) {
    let cli = Cli::parse();
    pet.update();
    match cli.command {
        Some(PetCommand::Feed) => {
            if pet.feed() {
                println!("{} is fed!", pet.name);
            } else {
                println!("{} is full...", pet.name);
            }
        }
        Some(PetCommand::Play { direction }) => {
            let result: pet::PlayResult;
            let direction_bool: bool;
            let direction = direction.unwrap_or_default();
            match direction.to_lowercase().as_str() {
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
                    if direction_bool {
                        println!("You look right!");
                        println!("{} looks right too!", pet.name);
                    } else {
                        println!("You look left!");
                        println!("{} looks left too!", pet.name);
                    }
                    println!("You win! {} is very happy at your victory!", pet.name);
                }
                pet::PlayResult::Lost => {
                    if direction_bool {
                        println!("You look right!");
                        println!("{} looks left...", pet.name);
                    } else {
                        println!("You look left!");
                        println!("{} looks right...", pet.name);
                    }
                    println!("You lose. {} is slightly happy at their victory.", pet.name);
                }
            }
        }
        Some(PetCommand::Toilet) => {
            if pet.toilet() {
                println!("{} is refreshed!", pet.name);
            } else {
                println!("{} doesn't need to go.", pet.name);
            }
        }
        Some(PetCommand::Medicate) => match pet.medicate() {
            pet::MedicateResult::DeniedHungry => {
                println!("{} is hungry... Can't heal an empty stomach!", pet.name);
            }
            pet::MedicateResult::DeniedNotIll => {
                println!("{} is healthy. No need to heal!", pet.name);
            }
            pet::MedicateResult::Healed => {
                println!("{} is feeling better!", pet.name);
            }
        },
        None => {
            pet.check();
        }
    }
    if let Err(e) = pet.save_pet(&save_path.to_string_lossy()) {
        eprintln!("Warning: failed to save pet: {}", e);
    }
}
