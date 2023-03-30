use crate::args::FlashcardArgs;
use clap::Parser;

pub fn read_inputs() -> Vec<String>{
    let args = FlashcardArgs::parse();

    args.card_stack
}