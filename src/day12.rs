type Num = u32;

const GRID_SIZE: usize = 140 + 2;

pub fn part1(input: &str) -> Num {
    #[derive(Copy, Clone)]
    struct Plot {
        vegetable: u8,
        in_a_region: bool,
    }

    struct Region {
        perimeter: Num,
        area: Num,
    }
    let mut grid = [[Plot {
        vegetable: b'.',
        in_a_region: false,
    }; GRID_SIZE]; GRID_SIZE];

    input.lines().enumerate().for_each(|(y, l)| {
        l.bytes().enumerate().for_each(|(x, vegetable)| {
            grid[x + 1][y + 1].vegetable = vegetable;
        })
    });

    let mut sum = 0;
    for x in 1..GRID_SIZE {
        for y in 1..GRID_SIZE {
            let first_plot = grid[y][x];

            if first_plot.in_a_region || first_plot.vegetable == b'.' {
                continue;
            }

            // We are exploring a new region
            let mut region = Region {
                perimeter: 0,
                area: 0,
            };

            let mut queue = Vec::new();
            queue.push((x, y));

            while let Some((x, y)) = queue.pop() {
                let plot = &mut grid[y][x];

                if plot.vegetable != first_plot.vegetable {
                    // This is not the same region, just count a border
                    region.perimeter += 1;
                    continue;
                }

                if plot.in_a_region {
                    // We already counted this plot in the region
                    continue;
                }

                // This plot is part of the region
                plot.in_a_region = true;
                region.area += 1;

                [(0, 1), (1, 0), (0, -1), (-1, 0)]
                    .iter()
                    .for_each(|(dx, dy)| {
                        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                        queue.push((nx as usize, ny as usize));
                    });
            }

            sum += region.area * region.perimeter;
        }
    }
    sum
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
        assert_eq!(output, 1464678);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, 0);
    }
}
