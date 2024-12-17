use arrayvec::ArrayVec;
use itertools::Itertools;

type Num = usize;

const INSTRUCTIONS_SIZE: usize = 20;

#[derive(Debug, Default, Clone)]
struct Registers {
    a: Num,
    b: Num,
    c: Num,
    pc: Num,
    out: Vec<u8>,
}

#[derive(Debug)]
enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for OpCode {
    fn from(n: u8) -> Self {
        match n {
            0 => OpCode::Adv,
            1 => OpCode::Bxl,
            2 => OpCode::Bst,
            3 => OpCode::Jnz,
            4 => OpCode::Bxc,
            5 => OpCode::Out,
            6 => OpCode::Bdv,
            7 => OpCode::Cdv,
            _ => unreachable!(),
        }
    }
}

impl Registers {
    fn execute(&mut self, program: &[u8]) {
        let op = OpCode::from(program[self.pc]);
        let arg = match op {
            OpCode::Bxl | OpCode::Jnz => program[self.pc + 1] as usize,
            _ => match program[self.pc + 1] {
                4 => self.a,
                5 => self.b,
                6 => self.c,
                n => n as usize,
            },
        };

        match op {
            OpCode::Adv => {
                self.a >>= arg;
            }
            OpCode::Bxl => {
                self.b ^= arg;
            }
            OpCode::Bst => {
                self.b = arg & 0b111;
            }
            OpCode::Jnz => {
                if self.a != 0 {
                    self.pc = arg;
                    return; // Don't increment pc by 2 like the other instructions
                }
            }
            OpCode::Bxc => {
                self.b ^= self.c;
            }
            OpCode::Out => {
                self.out.push(arg as u8 & 0b111);
            }
            OpCode::Bdv => {
                self.b = self.a >> arg;
            }
            OpCode::Cdv => {
                self.c = self.a >> arg;
            }
        }

        self.pc += 2;
    }

    fn execute_all(&mut self, program: &[u8]) {
        while self.pc < program.len() {
            self.execute(program);
        }
    }
}

pub fn part1(input: &str) -> String {
    let mut reg = Registers::default();

    let program: ArrayVec<u8, INSTRUCTIONS_SIZE> = {
        let mut input = input.lines();
        reg.a = input
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .parse()
            .unwrap();

        // B & C are always 0, skip them and the empty line after them
        for _ in 0..3 {
            input.next();
        }

        input
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(',')
            .flat_map(|p| p.parse())
            .collect()
    };

    reg.execute_all(&program);

    reg.out.iter().join(",")
}

pub fn part2(input: &str) -> Num {
    let (reg, program): (Registers, ArrayVec<u8, INSTRUCTIONS_SIZE>) = {
        let mut reg = Registers::default();
        let mut input = input.lines();
        reg.a = input
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .parse()
            .unwrap();

        // B & C are always 0, skip them and the empty line after them
        for _ in 0..3 {
            input.next();
        }

        (
            reg,
            input
                .next()
                .unwrap()
                .split_once(": ")
                .unwrap()
                .1
                .split(',')
                .flat_map(|p| p.parse())
                .collect(),
        )
    };

    // Build the number from right to left
    let mut queue = vec![(0, program.len() - 1)];
    let mut values = Vec::new();
    while let Some((partial_input, value_to_print_index)) = queue.pop() {
        for i in 0..8 {
            let candidate = (partial_input << 3) + i;
            let mut candidate_reg = Registers {
                a: candidate,
                ..reg.clone()
            };

            candidate_reg.execute_all(&program);

            if candidate_reg.out == program[value_to_print_index..] {
                if value_to_print_index == 0 {
                    values.push(candidate);
                } else {
                    queue.push((candidate, value_to_print_index - 1));
                }
            }
        }
    }

    values.into_iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = indoc::indoc! {"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"};

    const EXAMPLE2: &str = indoc::indoc! {"
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"};

    #[test]
    fn example() {
        assert_eq!(part1(EXAMPLE), "4,6,3,5,6,3,5,2,1,0");
        assert_eq!(part2(EXAMPLE2), 117440);
    }

    #[test]
    fn ops1() {
        let mut reg = Registers {
            c: 9,
            ..Default::default()
        };
        reg.execute_all(&[2, 6]);
        assert_eq!(reg.b, 1);
    }

    #[test]
    fn ops2() {
        let mut reg = Registers {
            a: 10,
            ..Default::default()
        };
        reg.execute_all(&[5, 0, 5, 1, 5, 4]);
        assert_eq!(reg.out, vec![0, 1, 2]);
    }

    #[test]
    fn ops3() {
        let mut reg = Registers {
            a: 2024,
            ..Default::default()
        };
        reg.execute_all(&[0, 1, 5, 4, 3, 0]);
        assert_eq!(reg.a, 0);
        assert_eq!(reg.out, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    }

    #[test]
    fn ops4() {
        let mut reg = Registers {
            b: 29,
            ..Default::default()
        };
        reg.execute_all(&[1, 7]);
        assert_eq!(reg.b, 26);
    }

    #[test]
    fn ops5() {
        let mut reg = Registers {
            b: 2024,
            c: 43690,
            ..Default::default()
        };
        reg.execute_all(&[4, 0]);
        assert_eq!(reg.b, 44354);
    }

    #[test]
    fn ops6() {
        let mut reg = Registers {
            a: 117440,
            ..Default::default()
        };
        reg.execute_all(&[0, 3, 5, 4, 3, 0]);
        assert_eq!(reg.out, vec![0, 3, 5, 4, 3, 0]);
    }

    #[test]
    fn workbench() {
        let mut reg = Registers {
            a: 164545346498493,
            ..Default::default()
        };
        reg.execute_all(&[2, 4, 1, 1, 7, 5, 1, 5, 0, 3, 4, 3, 5, 5, 3, 0]);
        assert_eq!(
            reg.out,
            vec![2, 4, 1, 1, 7, 5, 1, 5, 0, 3, 4, 3, 5, 5, 3, 0]
        );
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, "4,1,5,3,1,5,3,5,7");
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, 164542125272765);
    }
}
