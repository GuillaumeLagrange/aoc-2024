use std::collections::HashMap;

use crate::utils::number_of_digits_u64;

type Num = u64;

fn count_steps(stone: Num, steps: usize) -> Num {
    if steps == 0 {
        return 1;
    }

    if stone == 0 {
        return count_steps(1, steps - 1);
    }

    let total_digits = number_of_digits_u64(stone);
    match total_digits & 1 {
        0 => {
            let half_digits = total_digits / 2;

            let divisor = 10u64.pow(half_digits);

            let left = stone / divisor;
            let right = stone % divisor;

            count_steps(left, steps - 1) + count_steps(right, steps - 1)
        }
        1 => {
            let next_stone = stone * 2024;
            count_steps(next_stone, steps - 1)
        }
        _ => unreachable!(),
    }
}

pub fn part1(input: &str) -> Num {
    input
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .map(|n_str| {
            let n = n_str.parse().unwrap();

            count_steps(n, 25)
        })
        .sum()
}

fn count_steps_cached(stone: Num, steps: u32, cache: &mut HashMap<(Num, u32), Num>) -> Num {
    if steps == 0 {
        return 1;
    }

    if stone == 0 {
        return count_steps_cached(1, steps - 1, cache);
    }

    if let Some(&count) = cache.get(&(stone, steps)) {
        return count;
    }

    let total_digits = number_of_digits_u64(stone);
    match total_digits & 1 {
        0 => {
            let half_digits = total_digits / 2;

            let divisor = 10u64.pow(half_digits);

            let left = stone / divisor;
            let right = stone % divisor;

            let count = count_steps_cached(left, steps - 1, cache)
                + count_steps_cached(right, steps - 1, cache);

            cache.insert((stone, steps), count);

            count
        }
        1 => {
            let next_stone = stone * 2024;
            let count = count_steps_cached(next_stone, steps - 1, cache);

            cache.insert((stone, steps), count);

            count
        }
        _ => unreachable!(),
    }
}

pub fn part2(input: &str) -> Num {
    let mut cache: HashMap<(Num, u32), Num> = HashMap::new();
    input
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .map(|n_str| {
            let n = n_str.parse().unwrap();

            count_steps_cached(n, 75, &mut cache)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = indoc::indoc! {"
       125 17
"};

    #[test]
    fn example() {
        assert_eq!(part1(EXAMPLE), 55312);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 204022);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, 0);
    }
}
