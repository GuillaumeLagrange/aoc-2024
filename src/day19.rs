use arrayvec::ArrayVec;
use std::collections::HashMap;

type Num = u32;

const TOWELS_SIZE: usize = 1000;
const PATTERNS_SIZE: usize = 500;

pub fn parse(input: &str) -> (ArrayVec<&str, TOWELS_SIZE>, ArrayVec<&str, PATTERNS_SIZE>) {
    let mut lines = input.lines();

    let towels = lines
        .next()
        .unwrap()
        .split(", ")
        .collect::<ArrayVec<_, TOWELS_SIZE>>();

    lines.next();

    (towels, lines.collect())
}

pub fn part1(input: &str) -> Num {
    let (towels, patterns) = parse(input);

    fn is_solvable<'a>(
        pattern: &'a str,
        towels: &ArrayVec<&str, TOWELS_SIZE>,
        cache: &mut HashMap<&'a str, bool>,
    ) -> bool {
        if let Some(&result) = cache.get(pattern) {
            return result;
        }

        if pattern.is_empty() {
            return true;
        }

        let ret = towels.iter().any(|towel| {
            if towel.len() > pattern.len() {
                return false;
            }

            pattern.starts_with(towel) && is_solvable(&pattern[towel.len()..], towels, cache)
        });

        cache.insert(pattern, ret);

        ret
    }

    let mut cache = HashMap::new();
    patterns
        .iter()
        .map(|pattern| {
            if is_solvable(pattern, &towels, &mut cache) {
                1
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let (towels, patterns) = parse(input);

    fn count_combinations(pattern: &str, towels: &ArrayVec<&str, TOWELS_SIZE>) -> usize {
        let mut dp = vec![0; pattern.len() + 1];
        dp[0] = 1;

        for i in 0..pattern.len() {
            if dp[i] > 0 {
                for towel in towels.iter() {
                    if i + towel.len() <= pattern.len() && &pattern[i..i + towel.len()] == *towel {
                        dp[i + towel.len()] += dp[i];
                    }
                }
            }
        }

        dp[pattern.len()]
    }

    patterns
        .iter()
        .map(|pattern| count_combinations(pattern, &towels))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = indoc::indoc! {"
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"};

    #[test]
    fn example() {
        assert_eq!(part1(EXAMPLE), 6);
        assert_eq!(part2(EXAMPLE), 16);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 327);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, 772696486795255);
    }
}
