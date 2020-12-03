use std::collections::HashSet;

// Day 1: Report Repair
// A version of "Two Sum Problem"
// https://leetcode.com/problems/two-sum/solution/
//
// I did the naive solutions first which are O(n^2) and O(n^3).
// It was fast enough to get a quick answer. I then refactored
// using a HashSet which gets it down to O(n) and O(n^2) and it
// could be seen in the performance testing.

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> HashSet<u64> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}

/* Part One
 *
 * Before you leave, the Elves in accounting just need you to fix your
 * expense report (your puzzle input); apparently, something isn't quite adding up.
 *
 * Specifically, they need you to find the two entries that sum to
 * `2020` and then multiply those two numbers together.
 *
 * For example, suppose your expense report contained the following:
 *
 * 1721
 * 979
 * 366
 * 299
 * 675
 * 1456
 *
 * In this list, the two entries that sum to 2020 are 1721 and 299.
 * Multiplying them together produces 1721 * 299 = 514579, so the correct answer is 514579.
 *
 * Of course, your expense report is much larger. Find the two entries that sum to 2020; what do you get if you multiply them together?
*/

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "
1721
979
366
299
675
1456
";

        assert_eq!(part1_solution2(&input_generator(data)), 514579)
    }
    #[test]
    fn sample_02() {
        let data = "
1721
979
366
299
675
1456
";

        assert_eq!(part2_solution1(&input_generator(data)), 241861950)
    }
}
