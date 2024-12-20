pub use manhattan::*;

mod manhattan;

#[macro_export]
macro_rules! get_day_input {
    () => {{
        let filepath = file!();
        let day: u32 = filepath
            .strip_prefix("src/day")
            .and_then(|s| s.strip_suffix(".rs"))
            .and_then(|s| s.parse().ok())
            .expect("unable to parse the day");
        let path = format!("./inputs/day{:0>2}.txt", day);
        let data = std::fs::read_to_string(path).expect("Could not open file");
        data
    }};
}

#[cfg(test)]
pub use get_day_input;

pub fn number_of_digits_u64(n: u64) -> u32 {
    (n as f64).log10().floor() as u32 + 1
}

pub trait CoordinateAddIter<const N: usize> {
    /// Does not check for out of bounds
    fn coord_add_iter(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)>;
}

impl<const N: usize> CoordinateAddIter<N> for [(i32, i32); N] {
    fn coord_add_iter(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .map(move |(dx, dy)| ((x as i32 + dx) as usize, (y as i32 + dy) as usize))
    }
}

pub const CARDINAL_DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub trait CoordinateAdd {
    /// Does not check for out of bounds
    fn coord_add(&self, coord: (usize, usize)) -> (usize, usize);
}

impl CoordinateAdd for (i32, i32) {
    fn coord_add(&self, (x, y): (usize, usize)) -> (usize, usize) {
        ((x as i32 + self.0) as usize, (y as i32 + self.1) as usize)
    }
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

    #[test]
    fn coord_diff() {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        itertools::assert_equal(
            directions.coord_add_iter(1, 1),
            vec![(1, 2), (2, 1), (1, 0), (0, 1)],
        );
        itertools::assert_equal(
            directions.coord_add_iter(2, 1),
            vec![(2, 2), (3, 1), (2, 0), (1, 1)],
        );
    }
}
