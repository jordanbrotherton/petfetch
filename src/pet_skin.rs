use std::vec;

use crate::pet::PetMoods;

// TODO - make this modular

pub enum PetSkin {
    Blob,
}

impl PetSkin {
    pub fn get_art(&self, mood: PetMoods) -> Vec<&'static str> {
        match self {
            PetSkin::Blob => match mood {
                PetMoods::Happy => vec![
                    r"  .------.  ",
                    r" /  ^‿^   \ ",
                    r"|          |",
                    r" \        / ",
                    r"  '------'  ",
                ],
                PetMoods::Normal => vec![
                    r"  .------.  ",
                    r" /        \ ",
                    r"|   ._.    |",
                    r" \        / ",
                    r"  '------'  ",
                ],
                PetMoods::Hungry => vec![
                    r"  .------.  ",
                    r" /  . .   \ ",
                    r"|    o     |",
                    r" \        / ",
                    r"  '------'  ",
                ],
                PetMoods::Ill => vec![
                    r"  .------.  ",
                    r" / '-_-   \ ",
                    r"|       '  |",
                    r" \  '     / ",
                    r"  '------'  ",
                ],
                PetMoods::Bladder => vec![
                    r"  .------.  ",
                    r" /  >_< ' \ ",
                    r"|          |",
                    r" \        / ",
                    r"  '------'  ",
                ],
                PetMoods::Dead => vec![
                    r"  .------.  ",
                    r" /        \ ",
                    r"|   x_x    |",
                    r" \        / ",
                    r"  '------'  ",
                ],
            },
        }
    }
}
