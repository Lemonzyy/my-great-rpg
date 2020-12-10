use std::io::{self, Write};
use std::time::Duration;

use ordinal::Ordinal;

use super::character::CharacterType;
use super::team::Team;
use super::util::read_line;

const TEAMS_NUMBER: u8 = 3;
const CHARACTERS_NUMBER: u8 = 3;

pub struct Game {
    teams: Vec<Team>,
    current_team_i: u8,
}

impl Game {
    pub fn new() -> Self {
        Game {
            teams: vec![],
            current_team_i: 0,
        }
    }

    pub fn init(&mut self) -> &mut Self {
        for i in 0u8..TEAMS_NUMBER {
            let team_name = self.ask_team_name(i);
            let mut characters = vec![];

            for j in 0u8..CHARACTERS_NUMBER {
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
        self
    }

    pub fn start(&mut self) -> &Self {
        while self.alive_teams() > 1 {
            let current_team = self.get_current_team();
            current_team.print_comp();

            self.next_team();
        }

        self
    }

    fn get_current_team(&self) -> &Team {
        &self.teams[self.current_team_i as usize]
    }

    fn next_team(&mut self) {
        let max = self.teams.len();

        if self.current_team_i + 1 == max as u8 {
            self.current_team_i = 0
        } else {
            self.current_team_i += 1
        };
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

    fn alive_teams(&self) -> u8 {
        let mut total: u8 = 0;

        for team in self.teams.iter() {
            if team.is_alive() { total += 1 }
        }

        total
    }
}