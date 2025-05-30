use std::collections::HashMap;

use crate::Solvable;


pub struct Solution;

impl Solvable for Solution  {
    type InputT = (Vec<i32>, Vec<i32>);
    type SolutionT = (i32, i32); // tuple for p1 and p2 solutions.

    fn parse(input: &str) -> Self::InputT {
        let lines_count = input.lines().count();

        let mut left: Vec<i32> = Vec::new();
        let mut right: Vec<i32> = Vec::new();

        // prealloc
        left.reserve(lines_count);
        right.reserve(lines_count);

        for line in input.lines() {
            let mut numbers = line.split("   ");
            let l: i32= numbers.next().expect("Left number not found").parse().expect("Couldn't parse as i32.");
            let r: i32= numbers.next().expect("Right number not found").parse().expect("Couldn't parse as i32.");

            left.push(l);
            right.push(r);
        }
        (left, right)
    }

    fn solve(input: Self::InputT) -> Self::SolutionT {
        let (mut left, mut right) = input;
        left.sort();
        right.sort();
        let dist = left.iter()
                            .zip(right.iter())
                            .map(|(&l,&r)| (l - r).abs())
                            .sum();
        // make a hashmap from the right list
        let mut right_freq: HashMap<i32, i32> = HashMap::new();
        for r in right {
             right_freq.entry(r).and_modify(|freq| *freq += 1).or_insert(1);
        }

        let similarity_score = left.into_iter()
                                        .map(|l| *right_freq.get(&l).unwrap_or(&0) * l)
                                        .sum();

        (dist, similarity_score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = Solution::parse("1   4\n2   5\n3   6");
        let output = Solution::solve(input);
        assert_eq!(output, (9, 9));
    }
}