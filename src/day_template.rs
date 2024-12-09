type Num = u32;

pub fn part1(input: &str) -> Num {
    0
}

pub fn part2(input: &str) -> Num {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = indoc::indoc! {"
        REPLACE_ME
"};

    #[test]
    fn example() {
        assert_eq!(part1(EXAMPLE), 0);
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
