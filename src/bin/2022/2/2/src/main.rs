mod solution {
    use itertools::Itertools;
    use std::str::FromStr;

    pub struct Instruction {
        pub action: Action,
        pub outcome: Outcome,
    }

    impl FromStr for Instruction {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (action, outcome) = s
                .split(' ')
                .collect_tuple()
                .ok_or("Should have exactly two space separated fields")?;

            Ok(Instruction {
                action: action.parse()?,
                outcome: outcome.parse()?,
            })
        }
    }

    impl Instruction {
        pub fn score(&self) -> u16 {
            self.action.action(&self.outcome) as u16 + self.outcome.score() as u16
        }
    }

    pub enum Action {
        Paper = 2,
        Rock = 1,
        Scissors = 3,
    }

    impl FromStr for Action {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "A" => Ok(Action::Rock),
                "B" => Ok(Action::Paper),
                "C" => Ok(Action::Scissors),
                _ => Err(format!("Invalid action: {}", s)),
            }
        }
    }

    impl Action {
        pub fn action(&self, outcome: &Outcome) -> Self {
            match self {
                Self::Paper => match outcome {
                    Outcome::Draw => Self::Paper,
                    Outcome::Lose => Self::Rock,
                    Outcome::Win => Self::Scissors,
                },

                Self::Rock => match outcome {
                    Outcome::Draw => Self::Rock,
                    Outcome::Lose => Self::Scissors,
                    Outcome::Win => Self::Paper,
                },

                Self::Scissors => match outcome {
                    Outcome::Draw => Self::Scissors,
                    Outcome::Lose => Self::Paper,
                    Outcome::Win => Self::Rock,
                },
            }
        }
    }

    pub enum Outcome {
        Draw,
        Lose,
        Win,
    }

    impl FromStr for Outcome {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "X" => Ok(Outcome::Lose),
                "Y" => Ok(Outcome::Draw),
                "Z" => Ok(Outcome::Win),
                _ => Err(format!("Invalid outcome: {}", s)),
            }
        }
    }

    impl Outcome {
        pub fn score(&self) -> u8 {
            match self {
                Self::Draw => 3,
                Self::Lose => 0,
                Self::Win => 6,
            }
        }
    }
}

fn solve(input: &str) -> Result<String, String> {
    Ok(input
        .lines()
        .map(|line| Ok(line.parse::<solution::Instruction>()?.score()))
        .sum::<Result<u16, String>>()?
        .to_string())
}

fn main() -> Result<(), String> {
    Ok(println!("{}", solve(include_str!("../../input.txt"))?))
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample() -> Result<(), String> {
        Ok(assert_eq!(
            super::solve(include_str!("../../sample.txt"))?,
            "12".to_string()
        ))
    }
}
