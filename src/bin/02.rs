use std::collections::HashMap;

advent_of_code::solution!(2);

fn parse_line(line: &str) -> Vec<(u64, u64)> {
    let mut vec = Vec::new();
    for pair in line.trim().split(',') {
        let tuple = pair.split_once('-').unwrap();
        let u: u64 = tuple.0.parse().unwrap();
        let v: u64 = tuple.1.parse().unwrap();
        vec.push((u,v))
    }
    vec
}

fn get_divider(length: u32) -> u64 {
    if length % 2 != 0 || length < 2 {
        return 0
    };
    let s = format!("1{}1", "0".repeat(length as usize/2-1));
    s.parse::<u64>().unwrap()
}

fn check_repeating(int: u64) -> bool {
    let DUMB_PRIME_MAP = HashMap::from([
        (1, vec![]),
        (2, vec![11]),
        (3, vec![111]),
        (4, vec![101, 1111]),
        (5, vec![11111]),
        (6, vec![10101, 1001, 111111]),
        (7, vec![1111111]),
        (8, vec![1010101, 10001]),
        (9, vec![1001001, 111111111]),
        (10, vec![101010101, 100001, 1111111111])
    ]);
    let len = int.ilog10()+1;
    for i in DUMB_PRIME_MAP.get(&len).unwrap_or_else(|| panic!("Int too large to check {}", int)) {
        if int % *i == 0 {
            return true
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let vec = parse_line(input.lines().next().unwrap());
    let mut sum = 0;
    for (i,j) in vec {
        for k in i..j {
            let len = k.ilog10()+1;
            if len % 2 == 0 && len >=2 && k % get_divider(len) == 0 {
                sum += k;
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let vec = parse_line(input.lines().next().unwrap());
    let mut sum = 0;
    for (i,j) in vec {
        for k in i..j {
            if check_repeating(k) {
                sum += k;
            }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
