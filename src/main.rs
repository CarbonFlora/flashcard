use flcard::features::*;

fn main() {
    let state = read_inputs();
    match state {
        Err(err) => eprintln!("{:?}", err),
        Ok(mut deck) => {
            let mut i = true;
            while i {
                (i, deck) = read_flashcards(deck);
            }
        },
    }
}