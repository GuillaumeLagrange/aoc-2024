const INPUT_SIZE: usize = 130;

type Input = ([[u8; INPUT_SIZE + 2]; INPUT_SIZE + 2], (usize, usize));

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn next_direction(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn next_coordinates(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Self::North => (x - 1, y),
            Self::East => (x, y + 1),
            Self::South => (x + 1, y),
            Self::West => (x, y - 1),
        }
    }

    fn to_bitmask(self) -> u8 {
        match self {
            Self::North => 1 << 0,
            Self::East => 1 << 1,
            Self::South => 1 << 2,
            Self::West => 1 << 3,
        }
    }
}

fn parse(input: &str) -> Input {
    let mut grid = [[b'%'; INPUT_SIZE + 2]; INPUT_SIZE + 2];
    let mut start_position = None;

    input.lines().enumerate().for_each(|(y, line)| {
        line.bytes().enumerate().for_each(|(x, c)| match c {
            b'^' => {
                start_position = Some((y + 1, x + 1));
                grid[y + 1][x + 1] = Direction::North.to_bitmask();
            }
            b'#' => grid[y + 1][x + 1] = c,
            _ => grid[y + 1][x + 1] = 0,
        });
    });

    (grid, start_position.unwrap())
}

pub fn part1(input: &str) -> u32 {
    let (mut grid, (mut x, mut y)) = parse(input);
    let mut visited = 1;
    let mut direction = Direction::North;

    loop {
        let (next_x, next_y) = direction.next_coordinates((x, y));

        match grid[next_x][next_y] {
            b'%' => break,
            b'#' => {
                // We hit a wall, just change direction
                direction = direction.next_direction();
            }
            0 => {
                // We can move forward, increase visited count
                x = next_x;
                y = next_y;
                grid[x][y] = b'X';
                visited += 1;
            }
            _ => {
                // We can move forward
                x = next_x;
                y = next_y;
            }
        }
    }

    visited
}

pub fn part2(input: &str) -> usize {
    let (mut grid, (mut x, mut y)) = parse(input);
    let mut cycles = 0;
    let mut direction = Direction::North;

    loop {
        let (next_x, next_y) = direction.next_coordinates((x, y));

        match grid[next_x][next_y] {
            b'%' => break,
            b'#' => {
                // We hit a wall, just change direction
                direction = direction.next_direction();
                grid[x][y] |= direction.to_bitmask();
            }
            0 => {
                let mut cycle_direction = direction.next_direction();
                let mut cycle_grid = grid;
                cycle_grid[next_x][next_y] = b'#';
                let (mut cycle_x, mut cycle_y) = (x, y);

                loop {
                    let (next_cycle_x, next_cycle_y) =
                        cycle_direction.next_coordinates((cycle_x, cycle_y));

                    match cycle_grid[next_cycle_x][next_cycle_y] {
                        b'%' => break,
                        b'#' => {
                            cycle_direction = cycle_direction.next_direction();
                        }
                        cycle_cell => {
                            if cycle_cell & cycle_direction.to_bitmask() != 0 {
                                cycles += 1;
                                break;
                            } else {
                                cycle_x = next_cycle_x;
                                cycle_y = next_cycle_y;
                                cycle_grid[cycle_x][cycle_y] |= cycle_direction.to_bitmask();
                            }
                        }
                    }
                }

                x = next_x;
                y = next_y;
                grid[x][y] |= direction.to_bitmask();
            }
            _ => {
                x = next_x;
                y = next_y;
                grid[x][y] |= direction.to_bitmask();
            }
        }
    }

    cycles
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...

";

    #[test]
    fn example() {
        assert_eq!(part1(EXAMPLE), 41);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 5162);
    }

    #[test]
    fn debug_part2() {
        let test_case = "\
..#..
....#
.#...
^..#.
";

        assert_eq!(part2(test_case), 1);

        let test_case = "\
.....
.....
^....
.....
";

        assert_eq!(part2(test_case), 0);

        let test_case = "\
.....
.....
#...#
.^.#.
";

        assert_eq!(part2(test_case), 1);

        assert_eq!(part2(EXAMPLE), 6);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, 1909);
    }
}
