use super::character::Character;

pub struct Team {
    pub(crate) name: String,
    pub(crate) characters: Vec<Character>,
}

impl Team {
    pub fn is_alive(&self) -> bool {
        let mut alive = false;

        for character in self.characters.iter() {
            if character.is_alive() { alive = true }
        }

        alive
    }

    pub fn print_comp(&self) {
        let mut s: String = String::new();

        for (i, c) in self.characters.iter().enumerate() {
            s.push_str(&*format!(
                "\n\t{i}. {char_type} character -> {name}: {life} of life, attack with {weapon}",
                i = i + 1,
                name = c.name,
                char_type = c.char_type,
                life = c.format_life(),
                weapon = c.weapon.w_type.get_name()
            ))
        }

        println!("The team {} is composed of:{}", self.name, s);
    }
}