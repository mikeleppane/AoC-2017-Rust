use itertools::Itertools;

const INPUT: &str = include_str!("./input.txt");

fn read_input(input: &str) -> Vec<u32> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

pub fn run() {
    println!("=== Day 1 ===");
    part1(INPUT);
    part2(INPUT);
    println!("=============");
    println!();
}

pub fn part1(input: &str) -> u32 {
    let inputs = read_input(input);
    let mut sum = 0u32;
    for (a, b) in inputs.iter().circular_tuple_windows() {
        if a == b {
            sum += a;
        }
    }
    println!("Day 1 part 1, solution: {}", sum);
    sum
}

pub fn part2(input: &str) -> u32 {
    let inputs = read_input(input);
    let mut sum = 0u32;
    let halfway = inputs.len() / 2;
    let inputs = inputs.repeat(2);

    for (index, a) in inputs[0..(inputs.len() / 2)].iter().enumerate() {
        if inputs[(index + 1)..(index + 1 + halfway)].contains(a) {
            sum += a;
        }
    }
    println!("Day 1 part 2, solution: {}", sum);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works_with_test_input() {
        assert_eq!(part1("1122"), 3);
        assert_eq!(part1("1111"), 4);
        assert_eq!(part1("1234"), 0);
        assert_eq!(part1("91212129"), 9);
    }
    #[test]
    fn part1_works_with_puzzle_input() {
        assert_eq!(part1(INPUT), 1136);
    }

    #[test]
    fn part2_works_with_test_input() {
        assert_eq!(part2("1212"), 6);
        assert_eq!(part2("1221"), 3);
        assert_eq!(part2("123425"), 4);
        assert_eq!(part2("123123"), 12);
        assert_eq!(part2("12131415"), 4);
    }
    #[test]
    fn part2_works_with_puzzle_input() {
        assert_eq!(part2(INPUT), 10536);
    }
}
