use itertools::Itertools;

mod solution {
    use std::{char, collections::HashSet};
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum InputError {
        #[error("Item must match the regex [a-zA-Z]")]
        ItemInvalidError,

        #[error("Cannot split Rucksack into two equally sized compartments")]
        RucksackSplitError,
    }

    pub struct Group<'a>(Rucksack<'a>, Rucksack<'a>, Rucksack<'a>);

    impl<'a> Group<'a> {
        pub fn new(rucksacks: (&'a str, &'a str, &'a str)) -> Result<Self, InputError> {
            Ok(Self(
                Rucksack::new(rucksacks.0)?,
                Rucksack::new(rucksacks.1)?,
                Rucksack::new(rucksacks.2)?,
            ))
        }

        pub fn priority(&self) -> Result<u32, InputError> {
            let sets = [
                (self.0).0.chars().collect::<HashSet<_>>(),
                (self.1).0.chars().collect::<HashSet<_>>(),
                (self.2).0.chars().collect::<HashSet<_>>(),
            ];

            sets.iter()
                .fold(sets[0].clone(), |mut intersection, set| {
                    intersection.retain(|item| set.contains(item));
                    intersection
                })
                .into_iter()
                .map(|item| Ok(Item::try_from(&item)?.0))
                .sum()
        }
    }

    struct Rucksack<'a>(&'a str);

    impl<'a> Rucksack<'a> {
        fn new(items: &'a str) -> Result<Self, InputError> {
            if items.len() % 2 != 0 {
                return Err(InputError::RucksackSplitError);
            }

            Ok(Self(items))
        }
    }

    struct Item(u32);

    impl TryFrom<&char> for Item {
        type Error = InputError;

        fn try_from(value: &char) -> Result<Self, Self::Error> {
            Ok(Self(match value {
                'A' => Ok(27),
                'B' => Ok(28),
                'C' => Ok(29),
                'D' => Ok(30),
                'E' => Ok(31),
                'F' => Ok(32),
                'G' => Ok(33),
                'H' => Ok(34),
                'I' => Ok(35),
                'J' => Ok(36),
                'K' => Ok(37),
                'L' => Ok(38),
                'M' => Ok(39),
                'N' => Ok(40),
                'O' => Ok(41),
                'P' => Ok(42),
                'Q' => Ok(43),
                'R' => Ok(44),
                'S' => Ok(45),
                'T' => Ok(46),
                'U' => Ok(47),
                'V' => Ok(48),
                'W' => Ok(49),
                'X' => Ok(50),
                'Y' => Ok(51),
                'Z' => Ok(52),
                'a' => Ok(1),
                'b' => Ok(2),
                'c' => Ok(3),
                'd' => Ok(4),
                'e' => Ok(5),
                'f' => Ok(6),
                'g' => Ok(7),
                'h' => Ok(8),
                'i' => Ok(9),
                'j' => Ok(10),
                'k' => Ok(11),
                'l' => Ok(12),
                'm' => Ok(13),
                'n' => Ok(14),
                'o' => Ok(15),
                'p' => Ok(16),
                'q' => Ok(17),
                'r' => Ok(18),
                's' => Ok(19),
                't' => Ok(20),
                'u' => Ok(21),
                'v' => Ok(22),
                'w' => Ok(23),
                'x' => Ok(24),
                'y' => Ok(25),
                'z' => Ok(26),
                _ => Err(InputError::ItemInvalidError),
            }?))
        }
    }
}

fn solve(input: &str) -> Result<String, solution::InputError> {
    Ok(input
        .lines()
        .tuples()
        .map(|rucksacks: (_, _, _)| solution::Group::new(rucksacks)?.priority())
        .sum::<Result<u32, solution::InputError>>()?
        .to_string())
}

fn main() -> Result<(), solution::InputError> {
    Ok(println!("{}", solve(include_str!("../../input.txt"))?))
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample() -> Result<(), super::solution::InputError> {
        Ok(assert_eq!(
            super::solve(include_str!("../../sample.txt"))?,
            "70".to_string()
        ))
    }
}
