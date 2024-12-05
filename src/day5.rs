use std::collections::{HashMap, HashSet};

type Number = u32;

struct Input {
    /// Each entry is the set of pages that **must** be before the key IF they are present in the update
    rules: HashMap<Number, HashSet<Number>>,
    updates: Vec<Vec<Number>>,
}

fn parse_input(input: &str) -> Input {
    let (rules, lists) = input.split_once("\n\n").unwrap();

    let rules: HashMap<Number, HashSet<Number>> =
        rules.lines().fold(HashMap::new(), |mut rules, line| {
            let (first, second): (Number, Number) = line
                .split_once("|")
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap();

            rules.entry(second).or_default().insert(first);

            rules
        });

    let updates: Vec<Vec<Number>> = lists
        .lines()
        .map(|line| {
            line.split(",")
                .map(|n| n.parse().unwrap())
                .collect::<Vec<Number>>()
        })
        .collect();

    Input { rules, updates }
}

pub fn part1_first_implem(input: &str) -> Number {
    let Input { rules, updates } = parse_input(input);

    updates.iter().fold(0, |sum, update| {
        for (i, page) in update.iter().enumerate() {
            // Check that rules are satisfied for this page, aka the current page should not be
            // placed before any already placed pages
            for previous_page in &update[..i] {
                if let Some(rule) = rules.get(previous_page) {
                    if rule.contains(page) {
                        return sum;
                    }
                }
            }
        }

        sum + update[update.len() / 2]
    })
}

pub fn part1(input: &str) -> Number {
    let Input { rules, updates } = parse_input(input);

    // Make a custom sort that respects the rules
    let sorter = |a: &Number, b: &Number| {
        let Some(rule) = rules.get(a) else {
            return std::cmp::Ordering::Less;
        };

        if rule.contains(b) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    };

    updates
        .into_iter()
        .map(|mut update| {
            let snapshot = update.clone();

            update.sort_unstable_by(sorter);

            if update != snapshot {
                0
            } else {
                update[update.len() / 2]
            }
        })
        .sum()
}

pub fn part2(input: &str) -> Number {
    let Input { rules, updates } = parse_input(input);

    // Make a custom sort that respects the rules
    let sorter = |a: &Number, b: &Number| {
        let Some(rule) = rules.get(a) else {
            return std::cmp::Ordering::Less;
        };

        if rule.contains(b) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    };

    updates
        .into_iter()
        .map(|mut update| {
            let snapshot = update.clone();

            update.sort_unstable_by(sorter);

            if update == snapshot {
                0
            } else {
                update[update.len() / 2]
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

        assert_eq!(part1(example), 143);
        assert_eq!(part2(example), 123);
    }

    #[test]
    fn part2_debug() {
        let example = "\
1|2
2|3

3,2,1
1,2,3
";

        assert_eq!(part2(example), 2);

        let example = "\
1|2
2|3
3|4
3|8
4|8
8|3
4|7

1,2,3,4,7
4,3,7,1,2
";

        assert_eq!(part1(example), 3);
        assert_eq!(part2(example), 3);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 4135);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, 5285);
    }
}
