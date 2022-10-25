use std::str::FromStr;

const INPUT: &str = include_str!("./input.txt");

fn read_input(input: &str) -> Vec<u32> {
    let mut offsets: Vec<u32> = Vec::new();
    for v in input.split(' ') {
        offsets.push(u32::from_str(v.trim()).unwrap());
    }
    offsets
}

pub fn run() {
    println!("=== Day 6 ===");
    part1(INPUT);
    part2(INPUT);
    println!("=============");
    println!();
}

struct Result {
    bank: Vec<u32>,
    redistributions: u32,
}

fn redistributions_cycles(mut banks: Vec<u32>) -> Result {
    let mut bank_cache: Vec<Vec<u32>> = Vec::new();
    bank_cache.push(banks.clone());
    let mut redistributions = 0u32;
    loop {
        let chosen_bank = banks
            .iter()
            .position(|x| x == banks.iter().max().unwrap())
            .unwrap();
        let blocks_in_chosen_bank = banks[chosen_bank];
        banks[chosen_bank] = 0;
        let mut next_bank = chosen_bank + 1;
        for _ in 0..blocks_in_chosen_bank {
            if next_bank == banks.len() {
                next_bank = 0;
            }
            banks[next_bank] += 1;
            next_bank += 1;
        }
        if bank_cache.contains(&banks) {
            redistributions += 1;
            break;
        }
        bank_cache.push(banks.clone());
        redistributions += 1;
    }
    Result {
        bank: banks,
        redistributions,
    }
}

fn part1(input: &str) -> u32 {
    let banks = read_input(input);
    let result = redistributions_cycles(banks);
    println!("Day 6 part 1, solution: {}", result.redistributions);
    result.redistributions
}

fn part2(input: &str) -> u32 {
    let banks = read_input(input);
    let result = redistributions_cycles(banks);
    let result = redistributions_cycles(result.bank);
    println!("Day 6 part 2, solution: {}", result.redistributions);
    result.redistributions
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn part1_works_with_test_input() {
        assert_eq!(part1("0 2 7 0"), 5);
    }

    #[test]
    fn part1_works_with_puzzle_input() {
        assert_eq!(part1(INPUT), 6681);
    }

    #[test]
    fn part2_works_with_test_input() {
        assert_eq!(part2("0 2 7 0"), 4);
    }

    #[test]
    fn part2_works_with_puzzle_input() {
        assert_eq!(part2(INPUT), 2392);
    }
}
