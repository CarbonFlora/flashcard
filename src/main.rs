use flashcard::features::*;

fn main() {
    let deck = read_inputs();
    let mut i = true;
    while i {
        i = read_flashcards(&deck);
    }
}