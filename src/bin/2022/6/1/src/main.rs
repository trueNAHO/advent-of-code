use itertools::Itertools;
use std::collections::HashSet;

fn solve(input: &str) -> String {
    const WINDOW_SIZE: usize = 4;

    let mut set = HashSet::<char>::with_capacity(WINDOW_SIZE);

    input
        .chars()
        .tuple_windows::<(_, _, _, _)>()
        .enumerate()
        .find_position(|(_, (a, b, c, d))| {
            set.clear();

            set.insert(*a);
            set.insert(*b);
            set.insert(*c);
            set.insert(*d);

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
            super::solve(include_str!("../../sample.txt")),
            "7".to_string()
        )
    }

    #[test]
    fn sample_2() {
        assert_eq!(
            super::solve("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            "5".to_string()
        )
    }

    #[test]
    fn sample_3() {
        assert_eq!(
            super::solve("nppdvjthqldpwncqszvftbrmjlhg"),
            "6".to_string()
        )
    }

    #[test]
    fn sample_4() {
        assert_eq!(
            super::solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            "10".to_string()
        )
    }

    #[test]
    fn sample_5() {
        assert_eq!(
            super::solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            "11".to_string()
        )
    }
}
