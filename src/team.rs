use super::{
    util::ask,
    character::Character
};

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

    pub fn get_formatted_chars(&self) -> String {
        let mut s: String = String::new();
        for (i, c) in self.characters.iter().enumerate() {
            s.push_str(&*format!(
                "\n\t{i}. {char_type} character -> {name}: {life}, attack with {weapon}",
                i = i + 1,
                name = c.name,
                char_type = c.char_type,
                life = c.format_life(),
                weapon = c.weapon.w_type.get_name()
            ))
        }
        s
    }

    pub fn print_comp(&self) {
        println!("The team {} is composed of:{}", self.name, self.get_formatted_chars());
    }

    pub fn ask_which_character_i(&self) -> usize {
        let parse = |input: &String| {
            match input.parse::<usize>() {
                Ok(val) => val,
                Err(_) => {
                    println!("'{}' is not a valid number!", input);
                    0usize
                }
            }
        };

        let input = parse(&ask(
            || print!(
                "Team {}, which character do you want to use? {}\nNumber: ",
                self.name,
                self.get_formatted_chars(),
            ),
            |str| {
                (1..=self.characters.len())
                    .contains(&parse(str))
            },
            |str| println!("'{}' is not a valid number!", str),
        ).unwrap());

        input - 1
    }
}