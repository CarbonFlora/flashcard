use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Flashcards")]
#[command(author = "Zi Hao Liang <zihaoliang0413@gmail.com>")]
#[command(version = "1.0.1")]
#[command(about="Simple flashcard system built in Rust.", long_about = None)]
pub struct FlashcardArgs {
    /// Reverse order
    #[arg(short, long, default_value_t = false)]
    pub reverse: bool,

    /// Never skip
    #[arg(short, long, default_value_t = false)]
    pub maintain: bool,

    /// Select any number of text files
    #[arg(required = true)]
    pub card_stack: Vec<String>,
}

// /// Name of the person to greet
// #[arg(short, long, default_value_t={"0".to_string()})]
// name: String,
