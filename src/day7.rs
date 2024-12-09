use crate::utils::number_of_digits_u64;
use std::collections::VecDeque;

const SIZE: usize = 12;

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

            operations_queue.push_back((Operations::Add, numbers[0] as u64, 0));
            operations_queue.push_back((Operations::Multiply, numbers[0] as u64, 0));

            while !operations_queue.is_empty() {
                let (operation, current_value, current_index) =
                    operations_queue.pop_back().unwrap();

                let next_index = current_index + 1;

                if next_index == numbers.len() {
                    // We reached the end, it's joever for this branch
                    continue;
                }

                let next_number = numbers[next_index] as u64;

                let new_value = match operation {
                    Operations::Add => current_value + next_number,
                    Operations::Multiply => current_value * next_number,
                };

                if next_index == numbers.len() - 1 && new_value == target {
                    return target;
                }

                if new_value <= target {
                    operations_queue.push_back((Operations::Add, new_value, next_index));
                    operations_queue.push_back((Operations::Multiply, new_value, next_index));
                }
            }

            0
        })
        .sum()
}

#[inline(always)]
fn is_solvable_2(target: u64, current: u64, values: &[u64]) -> bool {
    if current > target {
        return false;
    }
    if values.len() == 1 {
        return current + values[0] == target
            || current * values[0] == target
            || current * 10_u64.pow(number_of_digits_u64(values[0])) + values[0] == target;
    }

    is_solvable_2(target, current + values[0], &values[1..])
        || is_solvable_2(target, current * values[0], &values[1..])
        || is_solvable_2(
            target,
            current * 10_u64.pow(number_of_digits_u64(values[0])) + values[0],
            &values[1..],
        )
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let (total, values_str) = l.split_once(": ").unwrap();
            let total: u64 = total.parse().unwrap();
            let mut values: Vec<u64> = Vec::with_capacity(SIZE);
            values_str
                .split_whitespace()
                .for_each(|v| values.push(v.parse().unwrap()));

            if is_solvable_2(total, values[0], &values[1..]) {
                return total;
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = indoc::indoc! {"
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    "
    };

    #[test]
    fn example() {
        assert_eq!(part1(EXAMPLE), 3749);
        assert_eq!(part2(EXAMPLE), 11387);
    }

    #[test]
    fn debug_part1() {
        let sample = "\
609210: 91 2 85 798 5 14 3
6148: 6 95 376 8 9 58 6 16 6 1
";

        assert_eq!(part1(sample), 6148);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 3312271365652);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, 509463489296712);
    }
}
