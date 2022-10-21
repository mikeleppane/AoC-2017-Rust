use std::str::FromStr;

const TEST_INPUT: &str = include_str!("./test_input.txt");
const INPUT: &str = include_str!("./input.txt");

fn read_input(input: &str) -> Vec<i16> {
    let mut offsets: Vec<i16> = Vec::new();
    for v in input.split('\n') {
        offsets.push(i16::from_str(v.trim()).unwrap());
    }
    offsets
}

pub fn run() {
    println!("=== Day 5 ===");
    part1(INPUT);
    part2(INPUT);
    println!("=============");
    println!();
}

fn is_out_of_bounds(index: i16, limit: i16) -> bool {
    (index as i16 >= limit) || (index < 0)
}

fn part1(input: &str) -> u32 {
    let mut offsets = read_input(input);
    let mut steps = 0u32;
    let mut current_index = 0usize;
    let out_of_bounds = offsets.len() as i16;
    loop {
        let next_inst = offsets[current_index] + current_index as i16;
        if is_out_of_bounds(next_inst, out_of_bounds) {
            steps += 1;
            break;
        }
        offsets[current_index] += 1;
        current_index = next_inst as usize;
        steps += 1;
    }
    println!("Day 5 part 1, solution: {}", steps);
    steps
}

fn part2(input: &str) -> u32 {
    let mut offsets = read_input(input);
    let mut steps = 0u32;
    let mut current_index = 0usize;
    let out_of_bounds = offsets.len() as i16;
    loop {
        if offsets[current_index] == 0 {
            offsets[current_index] += 1;
            steps += 1;
            continue;
        }
        let next_inst = offsets[current_index] + current_index as i16;
        if is_out_of_bounds(next_inst, out_of_bounds) {
            steps += 1;
            break;
        }
        if offsets[current_index] >= 3 {
            offsets[current_index] -= 1;
        } else {
            offsets[current_index] += 1;
        }
        current_index = next_inst as usize;
        steps += 1;
    }
    println!("Day 5 part 2, solution: {}", steps);
    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn part1_works_with_test_input() {
        assert_eq!(part1(TEST_INPUT), 5);
    }

    #[test]
    fn part1_works_with_puzzle_input() {
        assert_eq!(part1(INPUT), 378980);
    }

    #[test]
    fn part2_works_with_test_input() {
        assert_eq!(part2(TEST_INPUT), 10);
    }

    #[test]
    fn part2_works_with_puzzle_input() {
        assert_eq!(part2(INPUT), 26889114);
    }
}
