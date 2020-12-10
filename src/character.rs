use crate::weapon::WeaponType;

use super::weapon::Weapon;

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
}

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
                weapon: Weapon::new(WeaponType::Sword, 10).unwrap(),
            },
            Self::Mage => Character {
                name,
                char_type: Self::Mage,
                life: 80,
                max_life: 80,
                weapon: Weapon::new(WeaponType::Wand, 10).unwrap(),
            },
            Self::Archer => Character {
                name,
                char_type: Self::Archer,
                life: 100,
                max_life: 100,
                weapon: Weapon::new(WeaponType::Bow, 10).unwrap(),
            },
            Self::Colossus => Character {
                name,
                char_type: Self::Colossus,
                life: 150,
                max_life: 150,
                weapon: Weapon::new(WeaponType::FistBump, 10).unwrap(),
            }
        }
    }
}