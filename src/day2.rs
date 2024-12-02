use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut line_numbers = line
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .peekable();

            let first = line_numbers.next().unwrap();
            let second = line_numbers.peek().unwrap();

            let increasing = first < *second;

            let diff = second.abs_diff(first);
            if !(1..=3).contains(&diff) {
                return 0;
            }

            while let Some(first) = line_numbers.next() {
                let Some(second) = line_numbers.peek() else {
                    return 1;
                };

                match (increasing, first < *second) {
                    (true, false) => return 0,
                    (false, true) => return 0,
                    _ => {}
                };

                let diff = second.abs_diff(first);
                if !(1..=3).contains(&diff) {
                    return 0;
                }
            }

            panic!("Should not reach here");
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let line_numbers = line
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            for i in 0..line_numbers.len() {
                let mut line_numbers = line_numbers.clone();
                line_numbers.remove(i);

                let mut line_numbers = line_numbers.into_iter().peekable();
                let first = line_numbers.next().unwrap();
                let second = line_numbers.peek().unwrap();

                let increasing = first < *second;

                let diff = second.abs_diff(first);
                if !(1..4).contains(&diff) {
                    continue;
                }

                while let Some(first) = line_numbers.next() {
                    let Some(second) = line_numbers.peek() else {
                        return 1;
                    };

                    match (increasing, first < *second) {
                        (true, false) => break,
                        (false, true) => break,
                        _ => {}
                    };

                    let diff = second.abs_diff(first);
                    if !(1..4).contains(&diff) {
                        break;
                    }
                }
            }

            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let example = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

        assert_eq!(part1(example), 2);
        assert_eq!(part2(example), 4);
    }

    #[test]
    fn part2_debugging() {
        let should_match = "\
1 3 2 4 5
";

        let should_not_match = "\
1 2 7 8 9
9 7 6 2 1
";

        assert_eq!(part2(should_match), 1);
        assert_eq!(part2(should_not_match), 0);
    }
}
