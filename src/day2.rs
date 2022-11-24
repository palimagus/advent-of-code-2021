use aoc_runner_derive::{aoc, aoc_generator};

// Parse each line to a tuple of (str, int32)
#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<(String, i32)> {
    input.lines().map(|l| {
        let mut split = l.split(" ");
        let s = split.next().unwrap().to_string();
        let i = split.next().unwrap().parse().unwrap();
        (s, i)
    }).collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(String, i32)]) -> i32 {
    // Horizontal position starts at 0
    let mut x = 0;
    // Depth starts at 0
    let mut depth = 0;
    // For each input:
    for (s, i) in input {
        // If String is 'forward', add i to depth
        if s == "forward" {
            x += i;
        }
        if s == "up" {
            depth -= i;
        }
        if s == "down" {
            depth += i;
        }
    }
    x * depth
}

#[aoc(day2, part2)]
fn part2(input: &[(String, i32)]) -> i32 {
    // Horizontal position starts at 0
    let mut x = 0;
    // Depth starts at 0
    let mut depth = 0;
    // Aim starts at 0
    let mut aim = 0;
    // For each input:
    for (s, i) in input {
        // If String is 'forward', add i to depth
        if s == "forward" {
            x += i;
            depth += aim * i;
        }
        if s == "up" {
            aim -= i;
        }
        if s == "down" {
            aim += i;
        }
    }
    x * depth
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&[
            ("forward".to_string(), 5),
            ("down".to_string(), 5),
            ("forward".to_string(), 8), 
            ("up".to_string(), 3),
            ("down".to_string(), 8),
            ("forward".to_string(), 2),
        ]), 150);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&[
            ("forward".to_string(), 5),
            ("down".to_string(), 5),
            ("forward".to_string(), 8), 
            ("up".to_string(), 3),
            ("down".to_string(), 8),
            ("forward".to_string(), 2),
        ]), 900);
    }
}