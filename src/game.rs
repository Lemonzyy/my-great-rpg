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
const CHARACTERS_NUMBER: u8 = 2;
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

pub struct Game {
    teams: Vec<Team>,
    current_team_i: usize,
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

            let current_team = &self.teams[self.current_team_i];

            if current_team.is_alive() {
                let character_i = loop {
                    let character_i = current_team.ask_which_character_i();

                    if current_team.characters.get(character_i).is_some() &&
                        current_team.characters.get(character_i).unwrap().is_alive() {
                        break character_i;
                    } else {
                        println!("This character is dead!")
                    }
                };

                self.attack_team(
                    self.current_team_i,
                    character_i,
                    self.get_next_team_i()
                );

                io::stdout().flush().expect("Error while stdout flushing");

                println!("\nPress any key to continue...");
                read_line();
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

    pub fn attack(
        &mut self,
        from_team_i: usize,
        from_character_i: usize,
        target_team_i: usize,
        target_character_i: usize
    ) {
        let l = self.teams[target_team_i].characters[target_character_i].life - self.teams[from_team_i].characters[from_character_i].weapon.damage;
        self.teams[target_team_i].characters[target_character_i].life = std::cmp::max(0, l);

        println!(
            "{from} attacked {target} with {weapon}! Now his life is {life}",
            from = self.teams[from_team_i].characters[from_character_i].name,
            target = self.teams[target_team_i].characters[target_character_i].name,
            weapon = self.teams[from_team_i].characters[from_character_i].weapon.w_type,
            life = self.teams[target_team_i].characters[target_character_i].format_life()
        );
    }

    pub fn attack_team(
        &mut self,
        from_team_i: usize,
        from_character_i: usize,
        target_team_i: usize
    ) {
        for target_character_i in 0..self.teams[target_team_i].characters.len() {
            if self.teams[target_team_i].characters[target_character_i].is_alive() {
                self.attack(from_team_i, from_character_i, target_team_i, target_character_i);
            }
        }
    }

    fn get_next_team_i(&self) -> usize {
        let max = self.teams.len();

        if self.current_team_i + 1 == max {
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