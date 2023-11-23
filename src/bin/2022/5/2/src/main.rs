mod solution {
    use itertools::Itertools;
    use std::{num::ParseIntError, str::FromStr};
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum InputError {
        #[error("Action indexx is out of bound")]
        ActionIndexOutOfBound,

        #[error("Cargo and actions must be separated by an empty line")]
        CargoAndActionsSeparatorError,

        #[error("Cargo cannot be empty")]
        EmptyCargoError,

        #[error("Actions must contain four whitespace separators")]
        MissingActionWords,

        #[error(transparent)]
        ParseIntError(#[from] ParseIntError),
    }

    pub struct Solution {
        actions: Actions,
        cargo: Cargo,
    }

    impl Solution {
        pub fn message(&self) -> String {
            self.cargo.message()
        }

        pub fn transfer(&mut self) -> Result<(), InputError> {
            for action in &self.actions.0 {
                self.cargo
                    .transfer(action.source_stack, action.target_stack, action.amount)?;
            }

            Ok(())
        }
    }

    impl FromStr for Solution {
        type Err = InputError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (cargo, actions) = s
                .split_once("\n\n")
                .ok_or(InputError::CargoAndActionsSeparatorError)?;

            Ok(Self {
                actions: actions.parse::<Actions>()?,
                cargo: cargo.parse::<Cargo>()?,
            })
        }
    }

    struct Action {
        amount: usize,
        source_stack: usize,
        target_stack: usize,
    }

    struct Actions(Vec<Action>);

    impl FromStr for Actions {
        type Err = InputError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut actions = Vec::<Action>::with_capacity(s.len());

            for line in s.lines() {
                let (_, amount, _, source_stack, _, target_stack) = line
                    .split_ascii_whitespace()
                    .collect_tuple()
                    .ok_or(InputError::MissingActionWords)?;

                actions.push(Action {
                    amount: amount.parse::<usize>()?,
                    source_stack: source_stack.parse::<usize>()? - 1,
                    target_stack: target_stack.parse::<usize>()? - 1,
                });
            }

            Ok(Actions(actions))
        }
    }

    struct Cargo(Vec<Stack>);

    impl FromStr for Cargo {
        type Err = InputError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut s_iter_rev = s.lines().rev();

            let mut cargo = Cargo::with_capacity(
                s_iter_rev.next().ok_or(InputError::EmptyCargoError)?.len() / 4 + 1,
                s_iter_rev.clone().count(),
            );

            s_iter_rev.for_each(|line| {
                line.chars()
                    .skip(1)
                    .step_by(4)
                    .enumerate()
                    .for_each(|(i, c)| {
                        if !c.is_whitespace() {
                            cargo.0[i].0.push(Item(c));
                        }
                    });
            });

            Ok(cargo)
        }
    }

    impl Cargo {
        fn message(&self) -> String {
            self.0
                .iter()
                .filter_map(|stack| Some(stack.0.last()?.0))
                .join("")
        }

        fn transfer(
            &mut self,
            source_stack: usize,
            target_stack: usize,
            amount: usize,
        ) -> Result<(), InputError> {
            let source_range = self
                .0
                .get(source_stack)
                .ok_or(InputError::ActionIndexOutOfBound)?
                .0
                .len()
                .saturating_sub(amount)..;

            let items: Vec<_> = self
                .0
                .get_mut(source_stack)
                .ok_or(InputError::ActionIndexOutOfBound)?
                .0
                .drain(source_range)
                .collect();

            self.0
                .get_mut(target_stack)
                .ok_or(InputError::ActionIndexOutOfBound)?
                .0
                .extend(items);

            Ok(())
        }

        fn with_capacity(stacks: usize, items: usize) -> Self {
            Cargo(
                (0..stacks)
                    .map(|_| Stack(Vec::<Item>::with_capacity(items)))
                    .collect(),
            )
        }
    }

    struct Item(char);

    struct Stack(Vec<Item>);
}

fn solve(input: &str) -> Result<String, solution::InputError> {
    let mut input = input.parse::<solution::Solution>()?;
    input.transfer()?;
    Ok(input.message())
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
            "MCD".to_string()
        ))
    }
}
