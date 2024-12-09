use std::collections::{HashMap, HashSet};

use itertools::Itertools;

type Num = usize;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
struct P {
    x: isize,
    y: isize,
}

fn parse(input: &str) -> (HashMap<u8, Vec<P>>, isize) {
    let mut antenas = HashMap::<u8, Vec<P>>::new();
    input.trim().lines().enumerate().for_each(|(x, l)| {
        l.bytes().enumerate().for_each(|(y, b)| match b {
            b'.' => (),
            c => {
                antenas.entry(c).or_default().push(P {
                    x: x as isize,
                    y: y as isize,
                });
            }
        })
    });

    (antenas, input.lines().count() as isize)
}

fn add_in_bound(lhs: (isize, isize), rhs: (isize, isize), bound: isize) -> Option<P> {
    let (x, y) = (lhs.0 + rhs.0, lhs.1 + rhs.1);

    (x >= 0 && y >= 0 && x < bound && y < bound).then_some(P { x, y })
}

pub fn part1(input: &str) -> Num {
    let (antenas, max_size) = parse(input);
    let mut antinodes = HashSet::<P>::new();

    antenas.into_iter().for_each(|(_, v)| {
        v.into_iter().permutations(2).for_each(|p| {
            let P { x: x0, y: y0 } = p[0];
            let P { x: x1, y: y1 } = p[1];

            let (dx, dy) = (x1 - x0, y1 - y0);

            add_in_bound((x1, y1), (dx, dy), max_size).map(|p| antinodes.insert(p));
            add_in_bound((x0, y0), (-dx, -dy), max_size).map(|p| antinodes.insert(p));
        });
    });

    antinodes.len()
}

pub fn part2(input: &str) -> Num {
    let (antenas, max_size) = parse(input);
    let mut antinodes = HashSet::<P>::new();

    antenas.into_iter().for_each(|(_, v)| {
        v.into_iter().permutations(2).for_each(|p| {
            let P { x: x0, y: y0 } = p[0];
            let P { x: x1, y: y1 } = p[1];

            let (dx, dy) = (x1 - x0, y1 - y0);

            let (mut x, mut y) = (x1, y1);
            antinodes.insert(P { x, y });
            while let Some(p) = add_in_bound((x, y), (dx, dy), max_size) {
                x = p.x;
                y = p.y;
                antinodes.insert(p);
            }
            let (mut x, mut y) = (x0, y0);
            antinodes.insert(P { x, y });
            while let Some(p) = add_in_bound((x, y), (-dx, -dy), max_size) {
                x = p.x;
                y = p.y;
                antinodes.insert(p);
            }
        });
    });

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
        "
    };

    #[test]
    fn example() {
        assert_eq!(part1(EXAMPLE), 14);
        assert_eq!(part2(EXAMPLE), 34);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 295);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, 1034);
    }
}
