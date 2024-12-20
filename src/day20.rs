use std::{cmp::Ordering, collections::BinaryHeap};

use crate::utils::{CoordinateAdd, ManhattanDistance, CARDINAL_DIRECTIONS};

type Num = u32;

#[derive(Debug, PartialEq, Eq)]
struct State {
    i: usize,
    j: usize,
    cost: Num,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Comparison is flipped to make the heap pop the smallest cost first
        other.cost.cmp(&self.cost)
    }
}

struct Input<const SIZE: usize> {
    grid: [[u8; SIZE]; SIZE],
    start: (usize, usize),
    end: (usize, usize),
}

fn parse_input<const SIZE: usize>(input: &str) -> Input<SIZE> {
    let mut grid = [[b'.'; SIZE]; SIZE];
    let mut start = None;
    let mut end = None;

    for i in 0..SIZE {
        grid[i][0] = b'%';
        grid[i][SIZE - 1] = b'%';
        grid[0][i] = b'%';
        grid[SIZE - 1][i] = b'%';
    }

    input.lines().enumerate().for_each(|(i, l)| {
        l.bytes().enumerate().for_each(|(j, b)| {
            if b == b'S' {
                start = Some((i + 1, j + 1));
            } else if b == b'E' {
                end = Some((i + 1, j + 1));
            }
            grid[i + 1][j + 1] = b;
        });
    });

    Input {
        grid,
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

fn part1_inner<const SIZE: usize>(input: &str, threshold: Num) -> Num {
    let Input { grid, start, end } = parse_input::<SIZE>(input);
    let mut queue = BinaryHeap::with_capacity(SIZE * SIZE);

    let mut paths_to_end = [[Num::MAX; SIZE]; SIZE];

    queue.push(State {
        i: end.0,
        j: end.1,
        cost: 0,
    });

    while let Some(u) = queue.pop() {
        if paths_to_end[u.i][u.j] <= u.cost {
            // There is already a better path for this node
            continue;
        }
        paths_to_end[u.i][u.j] = u.cost;

        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .for_each(|direction| {
                let (i, j) = direction.coord_add((u.i, u.j));

                if grid[i][j] == b'#' {
                    return;
                }

                queue.push(State {
                    i,
                    j,
                    cost: u.cost + 1,
                });
            });
    }

    let mut paths_from_start = [[Num::MAX; SIZE]; SIZE];

    queue.push(State {
        i: start.0,
        j: start.1,
        cost: 0,
    });

    while let Some(u) = queue.pop() {
        if paths_from_start[u.i][u.j] <= u.cost {
            // There is already a better path for this node
            continue;
        }
        paths_from_start[u.i][u.j] = u.cost;

        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .for_each(|direction| {
                let (i, j) = direction.coord_add((u.i, u.j));

                if grid[i][j] == b'#' {
                    return;
                }

                queue.push(State {
                    i,
                    j,
                    cost: u.cost + 1,
                });
            });
    }

    let best_path_without_cheats = paths_from_start[end.0][end.1];

    // Look for all the cheats and how much time they save
    let mut ret = 0;
    for i in 2..SIZE - 1 {
        for j in 2..SIZE - 1 {
            // Look for cheats
            match grid[i][j] {
                b'#' | b'%' | b'E' => {}
                _ => {
                    ret += CARDINAL_DIRECTIONS
                        .iter()
                        .map(|direction| {
                            let (i1, j1) = direction.coord_add((i, j));
                            let (i2, j2) = direction.coord_add((i1, j1));

                            if grid[i2][j2] == b'#' || grid[i2][j2] == b'%' {
                                // Cannot cheat from here
                                return 0;
                            }

                            let cost_with_cheat = paths_from_start[i][j] + 2 + paths_to_end[i2][j2];

                            let saved = best_path_without_cheats.saturating_sub(cost_with_cheat);

                            if saved >= threshold {
                                1
                            } else {
                                0
                            }
                        })
                        .sum::<Num>();
                }
            };
        }
    }

    ret
}

const INPUT_SIZE: usize = 143;
pub fn part1(input: &str) -> Num {
    part1_inner::<INPUT_SIZE>(input, 100)
}

fn part2_inner<const SIZE: usize>(input: &str, threshold: Num) -> Num {
    let Input { grid, start, end } = parse_input::<SIZE>(input);
    let mut queue = BinaryHeap::with_capacity(SIZE * SIZE);

    let mut paths_to_end = [[Num::MAX; SIZE]; SIZE];

    queue.push(State {
        i: end.0,
        j: end.1,
        cost: 0,
    });

    while let Some(u) = queue.pop() {
        if paths_to_end[u.i][u.j] <= u.cost {
            // There is already a better path for this node
            continue;
        }
        paths_to_end[u.i][u.j] = u.cost;

        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .for_each(|direction| {
                let (i, j) = direction.coord_add((u.i, u.j));

                if grid[i][j] == b'#' {
                    return;
                }

                queue.push(State {
                    i,
                    j,
                    cost: u.cost + 1,
                });
            });
    }

    let mut paths_from_start = [[Num::MAX; SIZE]; SIZE];

    queue.push(State {
        i: start.0,
        j: start.1,
        cost: 0,
    });

    while let Some(u) = queue.pop() {
        if paths_from_start[u.i][u.j] <= u.cost {
            // There is already a better path for this node
            continue;
        }
        paths_from_start[u.i][u.j] = u.cost;

        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .for_each(|direction| {
                let (i, j) = direction.coord_add((u.i, u.j));

                if grid[i][j] == b'#' {
                    return;
                }

                queue.push(State {
                    i,
                    j,
                    cost: u.cost + 1,
                });
            });
    }

    let best_path_without_cheats = paths_from_start[end.0][end.1];

    // Look for all the cheats and how much time they save
    let mut ret = 0;
    for i in 2..SIZE - 1 {
        for j in 2..SIZE - 1 {
            // Look for cheats
            match grid[i][j] {
                b'#' | b'%' | b'E' => {}
                _ => {
                    ret += (i, j)
                        .within_manhattan_distance(20, 0, SIZE)
                        .map(|(end_i, end_j)| {
                            if grid[end_i][end_j] == b'#' || grid[end_i][end_j] == b'%' {
                                // Cannot cheat from here
                                return 0;
                            }

                            let cost_with_cheat = paths_from_start[i][j]
                                + end_i.abs_diff(i) as u32
                                + end_j.abs_diff(j) as u32
                                + paths_to_end[end_i][end_j];

                            let saved = best_path_without_cheats.saturating_sub(cost_with_cheat);

                            if saved >= threshold {
                                1
                            } else {
                                0
                            }
                        })
                        .sum::<Num>();
                }
            };
        }
    }

    ret
}

pub fn part2(input: &str) -> Num {
    part2_inner::<INPUT_SIZE>(input, 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = indoc::indoc! {"
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"};

    const EXAMPLE_SIZE: usize = 15 + 2;

    #[test]
    fn example() {
        assert_eq!(part1_inner::<EXAMPLE_SIZE>(EXAMPLE, 12), 8);
        assert_eq!(
            part2_inner::<EXAMPLE_SIZE>(EXAMPLE, 50),
            [32, 31, 29, 39, 25, 23, 20, 19, 12, 14, 12, 22, 4, 3]
                .iter()
                .sum()
        );
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 1365);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, 986082);
    }
}
