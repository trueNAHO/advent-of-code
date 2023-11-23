use itertools::Itertools;
use std::collections::HashSet;

fn solve(input: &str) -> String {
    const WINDOW_SIZE: usize = 14;

    let mut set = HashSet::<char>::with_capacity(WINDOW_SIZE);

    input
        .chars()
        .collect_vec()
        .windows(WINDOW_SIZE)
        .enumerate()
        .find_position(|(_, window)| {
            set.clear();

            window.iter().for_each(|c| {
                set.insert(*c);
            });

            set.len() == WINDOW_SIZE
        })
        .map(|(i, _)| i + WINDOW_SIZE)
        .expect("Input should provide valid solution")
        .to_string()
}

fn main() {
    println!("{}", solve(include_str!("../../input.txt")))
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_1() {
        assert_eq!(
            super::solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            "19".to_string()
        )
    }

    #[test]
    fn sample_2() {
        assert_eq!(
            super::solve("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            "23".to_string()
        )
    }

    #[test]
    fn sample_3() {
        assert_eq!(
            super::solve("nppdvjthqldpwncqszvftbrmjlhg"),
            "23".to_string()
        )
    }

    #[test]
    fn sample_4() {
        assert_eq!(
            super::solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            "29".to_string()
        )
    }

    #[test]
    fn sample_5() {
        assert_eq!(
            super::solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            "26".to_string()
        )
    }
}
