use std::fmt::{self, Display, Formatter};

use super::{
    weapon::{Weapon, WeaponType}
};

pub struct Character {
    pub(crate) char_type: CharacterType,
    pub(crate) name: String,
    pub(crate) life: i32,
    pub(crate) max_life: i32,
    pub(crate) weapon: Weapon,
}

impl Character {
    pub fn new(char_type: CharacterType, name: String, life: i32, weapon: Weapon) -> Character {
        Self {
            char_type,
            name,
            life,
            max_life: life,
            weapon,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.life > 0
    }

    pub fn format_life(&self) -> String {
        format!("{}/{}❤", self.life, self.max_life)
    }

    pub fn print_life(&self) {
        print!("{}", self.format_life())
    }
}

#[derive(Debug)]
pub enum CharacterType {
    Soldier,
    Mage,
    Archer,
    Colossus,
}

impl CharacterType {
    pub fn get_character(&self, name: String) -> Character {
        match self {
            Self::Soldier => Character::new(Self::Soldier, name, 120, WeaponType::Sword.get_weapon()),
            Self::Mage => Character::new(Self::Mage, name, 80, WeaponType::Wand.get_weapon()),
            Self::Archer => Character::new(Self::Archer, name, 100, WeaponType::Bow.get_weapon()),
            Self::Colossus => Character::new(Self::Colossus, name, 180, WeaponType::FistBump.get_weapon()),
        }
    }
}

impl Display for CharacterType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}