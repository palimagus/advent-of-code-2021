use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[i32]) -> i32 {
    // If past value is greater or equal to current value, status is "decreasing"
    // Else status is "increasing"
    // Count the number of increasing
    let mut count = 0;
    for i in 1..input.len() {
        if input[i - 1] < input[i] {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&[1, 2, 3, 4, 5]), 4);
        assert_eq!(part1(&[5, 4, 3, 2, 1]), 0);
        assert_eq!(part1(&[1, 2, 3, 4, 5, 4, 3, 2, 1]), 4);
    }
}
