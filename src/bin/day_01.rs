use env_logger;
use std::collections::HashSet;
use std::time::Instant;

use aoc2020::*;

// Part 1 - leetcode two sum and three sum

fn main() {
    env_logger::init();

    let input = read_input("./data/day_01/input");
    log::info!("{:#?}", input);

    let now = Instant::now();
    println!(
        "Part 1 result: {:?}, Duration: {:?}",
        part_01_solution_02(&input),
        now.elapsed()
    );

    let now = Instant::now();
    println!(
        "Part 2 result: {:?}, Duration: {:?}",
        part_02_solution_01(&input),
        now.elapsed()
    );
}

// pub struct Solution {}

// impl Solution {
//     // Simplest
//     fn part_01_solution_01(input: &HashSet<u64>) -> u64 {
//         for x in input {
//             let y = 2020 - x;
//             if input.contains(&y) {
//                 return x * y;
//             }
//         }

//         0
//     }

//     // A better approach
//     fn part_01_solution_02(input: &HashSet<u64>) -> u64 {
//         for x in input {
//             if let Some(&y) = input.get(&(2020 - x)) {
//                 return x * y;
//             }
//         }

//         0
//     }

//     // To create a new HashSet and to return it increases the duration significantly, but it is a safer approach.
//     fn part_01_solution_03(input: &HashSet<u64>) -> HashSet<u64> {
//         let mut r = HashSet::new();

//         for x in input {
//             if let Some(&y) = input.get(&(2020 - x)) {
//                 r.insert(x * y);
//             }
//         }

//         r
//     }

//     fn part_02_solution_01(input: &HashSet<u64>) -> u64 {
//         for x in input {
//             for y in input {
//                 if x + y > 2020 {
//                     continue;
//                 }
//                 let z = 2020 - x - y;
//                 if input.contains(&z) {
//                     return x * y * z;
//                 }
//             }
//         }

//         0
//     }
// }

// Simplest
fn part_01_solution_01(input: &HashSet<u64>) -> u64 {
    for x in input {
        let y = 2020 - x;
        if input.contains(&y) {
            return x * y;
        }
    }

    0
}

// A better approach
fn part_01_solution_02(input: &HashSet<u64>) -> u64 {
    for x in input {
        if let Some(&y) = input.get(&(2020 - x)) {
            return x * y;
        }
    }

    0
}

// To create a new HashSet and to return it increases the duration significantly, but it is a safer approach.
fn part_01_solution_03(input: &HashSet<u64>) -> HashSet<u64> {
    let mut r = HashSet::new();

    for x in input {
        if let Some(&y) = input.get(&(2020 - x)) {
            r.insert(x * y);
        }
    }

    r
}

// Simplest
fn part_02_solution_01(input: &HashSet<u64>) -> u64 {
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
