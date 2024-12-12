use std::collections::HashSet;

type Num = u32;

const GRID_SIZE: usize = 140 + 2;

pub fn part1(input: &str) -> Num {
    struct Region {
        perimeter: Num,
        area: Num,
    }
    let mut regions = Vec::<Region>::new();

    let grid = [[b'.'; GRID_SIZE]; GRID_SIZE];

    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.bytes().enumerate().map(move |(x, b)| {
                let top = grid[y - 1][x];
                let left = grid[y][x - 1];

                if b == top || b == left {
                    // Region has already been accounted for
                    return 0;
                }

                // We are exploring a new region
                let mut region = HashSet::new();
                region.insert((x, y));
                let mut queue = Vec::new();
                let right = grid[y][x + 1];
                let bottom = grid[y + 1][x];
                queue.push(right);
                queue.push(bottom);

                0
            })
        })
        .sum()
}

pub fn part2(input: &str) -> Num {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = indoc::indoc! {"
AAAA
BBCD
BBCC
EEEC
"};

    const EXAMPLE_2: &str = indoc::indoc! {"
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
"};

    const EXAMPLE_3: &str = indoc::indoc! {"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"};

    #[test]
    fn example() {
        assert_eq!(part1(EXAMPLE_1), 140);
        assert_eq!(part1(EXAMPLE_2), 772);
        assert_eq!(part1(EXAMPLE_3), 1930);
        assert_eq!(part2(EXAMPLE_1), 0);
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
