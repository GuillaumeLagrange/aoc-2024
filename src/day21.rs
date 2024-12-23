use std::{cmp::Ordering, collections::HashMap};

type Num = usize;

/// +---+---+---+
/// | 7 | 8 | 9 |
/// +---+---+---+
/// | 4 | 5 | 6 |
/// +---+---+---+
/// | 1 | 2 | 3 |
/// +---+---+---+
///     | 0 | A |
///     +---+---+
const NUMERIC_KEYPAD: [[Option<u8>; 3]; 4] = [
    [Some(b'7'), Some(b'8'), Some(b'9')],
    [Some(b'4'), Some(b'5'), Some(b'6')],
    [Some(b'1'), Some(b'2'), Some(b'3')],
    [None, Some(b'0'), Some(b'A')],
];

fn get_keypad_path(from: u8, to: u8) -> Vec<Vec<u8>> {
    match (from, to) {
        // Starting from `<`
        (b'<', b'v') => vec![vec![b'>']],
        (b'<', b'>') => vec![vec![b'>', b'>']],
        (b'<', b'^') => vec![vec![b'>', b'^']],
        (b'<', b'A') => vec![vec![b'>', b'>', b'^']],

        // Starting from `v`
        (b'v', b'<') => vec![vec![b'<']],
        (b'v', b'>') => vec![vec![b'>']],
        (b'v', b'^') => vec![vec![b'^']],
        (b'v', b'A') => vec![vec![b'>', b'^'], vec![b'^', b'>']],

        // Starting from `^`
        (b'^', b'<') => vec![vec![b'v', b'<']],
        (b'^', b'>') => vec![vec![b'>', b'v'], vec![b'v', b'>']],
        (b'^', b'v') => vec![vec![b'v']],
        (b'^', b'A') => vec![vec![b'>']],

        // Starting from `<`
        (b'>', b'<') => vec![vec![b'<', b'<']],
        (b'>', b'^') => vec![vec![b'<', b'^'], vec![b'^', b'<']],
        (b'>', b'v') => vec![vec![b'<']],
        (b'>', b'A') => vec![vec![b'^']],

        // Starting from `A`
        (b'A', b'^') => vec![vec![b'<']],
        (b'A', b'v') => vec![vec![b'<', b'v'], vec![b'v', b'<']],
        (b'A', b'>') => vec![vec![b'v']],
        (b'A', b'<') => vec![vec![b'v', b'<', b'<']],

        _ => vec![],
    }
}

fn get_numeric_keypad_path(from: u8, to: u8) -> Vec<Vec<u8>> {
    if from == to {
        return Vec::new();
    }

    let (i_from, j_from, i_to, j_to) = {
        let mut i_from = None;
        let mut j_from = None;
        let mut i_to = None;
        let mut j_to = None;

        for (i, l) in NUMERIC_KEYPAD.iter().enumerate() {
            for (j, b) in l.iter().enumerate() {
                if let Some(b) = b {
                    if *b == from {
                        i_from = Some(i);
                        j_from = Some(j);
                    }
                    if *b == to {
                        i_to = Some(i);
                        j_to = Some(j);
                    }
                }
            }
        }

        (
            i_from.unwrap(),
            j_from.unwrap(),
            i_to.unwrap(),
            j_to.unwrap(),
        )
    };

    let horizontal_path = match j_to.cmp(&j_from) {
        Ordering::Less => Some(vec![b'<'; j_from - j_to]),
        Ordering::Greater => Some(vec![b'>'; j_to - j_from]),
        Ordering::Equal => None,
    };

    let vertical_path = match i_to.cmp(&i_from) {
        Ordering::Less => Some(vec![b'^'; i_from - i_to]),
        Ordering::Greater => Some(vec![b'v'; i_to - i_from]),
        Ordering::Equal => None,
    };

    // Avoid the gap
    let vertical_first_path = match vertical_path.clone() {
        Some(v) => {
            if i_to == 3 && j_from == 0 {
                None
            } else {
                Some(
                    v.into_iter()
                        .chain(horizontal_path.clone().unwrap_or_default())
                        .collect::<Vec<_>>(),
                )
            }
        }
        None => None,
    };

    let horizontal_first_path = match horizontal_path {
        Some(h) => {
            if j_to == 0 && i_from == 3 {
                None
            } else {
                Some(
                    h.into_iter()
                        .chain(vertical_path.clone().unwrap_or_default())
                        .collect::<Vec<_>>(),
                )
            }
        }
        None => None,
    };

    vec![vertical_first_path, horizontal_first_path]
        .into_iter()
        .flatten()
        .collect()
}

fn get_shortest_directional_sequence(
    keys: &[u8],
    depth: usize,
    cache: &mut HashMap<(Vec<u8>, usize), usize>,
) -> usize {
    if depth == 0 {
        return keys.len();
    }

    if let Some(len) = cache.get(&(keys.to_vec(), depth)) {
        return *len;
    }

    let mut total = 0;
    // Cut up the sequence between segments separated by A presses and recursively look for
    // shortest sequence for each one
    let sub_directionnal_sequences = keys.split_inclusive(|k| *k == b'A');

    for sub_directionnal_sequence in sub_directionnal_sequences {
        let mut sequences = Vec::new();

        build_sequence_directional(
            sub_directionnal_sequence,
            0,
            b'A',
            Vec::new(),
            &mut sequences,
        );

        let mut min = usize::MAX;
        for sequence in sequences.iter() {
            let len = get_shortest_directional_sequence(sequence, depth - 1, cache);
            min = len.min(min);
        }

        total += min;
    }

    cache.insert((keys.to_vec(), depth), total);

    total
}

fn build_sequence_directional(
    keys: &[u8],
    index: usize,
    previous_key: u8,
    mut current_path: Vec<u8>,
    result: &mut Vec<Vec<u8>>,
) {
    if index == keys.len() {
        result.push(current_path);
        return;
    }

    let paths = get_keypad_path(previous_key, keys[index]);

    if paths.is_empty() {
        current_path.push(b'A');
        build_sequence_directional(keys, index + 1, keys[index], current_path, result);
        return;
    }

    for path in paths {
        let mut new_path = current_path.clone();
        new_path.extend_from_slice(&path);
        new_path.push(b'A');
        build_sequence_directional(keys, index + 1, keys[index], new_path, result);
    }
}

fn build_sequence_numeric(
    keys: &[u8],
    index: usize,
    previous_key: u8,
    mut current_path: Vec<u8>,
    result: &mut Vec<Vec<u8>>,
) {
    if index == keys.len() {
        result.push(current_path);
        return;
    }

    let paths = get_numeric_keypad_path(previous_key, keys[index]);

    if paths.is_empty() {
        current_path.push(b'A');
        build_sequence_directional(keys, index + 1, keys[index], current_path, result);
        return;
    }

    for path in paths {
        let mut new_path = current_path.clone();
        new_path.extend_from_slice(&path);
        new_path.push(b'A');
        build_sequence_numeric(keys, index + 1, keys[index], new_path, result);
    }
}

pub fn part1(input: &str) -> Num {
    input
        .lines()
        .map(|l| {
            let keys = l.as_bytes();
            let number_to_type = l[..l.len() - 1].parse::<usize>().unwrap();
            (number_to_type, keys)
        })
        .fold(0, |acc, (number_to_type, keys)| {
            let mut keys_directionals = Vec::new();

            build_sequence_numeric(keys, 0, b'A', Vec::new(), &mut keys_directionals);

            let min_sequence_len = keys_directionals
                .into_iter()
                .map(|k| get_shortest_directional_sequence(&k, 2, &mut HashMap::new()))
                .min()
                .unwrap();

            acc + min_sequence_len * number_to_type
        })
}

pub fn part2(input: &str) -> Num {
    input
        .lines()
        .map(|l| {
            let keys = l.as_bytes();
            let number_to_type = l[..l.len() - 1].parse::<usize>().unwrap();
            (number_to_type, keys)
        })
        .fold(0, |acc, (number_to_type, keys)| {
            let mut keys_directionals = Vec::new();

            build_sequence_numeric(keys, 0, b'A', Vec::new(), &mut keys_directionals);

            let min_sequence_len = keys_directionals
                .into_iter()
                .map(|k| get_shortest_directional_sequence(&k, 25, &mut HashMap::new()))
                .min()
                .unwrap();

            acc + min_sequence_len * number_to_type
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = indoc::indoc! {"
029A
980A
179A
456A
379A
"};

    #[test]
    fn example() {
        assert_eq!(part1(EXAMPLE), 126384);
        assert_eq!(part2(EXAMPLE), 154115708116294);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 205160);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, 252473394928452);
    }

    #[test]
    fn test_get_numeric_keypad_path() {
        assert_eq!(get_numeric_keypad_path(b'1', b'1').len(), 0);
        assert_eq!(get_numeric_keypad_path(b'1', b'3'), vec![vec![b'>', b'>']]);
        assert_eq!(
            get_numeric_keypad_path(b'A', b'5'),
            vec![vec![b'^', b'^', b'<'], vec![b'<', b'^', b'^']]
        );
        assert_eq!(
            get_numeric_keypad_path(b'5', b'A'),
            vec![vec![b'v', b'v', b'>'], vec![b'>', b'v', b'v']]
        );
        assert_eq!(
            get_numeric_keypad_path(b'A', b'4'),
            vec![vec![b'^', b'^', b'<', b'<']]
        );
        assert_eq!(
            get_numeric_keypad_path(b'4', b'A'),
            vec![vec![b'>', b'>', b'v', b'v']]
        );
    }

    #[test]
    fn test_build_sequence_directional() {
        let mut sequences = Vec::new();
        build_sequence_directional(b"<A", 0, b'A', Vec::new(), &mut sequences);
        assert_eq!(
            sequences,
            vec![vec![b'v', b'<', b'<', b'A', b'>', b'>', b'^', b'A']]
        );

        let mut sequences = Vec::new();
        build_sequence_directional(b"v<<A", 0, b'A', Vec::new(), &mut sequences);
        assert_eq!(
            sequences,
            vec![b"<vA<AA>>^A".to_vec(), b"v<A<AA>>^A".to_vec()]
        );
    }

    #[test]
    fn test_get_shortest_directional_sequence() {
        assert_eq!(
            get_shortest_directional_sequence(b"<A", 1, &mut HashMap::new()),
            8
        );
        assert_eq!(
            get_shortest_directional_sequence(b"<A", 2, &mut HashMap::new()),
            18
        );
    }

    #[test]
    fn test_build_sequence_numeric() {
        let mut sequences = Vec::new();
        build_sequence_numeric(b"26", 0, b'A', Vec::new(), &mut sequences);
        assert_eq!(
            sequences,
            vec![
                b"^<A^>A".to_vec(),
                b"^<A>^A".to_vec(),
                b"<^A^>A".to_vec(),
                b"<^A>^A".to_vec()
            ]
        );

        let mut sequences = Vec::new();
        build_sequence_directional(b"v<<A", 0, b'A', Vec::new(), &mut sequences);
        assert_eq!(
            sequences,
            vec![b"<vA<AA>>^A".to_vec(), b"v<A<AA>>^A".to_vec()]
        );
    }
}
