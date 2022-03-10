pub fn play<G: Guesser>(answer: &'static str, mut guesser: G) -> Option<usize> {
    // play six rounds where it invokes gguesser each round
    let mut history = Vec::new();
    for i in 1..32 {
        let guess = guesser.guess(&history[..]);
        if guess == answer {
            return Some(i);
        }
        let correctness = Correctness::compute(answer, &guess);
        history.push(Guess {
            word: guess,
            mask: correctness,
        });
    }
    None
}

pub struct Guess {
    pub word: String,
    pub mask: [Correctness; 5],
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Correctness {
    Correct,
    Misplaced,
    Wrong,
}

impl Correctness {
    fn compute(answer: &str, guess: &str) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);
        let mut c = [Correctness::Wrong; 5];
        let mut used = [false; 5];
        answer
            .chars()
            .enumerate()
            .zip(guess.chars().enumerate())
            .enumerate()
            .for_each(|(idx, zipped)| match zipped {
                (a, b) if a.1 == b.1 => {
                    c[idx] = Correctness::Correct;
                    used[idx] = true;
                }
                (_, b) if !answer.contains(b.1) => {
                    c[idx] = Correctness::Wrong;
                }
                // (a, b) => {
                //     let already_guessed: bool = { true };
                //     c[idx] = if already_guessed {
                //         Correctness::Misplaced
                //     } else {
                //         Correctness::Wrong
                //     }
                // }
                _ => {}
            });
        for (i, g) in guess.chars().enumerate() {
            if c[i] != Correctness::Correct
                && answer.chars().enumerate().any(|(j, a)| {
                    if a == g && !used[j] {
                        used[i] = true;
                        return true;
                    }
                    false
                })
            {
                c[i] = Correctness::Misplaced;
            }
        }
        c
    }
}

pub trait Guesser {
    fn guess(&mut self, _guesses: &[Guess]) -> String {
        todo!()
    }
}

impl<T> Guesser for &mut T where T: Guesser {}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! mask {
	    (C) => {Correctness::Correct};
	    (M) => {Correctness::Misplaced};
	    (W) => {Correctness::Wrong};
        ($($c:ident)+) => {[
            $(mask!($c)),+
        ]};
    }

    #[test]
    fn basic() {
        assert_eq!(Correctness::compute("abcde", "abcde"), mask![C C C C C]);
    }
    #[test]
    fn repetitions_one_correct() {
        assert_eq!(Correctness::compute("abcde", "aaaaa"), mask![C W W W W]);
    }
    #[test]
    fn repetitions_one_incorrect_rest_misplaced() {
        assert_eq!(Correctness::compute("abcde", "xaaaa"), mask![W M W W W]);
    }
}
