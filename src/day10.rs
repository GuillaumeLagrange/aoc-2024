use std::collections::HashSet;

type Num = usize;

const INPUT_SIZE: usize = 55;

#[derive(Clone, Debug)]
struct Cell {
    height: u8,
    reachable_summits: HashSet<(usize, usize)>,
}

pub fn part1(input: &str) -> Num {
    let mut grid: [[Cell; INPUT_SIZE + 2]; INPUT_SIZE + 2] = core::array::from_fn(|_| {
        core::array::from_fn(|_| Cell {
            height: u8::MAX - 1,
            reachable_summits: HashSet::new(),
        })
    });

    input.lines().enumerate().for_each(|(x, line)| {
        line.chars().enumerate().for_each(|(y, c)| {
            let cell = &mut grid[x + 1][y + 1];
            cell.height = c.to_digit(10).unwrap_or((u8::MAX - 1).into()) as u8;
            if cell.height == 9 {
                cell.reachable_summits.insert((x + 1, y + 1));
            }
        });
    });

    for x in 1..INPUT_SIZE + 1 {
        for y in 1..INPUT_SIZE + 1 {
            if grid[x][y].height != 9 {
                continue;
            }

            // Explore all paths starting from 9
            let mut queue = Vec::with_capacity(20);

            queue.push((x, y));

            while let Some((path_cell_x, path_cell_y)) = queue.pop() {
                let current = &grid[path_cell_x][path_cell_y];

                if current.height == 0 {
                    continue;
                }

                let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

                let current_height = current.height;

                directions.iter().for_each(|(dx, dy)| {
                    let (n_x, n_y) = (
                        path_cell_x.checked_add_signed(*dx).unwrap(),
                        path_cell_y.checked_add_signed(*dy).unwrap(),
                    );

                    if grid[n_x][n_y].height == current_height - 1 {
                        grid[n_x][n_y].reachable_summits.insert((x, y));
                        queue.push((n_x, n_y));
                    }
                });
            }
        }
    }

    grid.iter()
        .flat_map(|row| {
            row.iter().map(|cell| {
                if cell.height == 0 {
                    cell.reachable_summits.len()
                } else {
                    0
                }
            })
        })
        .sum()
}

#[derive(Copy, Clone, Debug)]
struct Cell2 {
    height: u8,
    reachable_summits_count: u32,
}

pub fn part2(input: &str) -> Num {
    let mut grid = [[Cell2 {
        height: u8::MAX - 1,
        reachable_summits_count: 0,
    }; INPUT_SIZE + 2]; INPUT_SIZE + 2];

    input.lines().enumerate().for_each(|(x, line)| {
        line.chars().enumerate().for_each(|(y, c)| {
            let cell = &mut grid[x + 1][y + 1];
            cell.height = c.to_digit(10).unwrap_or((u8::MAX - 1).into()) as u8;
            if cell.height == 9 {
                cell.reachable_summits_count = 1;
            }
        });
    });

    for x in 1..INPUT_SIZE + 1 {
        for y in 1..INPUT_SIZE + 1 {
            if grid[x][y].height != 9 {
                continue;
            }

            // Explore all paths starting from 9
            let mut queue = Vec::with_capacity(20);

            queue.push((x, y));

            while let Some((x, y)) = queue.pop() {
                let current = grid[x][y];

                if current.height == 0 {
                    continue;
                }

                let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

                directions.iter().for_each(|(dx, dy)| {
                    let (x, y) = (
                        x.checked_add_signed(*dx).unwrap(),
                        y.checked_add_signed(*dy).unwrap(),
                    );

                    if grid[x][y].height == current.height - 1 {
                        grid[x][y].reachable_summits_count += 1;
                        queue.push((x, y));
                    }
                });
            }
        }
    }

    grid.iter()
        .flat_map(|row| {
            row.iter().map(|cell| {
                if cell.height == 0 {
                    cell.reachable_summits_count as usize
                } else {
                    0
                }
            })
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = indoc::indoc! {"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
"};

    #[test]
    fn example() {
        assert_eq!(part1(EXAMPLE), 36);
        assert_eq!(part2(EXAMPLE), 81);
    }

    #[test]
    fn sample() {
        let sample: &str = indoc::indoc! {"
            ...0...
            ...1...
            ...2...
            6543456
            7.....7
            8.....8
            9.....9
"};

        assert_eq!(part1(sample), 2);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 624);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, 1483);
    }
}
