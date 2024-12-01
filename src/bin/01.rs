advent_of_code::solution!(1);
use anyhow::{bail, Result};
use itertools::Itertools;

pub fn part_one(input: &str) -> Result<u32> {
    bail!("Solution not implemented")
}

pub fn part_two(input: &str) -> Result<u32> {
    bail!("Solution not implemented")
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
        assert_eq!(result, u32::MAX);
    }
}
