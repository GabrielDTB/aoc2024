advent_of_code::solution!(3);
use anyhow::{anyhow, bail, Context, Result};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while_m_n},
    character::{complete::char, is_digit},
    combinator::map,
    multi::many0,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

pub fn part_one(input: &str) -> Result<u32> {
    let thing: IResult<&str, Vec<Option<(&str, &str)>>> = many0(preceded(
        take_until("mul"),
        alt((
            map(
                delimited(
                    tag("mul("),
                    separated_pair(
                        take_while_m_n(1, 3, |c: char| is_digit(c as u8)),
                        char(','),
                        take_while_m_n(1, 3, |c: char| is_digit(c as u8)),
                    ),
                    char(')'),
                ),
                Some,
            ),
            map(preceded(tag("mul"), take_while(|c| c != 'm')), |_| None),
        )),
    ))(input);
    let answer = thing
        .unwrap()
        .1
        .into_iter()
        .flatten()
        .map(|(a, b)| a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap())
        .sum();
    Ok(answer)
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
        assert_eq!(result, 161);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, u32::MAX);
    }
}
