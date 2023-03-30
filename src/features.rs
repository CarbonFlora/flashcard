use crate::args::FlashcardArgs;
use clap::Parser;
use anyhow::{Result, anyhow};
use std::fs::File;
use std::io::{BufReader, BufRead, self};

type Arguments = (Vec<String>, Vec<String>);
const ALL_DELIMITERS: [char; 8] = ['|', '\\', '/', '(', '[', '{', '<', '-'];
const CLOSING_DELIMITERS: [char; 4] = [')', ']', '}', '>'];

pub fn read_inputs() -> Result<Arguments> {
    let args = FlashcardArgs::parse();
    let mut deck: Arguments = (Vec::new(), Vec::new());
    for file_path in args.card_stack {
        let mut buffered = BufReader::new(File::open(file_path)?).lines().flatten().peekable();
        let seperator = examine_seperator(buffered.peek())?;

        for line in buffered {
            let i = line.split_once(seperator);
            match i {
                None => eprintln!("Skipped line: {line}"),
                Some(args) => {
                    deck.0.push(args.0.to_string());
                    deck.1.push(clean_last(args.1.to_string()));},
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

pub fn read_flashcards(deck: &Result<Arguments>) -> bool {
    match deck {
        Ok(deck) => println!("{:#?}", deck),
        Err(err) => eprintln!("{:?}", err),
    }

    println!("\nFinished studying the deck!\nWould you like to complete another iteration? Y/N");
    let input = io::stdin().lock().lines().next().unwrap().unwrap().chars().next().unwrap_or('y');
    if input.to_lowercase().to_string() == "n".to_string() {
        return false;
    }
    true
}