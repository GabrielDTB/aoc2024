advent_of_code::solution!(1);
use advent_of_code::Counter;
use anyhow::Result;
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

    let mut answer = 0;
    for (l, r) in left.into_iter().zip(right) {
        answer += if l >= r { l - r } else { r - l };
    }
    Ok(answer)
}

pub fn part_two(input: &str) -> Result<u32> {
    let (left, right) = parse_input(input)?;
    let counts = Counter::from_iter(right);

    let mut answer = 0;
    for l in left {
        answer += l * counts.get(&l)
    }
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
