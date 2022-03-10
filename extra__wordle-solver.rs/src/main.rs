mod algorithms;
mod lib;

use algorithms::Naive;
use lib::*;

const GAMES: &str = include_str!("../answers.txt");

fn main() {
    for answer in GAMES.split_whitespace() {
        let mut guesser = Naive::new();
        play(answer, guesser);
    }
    println!("Hello, world!");
}
