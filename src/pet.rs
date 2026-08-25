use sysinfo::System;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Pet {
    pub name: String,
    last_checked: std::time::SystemTime,
    food: u32,
    bladder: u32,
    joy: u32,
    ill: bool,
    when_ill: Option<std::time::SystemTime>,
    is_dead: bool,
    skin: crate::pet_skin::PetSkin,
}

pub enum PetMoods {
    Happy,
    Normal,
    Hungry,
    Bladder,
    Ill,
    Dead,
}

pub enum PlayResult {
    DeniedHungry,
    DeniedBladder,
    DeniedIll,
    Won,
    Lost,
}

pub enum MedicateResult {
    DeniedHungry,
    DeniedNotIll,
    Healed,
}

impl Pet {
    pub fn new(name: String, skin: crate::pet_skin::PetSkin) -> Pet {
        Pet {
            name,
            last_checked: std::time::SystemTime::now(),
            food: 100,
            bladder: 0,
            joy: 100,
            ill: false,
            when_ill: None,
            is_dead: false,
            skin: skin,
        }
    }

    /// Prints the fetch of your pet.
    pub fn check(&self) {
        let art = self.skin.get_art(self.get_mood());

        let mut sys = System::new();
        sys.refresh_memory();

        let host = System::host_name().unwrap_or_else(|| "localhost".to_string());
        let os = System::name().unwrap_or_else(|| "Unknown OS".to_string());
        let uptime_mins = System::uptime() / 60;
        let mem_used = sys.used_memory() / 1024 / 1024;
        let mem_total = sys.total_memory() / 1024 / 1024;

        let stats = vec![
            format!("{} @ {}", self.name, host),
            "-------------".to_string(),
            format!("OS:      {}", os),
            format!("Uptime:  {}m", uptime_mins),
            format!("Memory:  {}MB / {}MB", mem_used, mem_total),
            format!("Food:    {}% / 100%", self.food),
            format!("Joy:     {}% / 100%", self.joy),
        ];

        let max_lines = art.len().max(stats.len());

        for i in 0..max_lines {
            let left = art.get(i).unwrap_or(&"            ");
            let right = stats.get(i).map(|s| s.as_str()).unwrap_or("");
            println!("{}   {}", left, right);
        }
    }

    /// Obtains the current mood of your pet.
    fn get_mood(&self) -> PetMoods {
        if self.is_dead {
            PetMoods::Dead
        } else if self.ill {
            PetMoods::Ill
        } else if self.bladder > 80 {
            PetMoods::Bladder
        } else if self.food < 25 {
            PetMoods::Hungry
        } else if self.food > 70 && self.joy > 70 {
            PetMoods::Happy
        } else {
            PetMoods::Normal
        }
    }

    /// Updates your pet's needs.
    pub fn update(&mut self) {
        if self.is_dead {
            return;
        }

        let check = self.last_checked.elapsed().unwrap();
        let check_hours = check.as_secs() / 3600;

        self.last_checked = std::time::SystemTime::now();

        if self.ill {
            if let Some(when_ill) = self.when_ill {
                let ill_duration = when_ill.elapsed().unwrap().as_secs();
                if ill_duration > crate::config::DEATH_TIME {
                    self.is_dead = true;
                }
            }
        }

        let food_decay = crate::config::FOOD_DECAY_RATE as u64;
        let joy_decay = crate::config::JOY_DECAY_RATE as u64;

        let lost_food = check_hours * food_decay;
        let lost_joy = check_hours * joy_decay;

        self.food = self.food.saturating_sub(lost_food as u32);
        self.joy = self.joy.saturating_sub(lost_joy as u32);

        if self.food == 0 && !self.ill {
            self.ill = true;
            self.when_ill = Some(self.last_checked);
        }

        if self.bladder >= 100 {
            self.bladder = 0;
            self.joy = 0;
            if !self.ill {
                self.ill = true;
                self.when_ill = Some(self.last_checked);
            }
        }
    }

    /// Feeds your pet.
    /// Returns true if fed, false if not.
    pub fn feed(&mut self) -> bool {
        if self.food >= 100 {
            return false; // <_< theyre full
        }
        let amount_ate = 100 - self.food;
        self.food = 100; // yum
        self.bladder = self.bladder.saturating_add(amount_ate / 2).min(100);
        return true;
    }

    /// Relieves your pet's bladder.
    /// Returns true if successful, false if not.
    pub fn toilet(&mut self) -> bool {
        if self.bladder == 0 {
            return false; // nothing there
        }

        self.bladder = 0;
        return true;
    }

    /// Plays a guessing game with your pet.
    /// Returns a PlayResult determining whether it played or if you won/lost.
    pub fn play(&mut self, choice: bool) -> PlayResult {
        if self.food < 10 {
            return PlayResult::DeniedHungry;
        }
        if self.bladder > 90 {
            return PlayResult::DeniedBladder;
        }
        if self.ill {
            return PlayResult::DeniedIll;
        }

        let pet_choice = rand::random::<bool>();

        if choice == pet_choice {
            self.joy = (self.joy + 35).min(100);
            return PlayResult::Won;
        } else {
            self.joy = (self.joy + 10).min(100);
            return PlayResult::Lost;
        }
    }

    /// Heals your sick pet.
    /// Returns MedicateResult, giving a reason if they were/weren't healed.
    pub fn medicate(&mut self) -> MedicateResult {
        if self.food == 0 {
            return MedicateResult::DeniedHungry;
        }
        if !self.ill {
            return MedicateResult::DeniedNotIll;
        }

        self.ill = false;
        self.when_ill = None;
        return MedicateResult::Healed;
    }
}
