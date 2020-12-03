// missing_docs, missing_debug_implementations
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate aoc_runner_derive;

use aoc_runner;
// use std::collections::HashSet;
// use std::fmt::Debug;
// use std::hash::Hash;
// use std::path::Path;
// use std::str::FromStr;

// pub fn read_input<P, T>(file_name: P) -> HashSet<T>
// where
//     P: AsRef<Path>,
//     T: FromStr + Hash + Eq + Debug,
//     <T as FromStr>::Err: Debug,
// {
//     std::fs::read_to_string(file_name)
//         .expect("file not found!")
//         .lines()
//         .map(|l| l.trim())
//         .filter(|l| !l.is_empty())
//         .map(|l| l.parse().unwrap())
//         .collect()
// }

pub mod day_01;

aoc_lib! { year = 2020 }
