struct Guess<T> {
    secret_number: T
}

impl<T:Ord> Guess<T> {
    fn new(secret_number: T) -> Guess<T> {
        Self {
            secret_number
        }
    }

    fn check_guess(&self, guess: T) {
        if guess == self.secret_number{
            println!("correct")
        }
    }
}

fn main() {
    let my_guess = Guess::new(50);
    my_guess.check_guess(50);
}
