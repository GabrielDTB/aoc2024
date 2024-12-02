advent_of_code::solution!(2);
use anyhow::{bail, Context, Result};
use itertools::Itertools;

fn parse_input(input: &str) -> Result<Vec<Vec<u32>>> {
    let result = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|e| e.parse::<u32>().with_context(|| "Failed to parse '{e}'."))
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<Vec<_>>>>()?;
    Ok(result)
}

fn good_report(input: Vec<u32>) -> bool {
    if input.len() < 2 {
        return false;
    }
    let condition = if input[0] > input[1] {
        |l, r| l > r && l - r >= 1 && l - r <= 3
    } else {
        |l, r| r > l && r - l >= 1 && r - l <= 3
    };
    for window in input.windows(2) {
        if !condition(window[0], window[1]) {
            return false;
        }
    }
    return true;
}

pub fn part_one(input: &str) -> Result<u32> {
    let reports = parse_input(input)?;

    let mut answer = 0;
    for report in reports {
        if good_report(report) {
            answer += 1;
        }
    }
    Ok(answer)
}

pub fn part_two(input: &str) -> Result<u32> {
    let reports = parse_input(input)?;

    let mut answer = 0;
    'outer: for report in reports {
        if good_report(report.clone()) {
            answer += 1;
            continue;
        }
        for i in 0..report.len() {
            if good_report(
                report
                    .iter()
                    .take(i)
                    .chain(report.iter().skip(i + 1))
                    .map(|e| *e)
                    .collect::<Vec<_>>(),
            ) {
                answer += 1;
                continue 'outer;
            }
        }
    }
    Ok(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, 4);
    }
}
