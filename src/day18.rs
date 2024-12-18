use std::{cmp::Ordering, collections::BinaryHeap};

use crate::utils::CoordinateAdd;

type Num = u32;

const INPUT_SIZE: usize = 71 + 2;

fn parse_input<const SIZE: usize>(input: &str, limit: usize) -> [[u8; SIZE]; SIZE] {
    let mut grid = [[b'.'; SIZE]; SIZE];

    for i in 0..SIZE {
        grid[i][0] = b'#';
        grid[i][SIZE - 1] = b'#';
        grid[0][i] = b'#';
        grid[SIZE - 1][i] = b'#';
    }

    input.lines().take(limit).for_each(|l| {
        let (i, j) = l
            .split_once(",")
            .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
            .unwrap();
        grid[i + 1][j + 1] = b'#';
    });

    grid
}

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

fn part1_inner<const SIZE: usize>(input: &str, limit: usize) -> Num {
    let grid = parse_input::<SIZE>(input, limit);

    // Let's dijkstra 😎
    let mut queue = BinaryHeap::with_capacity(SIZE * SIZE);
    let mut visited = [[false; SIZE]; SIZE];

    queue.push(State {
        i: 1,
        j: 1,
        cost: 0,
    });

    while let Some(u) = queue.pop() {
        if (u.i, u.j) == (SIZE - 2, SIZE - 2) {
            return u.cost;
        }

        if visited[u.i][u.j] {
            continue;
        }
        visited[u.i][u.j] = true;

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

    unreachable!()
}

pub fn part1(input: &str) -> Num {
    part1_inner::<INPUT_SIZE>(input, 1024)
}

fn is_end_reachable<const SIZE: usize>(input: &str, limit: usize) -> bool {
    let grid = parse_input::<SIZE>(input, limit);
    let mut queue = Vec::new();
    let mut visited = [[false; SIZE]; SIZE];

    queue.push((1, 1));

    // DFS to find the end
    while let Some((i, j)) = queue.pop() {
        if (i, j) == (SIZE - 2, SIZE - 2) {
            return true;
        }

        if grid[i][j] == b'#' {
            continue;
        }

        if visited[i][j] {
            continue;
        }
        visited[i][j] = true;

        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .for_each(|direction| {
                let (i, j) = direction.coord_add((i, j));
                queue.push((i, j));
            });
    }

    false
}

fn part2_inner<const SIZE: usize>(input: &str) -> &str {
    let lines = input.lines().collect::<Vec<_>>();

    lines
        .iter()
        .enumerate()
        .find(|(i, _l)| !is_end_reachable::<SIZE>(input, *i + 1))
        .unwrap()
        .1
}

pub fn part2(input: &str) -> &str {
    part2_inner::<INPUT_SIZE>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = indoc::indoc! {"
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"};

    const EXAMPLE_SIZE: usize = 7 + 2;

    #[test]
    fn example() {
        assert_eq!(part1_inner::<EXAMPLE_SIZE>(EXAMPLE, 12), 22);
        assert_eq!(part2_inner::<EXAMPLE_SIZE>(EXAMPLE), "6,1");
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 294);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, "31,22");
    }
}
