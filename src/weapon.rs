use std::fmt::{self, Display, Formatter};

pub const SWORD: Weapon = Weapon::new(WeaponType::Sword, 10).unwrap();
pub const WAND: Weapon = Weapon::new(WeaponType::Wand, 10).unwrap();
pub const BOW: Weapon = Weapon::new(WeaponType::Bow, 10).unwrap();
pub const FIST_BUMP: Weapon = Weapon::new(WeaponType::FistBump, 10).unwrap();

pub struct Weapon {
    w_type: WeaponType,
    damage: i32,
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

impl Display for WeaponType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}