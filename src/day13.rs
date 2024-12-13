use arrayvec::ArrayVec;

type Num = i64;

const INPUT_SIZE: usize = 400;

#[derive(Debug)]
struct Equation {
    a: Num,
    b: Num,
    p: Num,
}

#[derive(Debug)]
struct System {
    x: Equation,
    y: Equation,
}

fn parse(input: &str) -> ArrayVec<System, INPUT_SIZE> {
    input
        .split("\n\n")
        .map(|group| {
            let mut group_lines = group.lines();
            let regex = regex::Regex::new(r".*X.(?<x>\d+), Y.(?<y>\d+)").unwrap();

            let (a_x, a_y) = group_lines
                .next()
                .and_then(|line| regex.captures(line))
                .map(|caps| {
                    (
                        caps.name("x").unwrap().as_str().parse::<Num>().unwrap(),
                        caps.name("y").unwrap().as_str().parse::<Num>().unwrap(),
                    )
                })
                .unwrap();

            let (b_x, b_y) = group_lines
                .next()
                .and_then(|line| regex.captures(line))
                .map(|caps| {
                    (
                        caps.name("x").unwrap().as_str().parse::<Num>().unwrap(),
                        caps.name("y").unwrap().as_str().parse::<Num>().unwrap(),
                    )
                })
                .unwrap();

            let (p_x, p_y) = group_lines
                .next()
                .and_then(|line| regex.captures(line))
                .map(|caps| {
                    (
                        caps.name("x").unwrap().as_str().parse::<Num>().unwrap(),
                        caps.name("y").unwrap().as_str().parse::<Num>().unwrap(),
                    )
                })
                .unwrap();

            System {
                x: Equation {
                    a: a_x,
                    b: b_x,
                    p: p_x,
                },
                y: Equation {
                    a: a_y,
                    b: b_y,
                    p: p_y,
                },
            }
        })
        .collect::<ArrayVec<System, INPUT_SIZE>>()
}

pub fn part1(input: &str) -> Num {
    let systems = parse(input);

    systems
        .into_iter()
        .map(|System { x, y }| {
            let determinant: i64 = x.a * y.b - x.b * y.a;

            if determinant != 0
                && (y.b * x.p - x.b * y.p) % determinant == 0
                && (x.a * y.p - y.a * x.p) % determinant == 0
            {
                let a = (y.b * x.p - x.b * y.p) / determinant;
                let b = (x.a * y.p - y.a * x.p) / determinant;

                if a <= 100 && b <= 100 {
                    return 3 * a + b;
                }
            }

            0
        })
        .sum()
}

pub fn part2(input: &str) -> Num {
    let systems = parse(input);

    systems
        .into_iter()
        .map(|System { x, y }| {
            let determinant: i64 = x.a * y.b - x.b * y.a;

            const PART_2_OFFSET: i64 = 10000000000000;
            let x_p = x.p + PART_2_OFFSET;
            let y_p = y.p + PART_2_OFFSET;

            if determinant != 0
                && (y.b * x_p - x.b * y_p) % determinant == 0
                && (x.a * y_p - y.a * x_p) % determinant == 0
            {
                let a = (y.b * x_p - x.b * y_p) / determinant;
                let b = (x.a * y_p - y.a * x_p) / determinant;

                return 3 * a + b;
            }

            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = indoc::indoc! {"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"};

    #[test]
    fn example() {
        assert_eq!(part1(EXAMPLE), 480);
        assert_eq!(part2(EXAMPLE), 875318608908);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 33209);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, 83102355665474);
    }
}
