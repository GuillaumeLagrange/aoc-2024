use std::{collections::HashMap, iter};

type Num = u64;

const PRUNE_MOD: u64 = 16777216;

fn next_secret(secret: Num) -> Num {
    let secret = (secret ^ (secret << 6)) % PRUNE_MOD;

    let secret = (secret ^ (secret >> 5)) % PRUNE_MOD;

    (secret ^ (secret << 11)) % PRUNE_MOD
}

pub fn part1(input: &str) -> Num {
    input
        .lines()
        .map(|line| {
            let mut secret = line.parse().unwrap();
            (0..2000).for_each(|_| {
                secret = next_secret(secret);
            });

            secret
        })
        .sum()
}

pub fn part2(input: &str) -> Num {
    let benefit_per_sequence = input.lines().fold(HashMap::new(), |mut global_acc, line| {
        let mut secret = line.parse::<Num>().unwrap();

        let secrets = iter::once(secret).chain((0..2000).map(|_| {
            secret = next_secret(secret);
            secret
        }));

        let prices = secrets.map(|s| s % 10).collect::<Vec<_>>();

        let price_variations = prices
            .windows(2)
            .map(|w| (w[1] as i64 - w[0] as i64, w[1]))
            .collect::<Vec<_>>();

        let prices_per_sequence =
            price_variations
                .windows(4)
                .rev()
                .fold(HashMap::new(), |mut acc, w| {
                    acc.insert(w.iter().map(|entry| entry.0).collect::<Vec<_>>(), w[3].1);

                    acc
                });

        prices_per_sequence.into_iter().for_each(|(k, v)| {
            global_acc.entry(k).and_modify(|e| *e += v).or_insert(v);
        });

        global_acc
    });

    *benefit_per_sequence.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_secret() {
        assert_eq!(next_secret(123), 15887950);
        assert_eq!(next_secret(15887950), 16495136);
    }

    const EXAMPLE: &str = indoc::indoc! {"
1
10
100
2024
"};

    const EXAMPLE_2: &str = indoc::indoc! {"
1
2
3
2024
"};

    #[test]
    fn example() {
        assert_eq!(part1(EXAMPLE), 37327623);
        assert_eq!(part2(EXAMPLE_2), 23);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 12979353889);
    }

    #[test]
    fn run_part2() {
        // let input = crate::utils::get_day_input!();
        // let output = part2(&input);
        // println!("Part 2: {}", output);
        // assert_eq!(output, 1449);
    }
}
