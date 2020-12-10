pub struct Weapon {
    name: String,
    damage: i32,
}

impl Weapon {
    pub fn new(name: String, damage: i32) -> Result<Self, String> {
        if damage == 0 {
            return Err(format!("Weapon `{}` should not have a damage of {}", name, damage));
        }

        Ok(Self {
            name,
            damage,
        })
    }

    pub fn is_offensive(&self) -> bool {
        self.damage > 0
    }
}