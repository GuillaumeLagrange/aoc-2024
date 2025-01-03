use std::collections::{HashMap, HashSet};

type Num = u32;

pub fn part1(input: &str) -> Num {
    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();
    input.lines().fold(0, |count, line| {
        let (a, b) = line.split_once('-').unwrap();
        connections.entry(a).or_default().insert(b);
        connections.entry(b).or_default().insert(a);

        let a_entries = connections.get(a).unwrap();
        let b_entries = connections.get(b).unwrap();

        let a_or_b_starts_with_t = a.starts_with("t") || b.starts_with("t");

        let new_networks = a_entries.iter().filter(|&entry| {
            (a_or_b_starts_with_t || entry.starts_with("t")) && b_entries.contains(entry)
        });

        count + new_networks.count() as u32
    })
}

pub fn part2(input: &str) -> Num {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = indoc::indoc! {"
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"};

    #[test]
    fn example() {
        assert_eq!(part1(EXAMPLE), 7);
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
