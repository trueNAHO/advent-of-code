use itertools::Itertools;

fn solve(input: &str) -> String {
    input
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(|x| x.parse::<i32>().expect("Failed to parse line to i32"))
                .sum::<i32>()
        })
        .sorted_unstable()
        .rev()
        .take(3)
        .sum::<i32>()
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
            "45000".to_string()
        )
    }
}
