mod solution {
    use std::ops::RangeInclusive;

    pub struct Pair(RangeInclusive<u32>, RangeInclusive<u32>);

    impl Pair {
        pub fn new(range: ((u32, u32), (u32, u32))) -> Self {
            Self((range.0).0..=(range.0).1, (range.1).0..=(range.1).1)
        }

        pub fn fully_contains(&self) -> bool {
            (self.0.start() <= self.1.start() && self.0.end() >= self.1.end())
                || (self.0.start() >= self.1.start() && self.0.end() <= self.1.end())
        }
    }
}

fn solve(input: &str) -> String {
    input
        .lines()
        .filter_map(|pair| {
            let pairs = pair.split_once(',')?;
            let ranges = (pairs.0.split_once('-')?, pairs.1.split_once('-')?);

            solution::Pair::new((
                ((ranges.0).0.parse().ok()?, (ranges.0).1.parse().ok()?),
                ((ranges.1).0.parse().ok()?, (ranges.1).1.parse().ok()?),
            ))
            .fully_contains()
            .then_some(..)
        })
        .count()
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
            "2".to_string()
        )
    }
}
