use std::cmp::max;
use std::collections::VecDeque;

advent_of_code::solution!(3);

fn parse_line(line: &str) -> VecDeque<u32> {
    line.chars().map(|x| x.to_digit(10).expect("Invalid char! {x}")).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.lines() {
        let mut a = 0;
        let mut b = 0;
        let mut vec = parse_line(line);
        while !vec.is_empty() {
            let candidate = vec.pop_front().expect("Tried to pop empty vector!") as u64;
            if candidate > a && !vec.is_empty() {
                a = candidate;
                b = 0;
                continue
            } else if candidate > b {
                b = candidate;
            }
        }
        sum += 10*a+b;
        println!("line: {line} a: {a} b: {b} sum: {sum}")
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
