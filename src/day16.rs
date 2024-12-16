use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use crate::utils::CoordinateAdd;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(&self) -> [((i32, i32), Direction); 3] {
        match self {
            Direction::Up => [
                ((-1, 0), Direction::Up),
                ((0, -1), Direction::Left),
                ((0, 1), Direction::Right),
            ],
            Direction::Down => [
                ((1, 0), Direction::Down),
                ((0, 1), Direction::Right),
                ((0, -1), Direction::Left),
            ],
            Direction::Left => [
                ((0, -1), Direction::Left),
                ((-1, 0), Direction::Up),
                ((1, 0), Direction::Down),
            ],
            Direction::Right => [
                ((0, 1), Direction::Right),
                ((1, 0), Direction::Down),
                ((-1, 0), Direction::Up),
            ],
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    x: usize,
    y: usize,
    direction: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Comparison is flipped to make the heap pop the smallest cost first
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const SIZE: usize = 145;

fn parse(input: &str) -> ([[u8; SIZE]; SIZE], (usize, usize)) {
    let mut grid = [[b' '; SIZE]; SIZE];
    let mut start = (0, 0);

    input.lines().enumerate().for_each(|(i, l)| {
        l.bytes().enumerate().for_each(|(j, c)| {
            grid[i][j] = c;
            if c == b'S' {
                start = (i, j);
            }
        });
    });

    (grid, start)
}

pub fn part1(input: &str) -> usize {
    let (grid, start) = parse(input);
    let mut visited = [[false; SIZE]; SIZE];
    let mut p_queue = BinaryHeap::with_capacity(SIZE * SIZE);

    p_queue.push(State {
        cost: 0,
        x: start.0,
        y: start.1,
        direction: Direction::Right,
    });

    while let Some(cur) = p_queue.pop() {
        if grid[cur.x][cur.y] == b'E' {
            return cur.cost;
        }
        if visited[cur.x][cur.y] {
            continue;
        }
        visited[cur.x][cur.y] = true;

        for (next, direction) in cur.direction.next() {
            let next = next.coord_add((cur.x, cur.y));
            if grid[next.0][next.1] == b'#' {
                continue;
            }
            p_queue.push(State {
                cost: cur.cost + if direction == cur.direction { 1 } else { 1001 },
                x: next.0,
                y: next.1,
                direction,
            });
        }
    }

    unreachable!()
}

#[derive(Clone, Eq, PartialEq)]
struct StateWithPath {
    cost: usize,
    coords: (usize, usize),
    direction: Direction,
    path: Vec<(usize, usize)>,
}

impl Ord for StateWithPath {
    fn cmp(&self, other: &Self) -> Ordering {
        // Comparison is flipped to make the heap pop the smallest cost first
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for StateWithPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part2(input: &str) -> usize {
    let (grid, start) = parse(input);
    let mut p_queue = BinaryHeap::with_capacity(SIZE * SIZE);

    p_queue.push(StateWithPath {
        cost: 0,
        coords: start,
        direction: Direction::Right,
        path: vec![start],
    });

    let mut best_path_grid = grid;
    let mut min_cost = usize::MAX;
    let mut visited: HashMap<((usize, usize), Direction), usize> = HashMap::new();

    while let Some(u) = p_queue.pop() {
        if grid[u.coords.0][u.coords.1] == b'E' {
            min_cost = min_cost.min(u.cost);
            if u.cost > min_cost {
                // We have found all the paths with the minimum cost, stop here
                break;
            }
            u.path.iter().for_each(|&(x, y)| {
                best_path_grid[x][y] = b'O';
            });
        }
        if let Some(&cost) = visited.get(&(u.coords, u.direction)) {
            if u.cost > cost {
                continue;
            }
        } else {
            visited.insert((u.coords, u.direction), u.cost);
        }

        for (v, direction) in u.direction.next() {
            let next = v.coord_add(u.coords);
            if grid[next.0][next.1] == b'#' {
                continue;
            }
            let n_cost = u.cost + if direction == u.direction { 1 } else { 1001 };
            if let Some(&cost) = visited.get(&(next, direction)) {
                if n_cost >= cost {
                    continue;
                }
            }
            p_queue.push(StateWithPath {
                cost: n_cost,
                coords: next,
                direction,
                path: u
                    .path
                    .clone()
                    .into_iter()
                    .chain(std::iter::once(next))
                    .collect(),
            });
        }
    }

    best_path_grid
        .iter()
        .flatten()
        .filter(|&&b| b == b'O')
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = indoc::indoc! {"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"};

    const EXAMPLE2: &str = indoc::indoc! {"
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
"};

    const EXAMPLE3: &str = indoc::indoc! {"
###########################
#######################..E#
######################..#.#
#####################..##.#
####################..###.#
###################..##...#
##################..###.###
#################..####...#
################..#######.#
###############..##.......#
##############..###.#######
#############..####.......#
############..###########.#
###########..##...........#
##########..###.###########
#########..####...........#
########..###############.#
#######..##...............#
######..###.###############
#####..####...............#
####..###################.#
###..##...................#
##..###.###################
#..####...................#
#.#######################.#
#S........................#
###########################
"};

    #[test]
    fn example() {
        assert_eq!(part1(EXAMPLE), 7036);
        assert_eq!(part1(EXAMPLE2), 11048);
        assert_eq!(part1(EXAMPLE3), 21148);
        assert_eq!(part2(EXAMPLE), 45);
        assert_eq!(part2(EXAMPLE2), 64);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 160624);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, 692);
    }
}
