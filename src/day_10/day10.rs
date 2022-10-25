use std::str::FromStr;

const INPUT: &str = include_str!("./input.txt");

fn read_input(input: &str) -> Vec<usize> {
    let mut values: Vec<usize> = Vec::new();
    for v in input.split(',') {
        values.push(usize::from_str(v.trim()).unwrap());
    }
    values
}

fn read_input_as_ascii(input: &str) -> Vec<usize> {
    let mut values: Vec<usize> = Vec::new();
    let suffixes = [17, 31, 73, 47, 23];
    for v in input.chars() {
        values.push(v as usize);
    }
    for suffix in suffixes {
        values.push(suffix);
    }
    values
}

fn generate_values(input: (usize, usize)) -> Vec<usize> {
    let mut values: Vec<usize> = Vec::new();
    for v in input.0..input.1 {
        values.push(v);
    }
    values
}

pub fn run() {
    println!("=== Day 10 ===");
    part1((0, 256), INPUT);
    part2((0, 256), INPUT);
    println!("=============");
    println!();
}

fn part1(elements: (usize, usize), lengths: &str) -> usize {
    let lengths = read_input(lengths);
    let mut values = generate_values(elements);
    let size = values.len();
    let mut current_position = 0;
    for (skip_size, length) in lengths.into_iter().enumerate() {
        let mut positions: Vec<usize> = Vec::new();
        for i in 0..length {
            let mut position = current_position + i;
            if position >= size {
                position = (current_position + i) - size
            }
            positions.push(position)
        }
        let mut sorted_sublist: Vec<usize> = Vec::new();
        for position in &positions {
            sorted_sublist.push(values[*position])
        }
        for position in positions {
            values[position] = sorted_sublist.pop().unwrap()
        }
        current_position += length + skip_size;
        if current_position >= size {
            current_position %= size;
        }
    }
    let result = values[0] * values[1];
    println!("Day 10 part 1, solution: {}", result);
    result
}

fn part2(elements: (usize, usize), lengths: &str) -> String {
    let lengths = read_input_as_ascii(lengths);
    let mut values = generate_values(elements);
    let size = values.len();
    let mut current_position = 0;
    let mut skip_size = 0usize;
    const ROUNDS: usize = 64;
    for _ in 0..ROUNDS {
        for length in &lengths {
            let mut positions: Vec<usize> = Vec::new();
            for i in 0..*length {
                let mut position = current_position + i;
                if position >= size {
                    position = (current_position + i) % size
                }
                positions.push(position)
            }
            let mut sorted_sublist: Vec<usize> = Vec::new();
            for position in &positions {
                sorted_sublist.push(values[*position])
            }
            for position in positions {
                values[position] = sorted_sublist.pop().unwrap()
            }
            current_position += length + skip_size;
            skip_size += 1;
            if current_position >= size {
                current_position %= size;
            }
        }
    }
    let hash = to_dense_hash(values);
    let hash_hex = to_hex(hash);
    println!("Day 10 part 2, solution: {}", hash_hex);
    hash_hex
}

fn to_dense_hash(numbers: Vec<usize>) -> Vec<usize> {
    let mut dense_hash: Vec<usize> = Vec::new();
    for chunk in numbers.chunks(16) {
        dense_hash.push(chunk.iter().fold(0, |acc, &x| acc ^ x))
    }
    dense_hash
}

fn to_hex(hash: Vec<usize>) -> String {
    let mut hash_hex: String = String::from("");
    for num in hash {
        hash_hex.push_str(format!("{:02x}", num).as_str().trim());
    }
    hash_hex
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn part1_works_with_test_input() {
        assert_eq!(part1((0, 5), "3,4,1,5"), 12);
    }

    #[test]
    fn part1_works_with_puzzle_input() {
        assert_eq!(part1((0, 256), INPUT), 13760);
    }

    #[test]
    fn part2_works_with_test_input() {
        assert_eq!(part2((0, 256), ""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(
            part2((0, 256), "AoC 2017"),
            "33efeb34ea91902bb2f59c9920caa6cd"
        );
        assert_eq!(part2((0, 256), "1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(part2((0, 256), "1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }

    #[test]
    fn part2_works_with_puzzle_input() {
        assert_eq!(part2((0, 256), INPUT), "2da93395f1a6bb3472203252e3b17fe5");
    }
}
