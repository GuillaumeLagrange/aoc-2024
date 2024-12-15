// pub(crate) trait AdjacentPositions {
//     /// Bound is exclusive
//     fn adjacent_positions(self, bound: Self) -> impl IntoIterator<Item = Self>;
// }
//
// impl AdjacentPositions for (usize, usize) { fn adjacent_positions(self, bound: Self) -> impl IntoIterator<Item = (usize, usize)> { let directions = [
//             (-1, -1),
//             (-1, 0),
//             (-1, 1),
//             (0, -1),
//             (0, 1),
//             (1, -1),
//             (1, 0),
//             (1, 1),
//         ];
//
//         let i = self.0 as isize;
//         let j = self.1 as isize;
//
//         directions.into_iter().filter_map(move |(di, dj)| {
//             let i = i + di;
//             let j = j + dj;
//
//             // Check bounds and convert back to usize if valid
//             if i >= 0 && i < bound.0 as isize && j >= 0 && j < bound.1 as isize {
//                 Some((i as usize, j as usize))
//             } else {
//                 None
//             }
//         })
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     /// . . .
//     /// . X .
//     /// . . .
//     fn center() {
//         let bound = (3, 3);
//         let positions = (1, 1)
//             .adjacent_positions(bound)
//             .into_iter()
//             .collect::<Vec<_>>();
//         assert_eq!(
//             positions,
//             vec![
//                 (0, 0),
//                 (0, 1),
//                 (0, 2),
//                 (1, 0),
//                 (1, 2),
//                 (2, 0),
//                 (2, 1),
//                 (2, 2)
//             ]
//         );
//     }
//
//     #[test]
//     /// X . .
//     /// . . .
//     /// . . .
//     fn top_left() {
//         let bound = (3, 3);
//         let positions = (0, 0)
//             .adjacent_positions(bound)
//             .into_iter()
//             .collect::<Vec<_>>();
//         assert_eq!(positions, vec![(0, 1), (1, 0), (1, 1)]);
//     }
//
//     #[test]
//     /// . . .
//     /// . . .
//     /// . . X
//     fn bottom_right() {
//         let bound = (3, 3);
//         let mut positions = (2, 2).adjacent_positions(bound).into_iter();
//         assert!(positions.all(|pos| [(1, 1), (1, 2), (2, 1)].contains(&pos)));
//     }
//
//     #[test]
//     /// . . .
//     /// . . .
//     /// . X .
//     fn bottom() {
//         let bound = (3, 3);
//         let mut positions = (2, 1).adjacent_positions(bound).into_iter();
//
//         assert!(positions.all(|pos| [(1, 0), (2, 0), (1, 1), (1, 2), (2, 2)].contains(&pos)));
//     }
//
//     #[test]
//     /// . X .
//     /// . . .
//     /// . . .
//     fn top() {
//         let bound = (3, 3);
//         let mut positions = (0, 1).adjacent_positions(bound).into_iter();
//
//         assert!(positions.all(|pos| [(0, 0), (0, 2), (1, 0), (1, 1), (1, 2)].contains(&pos)));
//     }
//
//     #[test]
//     /// . . .
//     /// X . .
//     /// . . .
//     fn left() {
//         let bound = (3, 3);
//         let mut positions = (1, 0).adjacent_positions(bound).into_iter();
//
//         assert!(positions.all(|pos| [(0, 0), (0, 1), (1, 1), (2, 0), (2, 1)].contains(&pos)));
//     }
//
//     #[test]
//     /// . . .
//     /// . . X
//     /// . . .
//     fn right() {
//         let bound = (3, 3);
//         let mut positions = (1, 2).adjacent_positions(bound).into_iter();
//
//         assert!(positions.all(|pos| [(0, 1), (0, 2), (1, 1), (2, 1), (2, 2)].contains(&pos)));
//     }
// }
