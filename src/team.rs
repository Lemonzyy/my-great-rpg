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
        let team_names: Vec<_> = self.characters.iter().map(|c| c.name.clone()).collect();

        println!("{}: {}", self.name, team_names.join(", "));
    }
}