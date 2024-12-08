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

pub fn number_of_digits_u64(n: u64) -> u32 {
    (n as f64).log10().floor() as u32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_digits() {
        assert_eq!(number_of_digits_u64(0), 1);
        assert_eq!(number_of_digits_u64(1), 1);
        assert_eq!(number_of_digits_u64(10), 2);
        assert_eq!(number_of_digits_u64(100), 3);
        assert_eq!(number_of_digits_u64(1000), 4);
        assert_eq!(number_of_digits_u64(10000), 5);
        assert_eq!(number_of_digits_u64(100000), 6);
        assert_eq!(number_of_digits_u64(1023450), 7);
        assert_eq!(number_of_digits_u64(10000000), 8);
        assert_eq!(number_of_digits_u64(100456000), 9);
        assert_eq!(number_of_digits_u64(1000001230), 10);
    }
}
