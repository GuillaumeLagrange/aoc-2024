#[macro_export]
macro_rules! get_day_input {
    () => {{
        let filepath = file!();
        let day: u32 = filepath
            .strip_prefix("src/day")
            .and_then(|s| s.strip_suffix(".rs"))
            .and_then(|s| s.parse().ok())
            .expect("unable to parse the day");
        let path = format!("./inputs/day{}.txt", day);
        let data = std::fs::read_to_string(path).expect("Could not open file");
        data
    }};
}

#[cfg(test)]
pub use get_day_input;
