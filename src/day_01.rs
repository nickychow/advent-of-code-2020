use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> HashSet<u64> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}

// Simplest
// #[aoc(day1, part1)]
pub fn part1_solution1(input: &HashSet<u64>) -> u64 {
    for x in input {
        let y = 2020 - x;
        if input.contains(&y) {
            return x * y;
        }
    }

    0
}

// A better approach
#[aoc(day1, part1)]
pub fn part1_solution2(input: &HashSet<u64>) -> u64 {
    for x in input {
        if let Some(&y) = input.get(&(2020 - x)) {
            return x * y;
        }
    }

    0
}

// To create a new HashSet and to return it increases the duration significantly, but it is a safer approach.
// #[aoc(day1, part1)]
pub fn part1_solution3(input: &HashSet<u64>) -> HashSet<u64> {
    let mut r = HashSet::new();

    for x in input {
        if let Some(&y) = input.get(&(2020 - x)) {
            r.insert(x * y);
        }
    }

    r
}

// Simplest
#[aoc(day1, part2)]
fn part2_solution1(input: &HashSet<u64>) -> u64 {
    for x in input {
        for y in input {
            if x + y > 2020 {
                continue;
            }
            let z = 2020 - x - y;
            if input.contains(&z) {
                return x * y * z;
            }
        }
    }

    0
}
