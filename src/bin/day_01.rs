use env_logger;
use std::collections::HashSet;
use std::time::Instant;

use aoc2020::*;

fn main() {
    env_logger::init();

    let input = read_input("./data/day_01/input");
    log::info!("{:#?}", input);

    let now = Instant::now();
    println!(
        "Part 1 result: {:?}, Duration: {:?}",
        part_01(&input),
        now.elapsed()
    );

    let now = Instant::now();
    println!(
        "Part 2 result: {:?}, Duration: {:?}",
        part_02(&input),
        now.elapsed()
    );
}

fn part_01(input: &HashSet<u64>) -> u64 {
    for x in input {
        let y = 2020 - x;
        if input.contains(&y) {
            return x * y;
        }
    }

    0
}

fn part_02(input: &HashSet<u64>) -> u64 {
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
