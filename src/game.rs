use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

use ordinal::Ordinal;

use super::{
    character::CharacterType,
    team::Team,
    util::{ask, clear_console, print_title, read_line},
};

const TEAMS_NUMBER: u8 = 2;
const CHARACTERS_NUMBER: u8 = 1;
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

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
        clear_console();
        print_title(format!("Welcome to the My Great RPG game ! v{}", VERSION.unwrap_or("unknown")).as_str());

        for i in 0..TEAMS_NUMBER {
            let team_name = ask(
                || print!("\n{} team name: ", Ordinal(i + 1)),
                |_| true,
                |_| {},
            ).unwrap();

            let mut characters = vec![];

            for j in 0..CHARACTERS_NUMBER {
                let character_type = match &*ask(
                    || print!("\
What's gonna be your {} character?
\t1. soldier
\t2. mage
\t3. archer
\t4. colossus
Number: ", Ordinal(j + 1)),
                    |str| {
                        match &str.parse::<u8>() {
                            Ok(val) => (1..=4).contains(val),
                            Err(_) => false
                        }
                    },
                    |str| println!("'{}' is not a valid number!", str),
                ).unwrap() {
                    "1" => CharacterType::Soldier,
                    "2" => CharacterType::Mage,
                    "3" => CharacterType::Archer,
                    "4" | _ => CharacterType::Colossus
                };

                let name = ask(
                    || print!("What's gonna be his name? "),
                    |_| true,
                    |_| {},
                ).unwrap();

                let character = character_type.get_character(name);

                characters.push(character);
            }

            self.teams.push(Team {
                name: team_name,
                characters,
            })
        }

        clear_console();
        print_title("Recap time!");
        println!();

        sleep(Duration::from_millis(500));

        for team in self.teams.iter() {
            team.print_comp();
            sleep(Duration::from_secs(1));
        }

        println!("\nPress any key to start!");
        read_line();

        self
    }

    pub fn start(&mut self) -> &Self {
        while self.alive_teams() > 1 {
            clear_console();

            let current_team = self.get_current_team();
            println!("current team name: {}", current_team.name);

            if current_team.is_alive() {
                let character = current_team.ask_which_character();
                let next_team = self.get_next_team();
                character.attack_team(next_team);

                io::stdout().flush().expect("Error while stdout flushing");
            }

            self.next_team();
        }

        let remaining_teams: Vec<&Team> = self.teams
            .iter()
            .filter(|&team| team.is_alive() == true)
            .collect();

        if !remaining_teams.is_empty() {
            println!("The team {} won the game!", remaining_teams[0].name)
        } else {
            println!("None of the teams won the game!")
        }

        self
    }

    fn get_current_team(&self) -> &Team {
        &self.teams[self.current_team_i as usize]
    }

    fn get_next_team(&mut self) -> &mut Team {
        let i = self.get_next_team_i();
        &mut self.teams[i as usize]
    }

    fn get_next_team_i(&self) -> u8 {
        let max = self.teams.len();

        if self.current_team_i + 1 == max as u8 {
            0
        } else {
            self.current_team_i + 1
        }
    }

    fn next_team(&mut self) {
        self.current_team_i = self.get_next_team_i()
    }

    fn alive_teams(&self) -> u8 {
        let mut total: u8 = 0;

        for team in self.teams.iter() {
            if team.is_alive() { total += 1 }
        }

        total
    }
}