use aoc_runner_derive::aoc;
use regex;

use std::sync::LazyLock;

static MUL_REGEX_PART_1: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"mul\(\d+,\d+\)").unwrap());

static MUL_REGEX_PART_2: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"do\(\)|don't\(\)|mul\(\d+,\d+\)").unwrap());

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            MUL_REGEX_PART_1
                .find_iter(line)
                .map(|m| {
                    let m = m.as_str();
                    let Some((a, b)) = m[4..m.len() - 1].split_once(",") else {
                        panic!("Failed to parse")
                    };

                    a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap()
                })
                .sum::<usize>()
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let mut enabled = true;
    input
        .lines()
        .map(|line| {
            MUL_REGEX_PART_2
                .find_iter(line)
                .map(|m| {
                    let m = m.as_str();
                    match m {
                        "do()" => {
                            enabled = true;
                            0
                        }
                        "don't()" => {
                            enabled = false;
                            0
                        }
                        m => {
                            if !enabled {
                                return 0;
                            }
                            let Some((a, b)) = m[4..m.len() - 1].split_once(",") else {
                                panic!("Failed to parse")
                            };
                            a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap()
                        }
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

        assert_eq!(part1(example), 161);
    }

    #[test]
    fn part2_example() {
        let example = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

        assert_eq!(part2(example), 48);
    }
}
