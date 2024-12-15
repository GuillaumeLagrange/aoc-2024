use arraydeque::ArrayDeque;
use arrayvec::ArrayVec;
use colored::Colorize;
use std::{
    fmt,
    ops::{Index, IndexMut},
};

use crate::utils::CoordinateAdd;

type Num = u32;

const GRID_SIZE: usize = 50;

struct Grid<const W: usize, const H: usize> {
    inner: ArrayVec<ArrayVec<u8, W>, H>,
}

impl<const W: usize, const H: usize> FromIterator<ArrayVec<u8, W>> for Grid<W, H> {
    fn from_iter<T: IntoIterator<Item = ArrayVec<u8, W>>>(iter: T) -> Self {
        let inner = iter.into_iter().collect();
        Self { inner }
    }
}

impl<const W: usize, const H: usize> Index<usize> for Grid<W, H> {
    type Output = ArrayVec<u8, W>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl<const W: usize, const H: usize> IndexMut<usize> for Grid<W, H> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl<const W: usize, const H: usize> IntoIterator for Grid<W, H> {
    type Item = ArrayVec<u8, W>;
    type IntoIter = arrayvec::IntoIter<Self::Item, H>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<const W: usize, const H: usize> fmt::Debug for Grid<W, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for (i, l) in self.inner.iter().enumerate() {
            for (j, b) in l.iter().enumerate() {
                let c = if (i, j) == (32, 64) {
                    colored::Color::Yellow
                } else {
                    match b {
                        b'#' => colored::Color::Red,
                        b'@' => colored::Color::Green,
                        b'[' | b']' => colored::Color::Blue,
                        _ => colored::Color::White,
                    }
                };
                write!(f, "{}", (*b as char).to_string().color(c))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn part1(input: &str) -> Num {
    let (grid_str, directions_str) = input.split_once("\n\n").unwrap();

    let mut robot = (0, 0);
    let mut grid: Grid<GRID_SIZE, GRID_SIZE> = grid_str
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.bytes()
                .enumerate()
                .map(|(j, b)| {
                    if b == b'@' {
                        robot = (i, j);
                    }

                    b
                })
                .collect()
        })
        .collect();

    directions_str.bytes().for_each(|b| {
        if b == b'\n' {
            return;
        }

        let direction: (i32, i32) = match b {
            b'^' => (-1, 0),
            b'v' => (1, 0),
            b'<' => (0, -1),
            b'>' => (0, 1),
            _ => unreachable!(),
        };

        let end_of_movement = {
            let mut movement = robot;
            loop {
                let next = direction.coord_add(movement);

                match grid[next.0][next.1] {
                    b'#' => break None,
                    b'.' => {
                        break Some(next);
                    }
                    _ => {
                        // This is a box
                    }
                }

                movement = next;
            }
        };

        if let Some((i, j)) = end_of_movement {
            let next_robot = direction.coord_add(robot);
            grid[robot.0][robot.1] = b'.';
            grid[i][j] = b'O';
            grid[next_robot.0][next_robot.1] = b'@';
            robot = next_robot;
        }
    });

    grid.into_iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.into_iter().enumerate().map(
                move |(j, b)| {
                    if b == b'O' {
                        (i * 100 + j) as Num
                    } else {
                        0
                    }
                },
            )
        })
        .sum()
}

pub fn part2(input: &str) -> Num {
    let (grid_str, directions_str) = input.split_once("\n\n").unwrap();

    let mut robot = (0, 0);
    let mut grid: Grid<{ 2 * GRID_SIZE }, GRID_SIZE> = grid_str
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.bytes()
                .enumerate()
                .flat_map(|(j, b)| {
                    use std::iter;
                    match b {
                        b'O' => iter::once(b'[').chain(iter::once(b']')),
                        b'@' => {
                            robot = (i, 2 * j);

                            iter::once(b'@').chain(iter::once(b'.'))
                        }
                        b => iter::once(b).chain(iter::once(b)),
                    }
                })
                .collect()
        })
        .collect();

    let mut count = 0;
    directions_str.bytes().for_each(|b| {
        count += 1;
        if b == b'\n' {
            return;
        }

        let (direction, horizontal): ((i32, i32), bool) = match b {
            b'^' => ((-1, 0), false),
            b'v' => ((1, 0), false),
            b'<' => ((0, -1), true),
            b'>' => ((0, 1), true),
            _ => unreachable!(),
        };

        const CAP: usize = GRID_SIZE * 3;
        let mut boxes_to_move = ArrayVec::<(usize, usize), CAP>::new();
        let mut can_move = true;
        let mut queue = ArrayDeque::<_, CAP>::new();
        queue.push_back(direction.coord_add(robot)).unwrap();

        while let Some((i, j)) = queue.pop_front() {
            match grid[i][j] {
                b'[' => {
                    boxes_to_move.push((i, j));
                    if horizontal {
                        queue
                            .push_back(direction.coord_add(direction.coord_add((i, j))))
                            .unwrap();
                    } else {
                        queue.push_back(direction.coord_add((i, j))).unwrap();
                        queue.push_back(direction.coord_add((i, j + 1))).unwrap();
                    }
                }
                b']' => {
                    // Left box
                    boxes_to_move.push((i, j - 1));
                    if horizontal {
                        queue
                            .push_back(direction.coord_add(direction.coord_add((i, j))))
                            .unwrap();
                    } else {
                        queue.push_back(direction.coord_add((i, j - 1))).unwrap();
                        queue.push_back(direction.coord_add((i, j))).unwrap();
                    }
                }
                b'#' => {
                    can_move = false;
                    break;
                }
                _ => {
                    // Empty space
                }
            }
        }

        if can_move {
            // Move all the boxes starting from the last one to avoid overwriting changes
            boxes_to_move.into_iter().rev().for_each(|box_left| {
                let new_box_left = direction.coord_add(box_left);
                // Remove old box
                grid[box_left.0][box_left.1] = b'.';
                grid[box_left.0][box_left.1 + 1] = b'.';
                // Draw new box
                grid[new_box_left.0][new_box_left.1] = b'[';
                grid[new_box_left.0][new_box_left.1 + 1] = b']';
            });

            // Move robot
            let next_robot = direction.coord_add(robot);
            grid[robot.0][robot.1] = b'.';
            grid[next_robot.0][next_robot.1] = b'@';
            robot = next_robot;
        }
    });

    grid.into_iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.into_iter().enumerate().map(
                move |(j, b)| {
                    if b == b'[' {
                        (i * 100 + j) as Num
                    } else {
                        0
                    }
                },
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE: &str = indoc::indoc! {"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
"};

    const EXAMPLE: &str = indoc::indoc! {"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"};

    const SMALL_EXAMPLE_PART2: &str = indoc::indoc! {"
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
"};

    #[test]
    fn example() {
        assert_eq!(part1(SMALL_EXAMPLE), 2028);
        assert_eq!(part1(EXAMPLE), 10092);
        assert_eq!(part2(SMALL_EXAMPLE_PART2), 618);
        assert_eq!(part2(EXAMPLE), 9021);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 1479679);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, 1509780);
    }

    #[test]
    fn debug() {
        let debug_input = indoc::indoc! {"
#######
#.....#
#.....#
#.O...#
#.OO@.#
#.OO..#
#..O..#
#.....#
#######

<^<^<v
"};

        assert_eq!(part2(debug_input), 3328);
    }
}
