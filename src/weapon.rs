use std::fmt::{self, Display, Formatter};

pub struct Weapon {
    pub(crate) name: String,
    pub(crate) w_type: WeaponType,
    pub(crate) damage: i32,
}

impl Weapon {
    pub fn new(w_type: WeaponType, damage: i32, name: String) -> Result<Self, String> {
        if damage == 0 {
            return Err(format!("Weapon `{}` should not have a damage of {}", w_type, damage));
        }

        Ok(Self {
            name,
            w_type,
            damage,
        })
    }
}

#[derive(Debug)]
pub enum WeaponType {
    Sword,
    Wand,
    Bow,
    FistBump,
}

impl WeaponType {
    pub fn get_weapon(&self) -> Weapon {
        match self {
            Self::Sword => Weapon::new(Self::Sword, 8, "Sword".to_string()).unwrap(),
            Self::Wand => Weapon::new(Self::Wand, 15, "Wand".to_string()).unwrap(),
            Self::Bow => Weapon::new(Self::Bow, 10, "Bow".to_string()).unwrap(),
            Self::FistBump => Weapon::new(Self::FistBump, 5, "Fist Bump".to_string()).unwrap(),
        }
    }
}

impl Display for WeaponType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}