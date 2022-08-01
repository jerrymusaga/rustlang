struct Guess {
    secret_number: u32
}

impl Guess {
    fn new(secret_number: u32) -> Self {
        Self {
            secret_number
        }
    }

    fn check_guess(&self, guess: u32) {
        if guess == self.secret_number{
            println!("correct")
        }
    }
}

fn main() {
    let my_guess = Guess::new(50);
    my_guess.check_guess(50);
}
