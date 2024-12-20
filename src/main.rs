use aoc24::*;
use clap::Parser;
use paste::paste;
use std::time::Instant;

const N_DAYS: u8 = 25;

#[derive(Parser, Debug)]
pub struct Args {
    /// Day to run, if not specified all days will be run
    #[arg(short, long)]
    pub day: Option<u8>,
}

macro_rules! run_day {
    ($day:expr) => {{
        paste! {
            let input = include_str!(concat!("../inputs/day", stringify!($day), ".txt"));
            println!("Running day {}", $day);

            let start = Instant::now();
            let part1_result = [<day $day>]::part1(input);
            let elapsed1 = start.elapsed();
            println!("Part1:\n{} ({:.2?})", part1_result, elapsed1);

            let start = Instant::now();
            let part2_result = [<day $day>]::part2(input);
            let elapsed2 = start.elapsed();
            println!("Part2:\n{} ({:.2?})", part2_result, elapsed2);

            println!();
        }
    }};
}

fn run_day(day: u8) {
    if day == 0 || day > N_DAYS {
        eprintln!("Invalid day: {}. Must be between 1 and {}.", day, N_DAYS);
        return;
    }

    #[allow(clippy::zero_prefixed_literal)]
    match day {
        1 => run_day!(01),
        2 => run_day!(02),
        3 => run_day!(03),
        4 => run_day!(04),
        5 => run_day!(05),
        6 => run_day!(06),
        7 => run_day!(07),
        8 => run_day!(08),
        9 => run_day!(09),
        10 => run_day!(10),
        11 => run_day!(11),
        12 => run_day!(12),
        13 => run_day!(13),
        14 => run_day!(14),
        15 => run_day!(15),
        16 => run_day!(16),
        17 => run_day!(17),
        18 => run_day!(18),
        19 => run_day!(19),
        20 => run_day!(20),
        // 21 => run_day!(21),
        // 22 => run_day!(22),
        // 23 => run_day!(23),
        // 24 => run_day!(24),
        // 25 => run_day!(25),
        _ => {}
    }
}

fn run_all_days() {
    println!("Running all days\n");

    let start = Instant::now();
    for day in 1..=N_DAYS {
        run_day(day);
    }
    let elapsed = start.elapsed();
    println!("Ran all days in {:.2?}", elapsed);
}

fn main() {
    let args = Args::parse();

    match args.day {
        Some(day) => run_day(day),
        None => run_all_days(),
    }
}
