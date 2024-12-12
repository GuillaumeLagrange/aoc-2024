use crate::utils::CoordinateDiff;

type Num = u32;

const GRID_SIZE: usize = 140 + 2;

#[derive(Copy, Clone)]
struct Plot {
    plant: u8,
    in_a_region: bool,
}

pub fn part1(input: &str) -> Num {
    struct Region {
        perimeter: Num,
        area: Num,
    }

    let mut grid = [[Plot {
        plant: b'.',
        in_a_region: false,
    }; GRID_SIZE]; GRID_SIZE];

    input.lines().enumerate().for_each(|(y, l)| {
        l.bytes().enumerate().for_each(|(x, plant)| {
            grid[x + 1][y + 1].plant = plant;
        })
    });

    let mut sum = 0;
    for x in 1..GRID_SIZE {
        for y in 1..GRID_SIZE {
            let first_plot = grid[y][x];

            if first_plot.in_a_region || first_plot.plant == b'.' {
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

                if plot.plant != first_plot.plant {
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
                    .coord_diff(x, y)
                    .for_each(|(dx, dy)| {
                        queue.push((dx, dy));
                    });
            }

            sum += region.area * region.perimeter;
        }
    }
    sum
}

fn is_corner(plant: u8, side1: &Plot, side2: &Plot, diagonal: &Plot) -> bool {
    let side_1_is_same_plant = side1.plant == plant;
    let side_2_is_same_plant = side2.plant == plant;
    let corner_is_same_plant = diagonal.plant == plant;

    match (
        side_1_is_same_plant,
        side_2_is_same_plant,
        corner_is_same_plant,
    ) {
        //   🟥
        // 🟥🟩
        (false, false, _) => true,
        // 🟥🟩
        // 🟩🟩
        (true, true, false) => true,
        _ => false,
    }
}

pub fn part2(input: &str) -> Num {
    #[derive(Debug)]
    struct Region {
        sides: Num,
        area: Num,
    }

    let mut grid = [[Plot {
        plant: b'.',
        in_a_region: false,
    }; GRID_SIZE]; GRID_SIZE];

    input.lines().enumerate().for_each(|(y, l)| {
        l.bytes().enumerate().for_each(|(x, plant)| {
            grid[y + 1][x + 1].plant = plant;
        })
    });

    let mut sum = 0;
    for x in 1..GRID_SIZE {
        for y in 1..GRID_SIZE {
            let first_plot = grid[y][x];

            if first_plot.in_a_region || first_plot.plant == b'.' {
                continue;
            }

            // We are exploring a new region
            let mut region = Region { sides: 0, area: 0 };

            let mut queue = Vec::new();
            queue.push((x, y));

            while let Some((x, y)) = queue.pop() {
                let plot = &mut grid[y][x];

                if plot.plant != first_plot.plant || plot.in_a_region {
                    // This is not the same region or is already counted
                    continue;
                }

                // This plot is part of the region
                region.area += 1;
                plot.in_a_region = true;

                // Check for corners
                [
                    // Top left
                    (-1, -1),
                    // Top right
                    (-1, 1),
                    // Bottom right
                    (1, 1),
                    // Bottom left
                    (1, -1),
                ]
                .coord_diff(x, y)
                .for_each(|(dx, dy)| {
                    if is_corner(first_plot.plant, &grid[dy][x], &grid[y][dx], &grid[dy][dx]) {
                        region.sides += 1;
                    }
                });

                [
                    // Left
                    (-1, 0),
                    // Bottom
                    (0, -1),
                    // Right
                    (1, 0),
                    // Top
                    (0, 1),
                ]
                .coord_diff(x, y)
                .for_each(|(dx, dy)| {
                    queue.push((dx, dy));
                });
            }

            sum += region.area * region.sides;
        }
    }

    sum
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

    const EXAMPLE_4: &str = indoc::indoc! {"
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
"};

    #[test]
    fn example() {
        assert_eq!(part1(EXAMPLE_1), 140);
        assert_eq!(part1(EXAMPLE_2), 772);
        assert_eq!(part1(EXAMPLE_3), 1930);

        assert_eq!(part2(EXAMPLE_1), 80);
        assert_eq!(part2(EXAMPLE_2), 436);
        assert_eq!(part2(EXAMPLE_3), 1206);
        assert_eq!(part2(EXAMPLE_4), 368);
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
        assert_eq!(output, 877492);
    }
}
