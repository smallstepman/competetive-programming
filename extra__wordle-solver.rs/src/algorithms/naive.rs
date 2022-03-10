use crate::{Correctness, Guess, Guesser};

pub struct Naive;

impl Naive {
    pub fn new() -> Self {
        Naive
    }
}

impl Guesser for Naive {
    fn guess(&mut self, guesses: &[Guess]) -> String {
        todo!()
    }
}
