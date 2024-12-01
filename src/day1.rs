use std::collections::HashMap;

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

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let mut first_list = Vec::new();
    let mut second_list: HashMap<u32, u32> = HashMap::new();

    for line in input.lines() {
        let (first, second) = line
            .split_once("   ")
            .map(|(first, second)| {
                (
                    first.parse::<u32>().unwrap(),
                    second.parse::<u32>().unwrap(),
                )
            })
            .unwrap();

        first_list.push(first);

        let second_entry = second_list.entry(second).or_default();
        *second_entry += 1;
    }

    first_list
        .into_iter()
        .map(|number| number * second_list.get(&number).copied().unwrap_or_default())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let example = "\
3   4
4   3
2   5
1   3
3   9
3   3
";
        assert_eq!(part1(example), 11);
        assert_eq!(part2(example), 31);
    }
}
