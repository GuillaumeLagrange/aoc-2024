type Num = usize;

pub fn part1(input: &str) -> Num {
    let line = input.trim().lines().next().unwrap();

    let input_size = line.len();

    // Id if file is its index *2
    let mut file_sizes = Vec::with_capacity(input_size / 2);
    let mut empty_space_sizes = Vec::with_capacity(input_size / 2);

    line.chars().enumerate().for_each(|(i, b)| {
        let b = b.to_digit(10).unwrap();
        match i & 1 {
            0 => {
                // This is a file
                file_sizes.push(b as i32);
            }
            1 => {
                // This is an empty space
                empty_space_sizes.push(b as i32);
            }
            _ => unreachable!(),
        }
    });

    let mut sum = 0;
    let mut counting_index = 0;

    let mut index = 0;

    #[derive(Debug)]
    struct RightFileState {
        index: usize,
        /// Partially copied file
        leftover: i32,
    }
    let mut right_file_state = RightFileState {
        index: file_sizes.len(),
        leftover: 0,
    };

    loop {
        // Check if we have no remaining files to copy
        if index >= right_file_state.index {
            for _ in 0..right_file_state.leftover {
                sum += counting_index * (index);
                counting_index += 1;
            }
            break;
        }

        // 1. Get the file at the current index and count it
        for _ in 0..file_sizes[index] {
            sum += counting_index * (index);
            counting_index += 1;
        }

        // 2. Fill the empty space with the current rightmost file
        for _ in 0..empty_space_sizes[index] {
            match right_file_state.leftover {
                0 => {
                    // We have no ongoing file to copy, find the next one
                    right_file_state.index -= 1;
                    if right_file_state.index + 1 == index + 1 {
                        // We have no more files to copy
                        break;
                    }
                    let file = file_sizes[right_file_state.index];
                    if file == 0 {
                        // Skip empty files
                        panic!("MANAGE EMPTY FILES");
                    }
                    sum += counting_index * (right_file_state.index);
                    right_file_state.leftover = file - 1;
                    counting_index += 1;
                }
                _ => {
                    // We continue copying the current file
                    sum += counting_index * (right_file_state.index);
                    right_file_state.leftover -= 1;
                    counting_index += 1;
                }
            }
        }

        index += 1;
    }

    sum
}

pub fn part2(input: &str) -> Num {
    let line = input.trim().lines().next().unwrap();

    let input_size = line.len();

    // Id if file is its index *2
    #[derive(Debug)]
    struct File {
        index: usize,
        size: usize,
        moved: bool,
    }
    #[derive(Debug)]
    struct EmptySpace {
        size: usize,
        filled_by_file_indexes: Vec<usize>,
    }
    let mut file_sizes = Vec::with_capacity(input_size / 2);
    let mut empty_spaces = Vec::with_capacity(input_size / 2);

    line.chars().enumerate().for_each(|(i, b)| {
        let b = b.to_digit(10).unwrap();
        match i & 1 {
            0 => {
                file_sizes.push(File {
                    size: b as usize,
                    index: i / 2,
                    moved: false,
                });
            }
            1 => {
                empty_spaces.push(EmptySpace {
                    size: b as usize,
                    filled_by_file_indexes: Vec::new(),
                });
            }
            _ => unreachable!(),
        }
    });

    // Go through all files from right to left
    // For each file, find the leftmost empty space that fits the file
    // Do not move the file if we reach the current file index
    // Stop when we have attempted to move all files once
    for file in file_sizes.iter_mut().rev() {
        if let Some(empty_space) = empty_spaces[0..file.index]
            .iter_mut()
            .find(|es| es.size >= file.size)
        {
            empty_space.filled_by_file_indexes.push(file.index);
            empty_space.size -= file.size;
            file.moved = true;

            empty_spaces[file.index - 1].size += file.size;
        }
    }

    let mut sum = 0;
    let mut counting_index = 0;
    for i in 0..file_sizes.len() {
        let file = &file_sizes[i];
        if !file.moved {
            for _ in 0..file.size {
                sum += counting_index * file.index;
                counting_index += 1;
            }
        }

        if i == empty_spaces.len() {
            continue;
        }

        let empty_space = &empty_spaces[i];

        for moved_file_index in empty_space.filled_by_file_indexes.iter() {
            let file = &file_sizes[*moved_file_index];
            for _ in 0..file.size {
                sum += counting_index * file.index;
                counting_index += 1;
            }
        }

        for _ in 0..empty_space.size {
            counting_index += 1;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = indoc::indoc! {"
        2333133121414131402
"};

    #[test]
    fn example() {
        assert_eq!(part1(EXAMPLE), 1928);
        assert_eq!(part2(EXAMPLE), 2858);
    }

    #[test]
    fn sample() {
        let sample = indoc::indoc! {"
          12345
"};
        assert_eq!(part1(sample), 60);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 6471961544878);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, 6511178035564);
    }
}
