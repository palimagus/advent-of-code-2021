use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<u16> {
    input.lines().map(|l| l.trim().parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn solve_part1(input: &[u16]) -> u16 {
    // If past value is greater or equal to current value, status is "decreasing"
    // Else status is "increasing"
    // Count the number of increasing
    // let mut count = 0;
    // for i in 1..input.len() {
    //     if input[i - 1] < input[i] {
    //         count += 1;
    //     }
    // }
    // count
    input.windows(2).map(|w| (w[1] > w[0]) as u16).sum()
}

#[aoc(day1, part2)]
fn solve_part2(input: &[u16]) -> u16 {
    solve_part1(
        &input
            .windows(3)
            .map(|w| w.iter().sum())
            .collect::<Vec<u16>>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&[1, 2, 3, 4, 5]), 4);
        assert_eq!(solve_part1(&[5, 4, 3, 2, 1]), 0);
        assert_eq!(solve_part1(&[1, 2, 3, 4, 5, 4, 3, 2, 1]), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            solve_part2(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            5
        );
    }
}
