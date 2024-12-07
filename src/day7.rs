use std::collections::VecDeque;

#[derive(Debug)]
enum Operations {
    Add,
    Multiply,
}

impl std::fmt::Display for Operations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operations::Add => write!(f, "+"),
            Operations::Multiply => write!(f, "*"),
        }
    }
}

fn parse(input: &str) -> Vec<(u64, Vec<u32>)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (target, numbers) = line.split_once(": ").unwrap();

            (
                target.parse().unwrap(),
                numbers.split(' ').map(|n| n.parse().unwrap()).collect(),
            )
        })
        .collect()
}

pub fn part1(input: &str) -> u64 {
    let input = parse(input);

    input
        .into_iter()
        .map(|(target, numbers)| {
            // Content is (operation, current_total, current_index)
            let mut operations_queue = VecDeque::with_capacity(4 * numbers.len());

            operations_queue.push_back((
                Operations::Add,
                numbers[0] as u64,
                0,
                format!("{}", numbers[0]),
            ));
            operations_queue.push_back((
                Operations::Multiply,
                numbers[0] as u64,
                0,
                format!("{}", numbers[0]),
            ));

            while !operations_queue.is_empty() {
                let (operation, current_value, current_index, history) =
                    operations_queue.pop_back().unwrap();

                let next_index = current_index + 1;

                if next_index == numbers.len() {
                    // We reached the end, it's joever for this branch
                    continue;
                }

                let new_value = match operation {
                    Operations::Add => current_value + numbers[next_index] as u64,
                    Operations::Multiply => current_value * numbers[next_index] as u64,
                };

                let history_string = format!("({history}) {operation} {}", numbers[next_index]);

                if new_value == target {
                    println!("Found target: {history_string} = {target}",);
                    return target;
                }

                if new_value < target {
                    operations_queue.push_back((
                        Operations::Add,
                        new_value,
                        next_index,
                        history_string.clone(),
                    ));
                    operations_queue.push_back((
                        Operations::Multiply,
                        new_value,
                        next_index,
                        history_string,
                    ));
                }
            }

            0
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn example() {
        assert_eq!(part1(EXAMPLE), 3749);
        assert_eq!(part2(EXAMPLE), 0);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 0);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, 0);
    }
}
