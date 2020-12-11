use super::weapon::{Weapon, WeaponType};
use std::fmt::{self, Display, Formatter};

pub struct Character {
    pub(crate) name: String,
    pub(crate) char_type: CharacterType,
    pub(crate) life: i32,
    pub(crate) max_life: i32,
    pub(crate) weapon: Weapon,
}

impl Character {
    pub fn is_alive(&self) -> bool {
        self.life > 0
    }

    pub fn format_life(&self) -> String {
        format!("{}/{}", self.life, self.max_life)
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
            Self::Soldier => Character {
                name,
                char_type: Self::Soldier,
                life: 100,
                max_life: 100,
                weapon: WeaponType::Sword.get_weapon(),
            },
            Self::Mage => Character {
                name,
                char_type: Self::Mage,
                life: 80,
                max_life: 80,
                weapon: WeaponType::Wand.get_weapon(),
            },
            Self::Archer => Character {
                name,
                char_type: Self::Archer,
                life: 100,
                max_life: 100,
                weapon: WeaponType::Bow.get_weapon(),
            },
            Self::Colossus => Character {
                name,
                char_type: Self::Colossus,
                life: 150,
                max_life: 150,
                weapon: WeaponType::FistBump.get_weapon(),
            }
        }
    }
}

impl Display for CharacterType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}