use env_logger;
use std::collections::HashSet;

use aoc2020::*;

fn main() {
    env_logger::init();

    let input = read_input("./data/day_01/input");
    log::info!("{:#?}", input);
    println!("Part 1: {:?}", part_01(&input))
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
