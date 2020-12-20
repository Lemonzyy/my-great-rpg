use std::io::stdout;

use crossterm::{
    QueueableCommand,
    style::{self, Colorize}
};
use crossterm::style::Styler;

use crate::{
    character::Character,
    util::ask,
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

    pub fn print_formatted_chars(&self) {
        for (i, c) in self.characters.iter().enumerate() {
            let text = format!(
                "{i}. {char_type} character -> {name}: {life}, attack with {weapon} ({damage} damage/attack)\n",
                i = i + 1,
                name = c.name,
                char_type = c.char_type,
                life = c.format_life(),
                weapon = c.weapon.name,
                damage = c.weapon.damage
            );

            print!("\t");
            stdout()
                .queue(style::PrintStyledContent(
                    if c.is_alive() {
                        text.green()
                    } else {
                        text.crossed_out().red()
                    }
                ))
                .expect("Error while printing a styled message");
        }
    }

    pub fn print_comp(&self) {
        println!("\nThe team {} is composed of:", self.name);
        self.print_formatted_chars();
    }

    pub fn ask_which_character_i(&self) -> usize {
        let parse = |input: &String| {
            match input.parse::<usize>() {
                Ok(val) => val,
                Err(_) => 0usize
            }
        };

        let input = parse(&ask(
            || {
                println!(
                    "Team {}, which character do you want to use?",
                    self.name,
                );
                self.print_formatted_chars();
                print!("Number: ")
            },
            |str| {
                (1..=self.characters.len())
                    .contains(&parse(str))
            },
            |str| println!("'{}' is not a valid number!", str),
        ).unwrap());

        input - 1
    }
}