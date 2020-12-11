use std::fmt::{self, Display, Formatter};

pub struct Weapon {
    pub(crate) w_type: WeaponType,
    pub(crate) damage: i32,
}

impl Weapon {
    pub fn new(w_type: WeaponType, damage: i32) -> Result<Self, String> {
        if damage == 0 {
            return Err(format!("Weapon `{}` should not have a damage of {}", w_type, damage));
        }

        Ok(Self {
            w_type,
            damage,
        })
    }

    pub fn is_offensive(&self) -> bool {
        self.damage > 0
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
            Self::Sword => Weapon::new(Self::Sword, 10).unwrap(),
            Self::Wand => Weapon::new(Self::Wand, 10).unwrap(),
            Self::Bow => Weapon::new(Self::Bow, 10).unwrap(),
            Self::FistBump => Weapon::new(Self::FistBump, 10).unwrap(),
        }
    }
    pub fn get_name(&self) -> String {
        match self {
            Self::Sword => "Sword",
            Self::Wand => "Wand",
            Self::Bow => "Bow",
            Self::FistBump => "Fist Bump",
        }.to_string()
    }
}

impl Display for WeaponType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}