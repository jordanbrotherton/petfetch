mod config;
mod pet;
mod pet_skin;

fn main() {
    let mut pet = pet::Pet::new("Pet".to_string(), pet_skin::PetSkin::Blob);
    pet.update();
    pet.check();
}
