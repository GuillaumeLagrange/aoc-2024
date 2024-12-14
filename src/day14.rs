use std::collections::HashMap;

use arrayvec::ArrayVec;

type Num = i32;

const GUARD_COUNT: usize = 500;

const EXAMPLE_WIDTH: i32 = 11;
const EXAMPLE_HEIGHT: i32 = 7;

const INPUT_WIDTH: i32 = 101;
const INPUT_HEIGHT: i32 = 103;

#[derive(Debug)]
struct Guard {
    position: (Num, Num),
    speed: (i32, i32),
}

fn parse(input: &str) -> ArrayVec<Guard, GUARD_COUNT> {
    input
        .lines()
        .map(|line| {
            let (p_str, v_str) = line.split_once(" ").unwrap();

            let (p_x, p_y) = p_str[2..]
                .split_once(",")
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap();

            let (v_x, v_y) = v_str[2..]
                .split_once(",")
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap();

            Guard {
                position: (p_x, p_y),
                speed: (v_x, v_y),
            }
        })
        .collect()
}

pub fn part1(input: &str) -> Num {
    part1_inner(input, INPUT_WIDTH, INPUT_HEIGHT)
}

pub fn part1_example(input: &str) -> Num {
    part1_inner(input, EXAMPLE_WIDTH, EXAMPLE_HEIGHT)
}

fn part1_inner(input: &str, width: i32, height: i32) -> Num {
    let mut guards = parse(input);

    (0..100).for_each(|_| {
        guards.iter_mut().for_each(|guard| {
            let new_x = guard.position.0 + guard.speed.0;
            if new_x < 0 {
                guard.position.0 = new_x + width;
            } else if new_x >= width {
                guard.position.0 = new_x - width;
            } else {
                guard.position.0 = new_x;
            }

            let new_y = guard.position.1 + guard.speed.1;
            if new_y < 0 {
                guard.position.1 = new_y + height;
            } else if new_y >= height {
                guard.position.1 = new_y - height;
            } else {
                guard.position.1 = new_y;
            }
        });
    });

    #[derive(Default)]
    struct Count {
        top_left: i32,
        top_right: i32,
        bottom_left: i32,
        bottom_right: i32,
    }
    let count = guards
        .into_iter()
        .fold(Count::default(), |mut count, guard| {
            use std::cmp::Ordering;
            let x_border = width / 2;
            let y_border = height / 2;

            match (
                x_border.cmp(&guard.position.0),
                y_border.cmp(&guard.position.1),
            ) {
                (Ordering::Less, Ordering::Less) => count.top_right += 1,
                (Ordering::Less, Ordering::Greater) => count.bottom_right += 1,
                (Ordering::Greater, Ordering::Less) => count.top_left += 1,
                (Ordering::Greater, Ordering::Greater) => count.bottom_left += 1,
                _ => {}
            }

            count
        });

    count.top_left * count.bottom_right * count.top_right * count.bottom_left
}

#[allow(dead_code)]
fn print_grid(guards: &[Guard], width: i32, height: i32) {
    let mut grid = vec![vec!['.'; width as usize]; height as usize];

    guards.iter().for_each(|guard| {
        grid[guard.position.1 as usize][guard.position.0 as usize] = '#';
    });

    grid.iter().for_each(|row| {
        println!("{}", row.iter().collect::<String>());
    });
}

fn check_easter_egg(guards: &[Guard]) -> bool {
    // Count robots x values and trigger
    let (x_count, y_count) = guards.iter().fold(
        (HashMap::new(), HashMap::new()),
        |(mut x_counts, mut y_counts), guard| {
            let x_count = x_counts.entry(guard.position.0).or_insert(0);
            *x_count += 1;

            let y_count = y_counts.entry(guard.position.1).or_insert(0);
            *y_count += 1;

            (x_counts, y_counts)
        },
    );

    x_count.iter().any(|(_, count)| *count >= 35) && y_count.iter().any(|(_, count)| *count >= 25)
}

pub fn part2(input: &str) -> Num {
    part2_inner(input, INPUT_WIDTH, INPUT_HEIGHT)
}

pub fn part2_example(input: &str) -> Num {
    part2_inner(input, EXAMPLE_WIDTH, EXAMPLE_HEIGHT)
}

fn part2_inner(input: &str, width: i32, height: i32) -> Num {
    let mut guards = parse(input);

    let mut count = 0;
    loop {
        count += 1;

        guards.iter_mut().for_each(|guard| {
            let new_x = guard.position.0 + guard.speed.0;
            if new_x < 0 {
                guard.position.0 = new_x + width;
            } else if new_x >= width {
                guard.position.0 = new_x - width;
            } else {
                guard.position.0 = new_x;
            }

            let new_y = guard.position.1 + guard.speed.1;
            if new_y < 0 {
                guard.position.1 = new_y + height;
            } else if new_y >= height {
                guard.position.1 = new_y - height;
            } else {
                guard.position.1 = new_y;
            }
        });

        if check_easter_egg(&guards) {
            return count;

            // print_grid(&guards, width, height);
            // println!("Count: {count}");
            // println!("Press Enter to continue...");
            // let mut buffer = String::new();
            // std::io::stdin().read_line(&mut buffer).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = indoc::indoc! {"
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"};

    #[test]
    fn example() {
        assert_eq!(part1_example(EXAMPLE), 12);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 217328832);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 7412);
    }
}
