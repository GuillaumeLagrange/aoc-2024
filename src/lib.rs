pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

extern crate aoc_runner;

// Disable dead code warning for this module
#[allow(dead_code)]
pub(crate) mod utils;

#[macro_use]
extern crate aoc_runner_derive;
extern crate crypto;

aoc_lib! { year = 2024 }
