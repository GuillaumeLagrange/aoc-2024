pub trait ManhattanDistance {
    /// Returns an iterator of coordinates within a manhattan distance of `distance` from `self`, excluding out of bound coordinates.
    /// `min_bound` and `distance` is inclusive and `max_bound` is exclusive.
    /// ```
    /// use aoc24::utils::ManhattanDistance;
    ///
    /// let result = (1, 1).at_manhattan_distance(1, 0, 4).collect::<Vec<_>>();
    /// assert_eq!(result, vec![(1, 2), (2, 1), (1, 0), (0, 1)]);
    ///
    /// let result = (1, 1).at_manhattan_distance(2, 0, 4).collect::<Vec<_>>();
    /// assert_eq!(result, vec![(1, 3), (3, 1), (2, 2), (2, 0), (0, 0), (0, 2)]);
    /// ```
    fn at_manhattan_distance(
        &self,
        distance: usize,
        min_bound: usize,
        max_bound: usize,
    ) -> impl Iterator<Item = (usize, usize)>;

    /// Returns an iterator of coordinates within a manhattan distance of `distance` from `self`, excluding out of bound coordinates.
    /// `min_bound` and `distance` is inclusive and `max_bound` is exclusive.
    /// ```
    /// use aoc24::utils::ManhattanDistance;
    ///
    /// let result = (1, 1)
    ///     .within_manhattan_distance(2, 0, 4)
    ///     .collect::<Vec<_>>();
    /// assert_eq!(
    ///     result,
    ///     vec![
    ///         (1, 2),
    ///         (2, 1),
    ///         (1, 0),
    ///         (0, 1),
    ///         (1, 3),
    ///         (3, 1),
    ///         (2, 2),
    ///         (2, 0),
    ///         (0, 0),
    ///         (0, 2)
    ///     ]
    /// );
    /// ```
    fn within_manhattan_distance(
        &self,
        distance: usize,
        min_bound: usize,
        max_bound: usize,
    ) -> impl Iterator<Item = (usize, usize)>;
}

impl ManhattanDistance for (usize, usize) {
    fn at_manhattan_distance(
        &self,
        distance: usize,
        min_bound: usize,
        max_bound: usize,
    ) -> impl Iterator<Item = (usize, usize)> {
        let distance = distance as isize;
        let min_bound = min_bound as isize;
        let max_bound = max_bound as isize;
        (0..distance)
            .flat_map(move |offset| {
                let inv_offset = distance - offset;
                let x = self.0 as isize;
                let y = self.1 as isize;

                [
                    (x + offset, y + inv_offset),
                    (x + inv_offset, y - offset),
                    (x - offset, y - inv_offset),
                    (x - inv_offset, y + offset),
                ]
                .into_iter()
            })
            .filter_map(move |(x, y)| {
                (x >= min_bound && x < max_bound && y >= min_bound && y < max_bound)
                    .then_some((x as usize, y as usize))
            })
    }

    fn within_manhattan_distance(
        &self,
        distance: usize,
        min_bound: usize,
        max_bound: usize,
    ) -> impl Iterator<Item = (usize, usize)> {
        (0..=distance).flat_map(move |d| self.at_manhattan_distance(d, min_bound, max_bound))
    }
}
