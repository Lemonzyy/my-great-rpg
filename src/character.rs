use super::weapon::Weapon;

pub struct Character {
    pub(crate) name: String,
    pub(crate) char_type: CharacterType,
    pub(crate) life: i32,
    pub(crate) max_life: i32,
    pub(crate) weapon: Weapon,
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
                life: 0,
                max_life: 0,
                weapon: Weapon::new("Sword".to_string(), 10).unwrap()
            },
            Self::Mage => Character {
                name,
                char_type: Self::Mage,
                life: 0,
                max_life: 0,
                weapon: Weapon::new("Wand".to_string(), 10).unwrap()
            },
            Self::Archer => Character {
                name,
                char_type: Self::Archer,
                life: 0,
                max_life: 0,
                weapon: Weapon::new("Bow".to_string(), 10).unwrap()
            },
            Self::Colossus => Character {
                name,
                char_type: Self::Colossus,
                life: 0,
                max_life: 0,
                weapon: Weapon::new("Fist Bump".to_string(), 10).unwrap()
            }
        }
    }
}