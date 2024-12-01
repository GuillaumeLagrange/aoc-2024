#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    for line in input.lines() {
        let (first, second) = line.split_once("   ").unwrap();
        first_list.push(first.parse::<u32>().unwrap());
        second_list.push(second.parse::<u32>().unwrap());
    }

    first_list.sort();
    second_list.sort();

    first_list
        .into_iter()
        .enumerate()
        .map(|(i, first)| {
            let second = second_list[i];

            second.abs_diff(first)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn sample1() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
    }
}
