use std::collections::HashMap;

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
    fn example() {
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

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 936063);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, 23150395);
    }
}
