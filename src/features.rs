use crate::args::FlashcardArgs;
use clap::Parser;
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead, self, Write};
use colored::Colorize;

type Arguments = HashMap<String, String>;
const ALL_DELIMITERS: [char; 8] = ['|', '\\', '/', '(', '[', '{', '<', '-'];
const CLOSING_DELIMITERS: [char; 4] = [')', ']', '}', '>'];

pub fn read_inputs() -> Result<Arguments> {
    let args = FlashcardArgs::parse();
    let mut deck: Arguments = HashMap::new();
    for file_path in args.card_stack {
        let mut buffered = BufReader::new(File::open(file_path)?).lines().flatten().peekable();
        let seperator = examine_seperator(buffered.peek())?;

        for line in buffered {
            let i = line.split_once(seperator);
            match i {
                None => eprintln!("Skipped line: {line}"),
                Some(args) => {
                    deck.insert(args.0.to_string(), clean_last(args.1.to_string()));
                },
            };
        }
        
    }

    Ok(deck)
}

fn examine_seperator(line: Option<&String>) -> Result<char> {
    if let Some(line) = line {
        for delimiter in ALL_DELIMITERS {
            if let Some(char) = line.chars().find(|x| *x == delimiter) {
                return Ok(char)
            }
        }
    }
    Err(anyhow!("No delimiter was found. Please use one of the following as a delimiter:\n{:?}", ALL_DELIMITERS))
}

fn clean_last(mut arg2: String) -> String {
    for i in CLOSING_DELIMITERS {
        if arg2.ends_with(i) {
            arg2.pop();
        }
    }
    arg2
}

pub fn read_flashcards(deck: Arguments) -> (bool, Arguments) {
    // Body
    let deck = random_shuffle(deck);

    // Conclusion
    println!("\nFinished studying the deck!\nWould you like to complete another iteration? Y/N");
    let input = io::stdin().lock().lines().next().unwrap().unwrap().chars().next().unwrap_or('y');
    if input.to_lowercase().to_string() == "n".to_string() {
        return (false, deck);
    }
    (true, deck)
}

fn random_shuffle(mut deck: Arguments) -> Arguments {
    let args = FlashcardArgs::parse();
    for mut card in &deck.clone() {
        if args.reverse {
            let i = card.0;
            card.0 = card.1;
            card.1 = i;
        }
        println!("{} {} ", "?".yellow(), card.0);
        print!("{} ", "~".yellow());
        let _ = io::stdout().flush();
        let mut pause = String::new();
        let _ = io::stdin().read_line(&mut pause);
        println!("{} {}", ">".yellow(), card.1.bright_green());
        if !args.maintain {
            print!("{} ", "Type anything to remove.".red());
            let _ = io::stdout().flush();
            let input = io::stdin().lock().lines().next().unwrap().unwrap().chars().next().unwrap_or(' ');
            if !(input.to_string().is_empty() || input == ' ') {
                deck.remove(card.0);
            }
        }
    }
    deck
}