mod solution {
    use std::{cmp::Ordering, str::FromStr};

    #[derive(Clone, Copy, PartialEq)]
    pub enum Move {
        Rock = 1,
        Paper = 2,
        Scissors = 3,
    }

    impl FromStr for Move {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "A" | "X" => Ok(Move::Rock),
                "B" | "Y" => Ok(Move::Paper),
                "C" | "Z" => Ok(Move::Scissors),
                _ => Err(format!("Invalid move: {}", s)),
            }
        }
    }

    impl PartialOrd for Move {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            if self == &Move::Scissors && other == &Move::Rock {
                Some(Ordering::Less)
            } else if self == &Move::Rock && other == &Move::Scissors {
                Some(Ordering::Greater)
            } else {
                Some((*self as u8).cmp(&(*other as u8)))
            }
        }
    }

    impl Move {
        pub fn win(&self, other: &Self) -> u8 {
            match self.partial_cmp(other) {
                Some(Ordering::Less) => 0,
                Some(Ordering::Greater) => 6,
                Some(Ordering::Equal) => 3,
                _ => unreachable!(),
            }
        }
    }
}

use itertools::Itertools;

fn solve(input: &str) -> String {
    input
        .lines()
        .map(|line| -> u32 {
            let (left, right) = line
                .split(' ')
                .map(|s| {
                    s.parse::<solution::Move>()
                        .expect("Input line should consist of moves")
                })
                .collect_tuple()
                .expect("Input line should have exactly two fields");

            right.win(&left) as u32 + right as u32
        })
        .sum::<u32>()
        .to_string()
}

fn main() {
    println!("{}", solve(include_str!("../../input.txt")))
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample() {
        assert_eq!(
            super::solve(include_str!("../../sample.txt")),
            "15".to_string()
        )
    }
}
