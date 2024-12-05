fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

struct ExplorationStatus {
    starting_point: (isize, isize),
    direction: (isize, isize),
    seeked_letter: char,
}

fn explore(
    input: &Vec<Vec<char>>,
    ExplorationStatus {
        starting_point,
        direction,
        seeked_letter,
    }: ExplorationStatus,
) -> usize {
    let to_explore = (
        starting_point.0 + direction.0,
        starting_point.1 + direction.1,
    );
    if to_explore.0 < 0
        || to_explore.0 >= input.len() as isize
        || to_explore.1 < 0
        || to_explore.1 >= input[0].len() as isize
    {
        return 0;
    }

    if seeked_letter != input[to_explore.0 as usize][to_explore.1 as usize] {
        return 0;
    }

    if seeked_letter == 'S' {
        return 1;
    }

    explore(
        input,
        ExplorationStatus {
            starting_point: to_explore,
            direction,
            seeked_letter: match seeked_letter {
                'M' => 'A',
                'A' => 'S',
                _ => unreachable!(),
            },
        },
    )
}

pub fn part1(input: &str) -> usize {
    let input = parse_input(input);
    let mut count = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == 'X' {
                for direction in [
                    (0, 1),
                    (1, 0),
                    (0, -1),
                    (-1, 0),
                    (1, 1),
                    (1, -1),
                    (-1, 1),
                    (-1, -1),
                ]
                .iter()
                {
                    count += explore(
                        &input,
                        ExplorationStatus {
                            starting_point: (i as isize, j as isize),
                            direction: *direction,
                            seeked_letter: 'M',
                        },
                    );
                }
            }
        }
    }

    count
}

pub fn part2(input: &str) -> usize {
    let input = parse_input(input);
    let mut count = 0;

    (1..(input.len() - 1)).for_each(|i| {
        (1..input[i].len() - 1).for_each(|j| {
            if input[i][j] == 'A' {
                let top_left = input[i - 1][j - 1];
                let top_right = input[i - 1][j + 1];
                let bottom_left = input[i + 1][j - 1];
                let bottom_right = input[i + 1][j + 1];

                match (top_left, top_right, bottom_right, bottom_left) {
                    ('M', 'M', 'S', 'S')
                    | ('S', 'M', 'M', 'S')
                    | ('S', 'S', 'M', 'M')
                    | ('M', 'S', 'S', 'M') => count += 1,
                    _ => {}
                }
            }
        });
    });

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

        assert_eq!(part1(example), 18);
        assert_eq!(part2(example), 9);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 2378);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, 1796);
    }
}
