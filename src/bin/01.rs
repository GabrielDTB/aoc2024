advent_of_code::solution!(1);
use anyhow::{bail, Context, Result};
use itertools::Itertools;

fn parse_input(input: &str) -> Result<(Vec<u32>, Vec<u32>)> {
    let result = input
        .split_whitespace()
        .map(|item| item.parse::<u32>())
        .process_results(|iter| iter.tuples().unzip())?;
    Ok(result)
}

pub fn part_one(input: &str) -> Result<u32> {
    let (mut left, mut right) = parse_input(input)?;
    left.sort();
    right.sort();
    let answer = left
        .into_iter()
        .zip(right)
        .map(|(l, r)| if l >= r { l - r } else { r - l })
        .sum();
    Ok(answer)
}

pub fn part_two(input: &str) -> Result<u32> {
    let (left, right) = parse_input(input)?;
    let answer = left
        .into_iter()
        .map(|l| l * right.iter().filter(|&&r| l == r).count() as u32)
        .sum();
    Ok(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, 31);
    }
}
