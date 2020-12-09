use std::io::{self, Write};
use super::team::Team;
use super::util::read_line;
use super::character::CharacterType;
use ordinal::Ordinal;

pub struct Game {
    teams: Vec<Team>,
    is_first_team: bool,
}

impl Game {
    pub fn new() -> Self {
        Game {
            teams: vec![],
            is_first_team: true,
        }
    }

    pub fn init(&mut self) {
        for i in 0u8..=1u8 {
            let team_name = self.ask_team_name(i);
            let mut characters = vec![];

            for j in 0u8..3u8 {
                let character_type = self.ask_character_type(j);
                let name = self.ask_character_name();
                let character = character_type.get_character(name);

                characters.push(character);
            }

            self.teams.push(Team {
                name: team_name,
                characters,
            })
        }
    }

    fn ask_team_name(&self, team_index: u8) -> String {
        let mut team_name;
        loop {
            print!("{} team name: ", Ordinal(team_index + 1));
            io::stdout().flush().expect("Error while stdout flushing");

            team_name = read_line();

            if !team_name.is_empty() { break; }
        }
        team_name
    }

    fn ask_character_type(&self, character_index: u8) -> CharacterType {
        let mut char_type;
        loop {
            print!("\
What's gonna be your {} character ?
    1. soldier
    2. mage
    3. archer
    4. colossus
Number: ", Ordinal(character_index + 1));
            io::stdout().flush().expect("Error while stdout flushing");
            char_type = read_line();

            if !char_type.is_empty() && (1..=4).contains(&char_type.parse::<u8>().unwrap()) { break; }
        }

        match &*char_type {
            "1" => CharacterType::Soldier,
            "2" => CharacterType::Mage,
            "3" => CharacterType::Archer,
            "4" | _ => CharacterType::Colossus
        }
    }

    fn ask_character_name(&self) -> String {
        let mut name;
        loop {
            print!("What's gonna be his name ? ");
            io::stdout().flush().expect("Error while stdout flushing");
            name = read_line();

            if !name.is_empty() { break; }
        }
        name
    }

    pub fn start(&self) {
        unimplemented!()
    }
}