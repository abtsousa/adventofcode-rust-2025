use std::collections::VecDeque;

advent_of_code::solution!(3);

fn parse_line(line: &str) -> VecDeque<u32> {
    println!("Parsing {line}");
    line.chars().map(|x| x.to_digit(10).expect("Invalid char! {x}")).collect()
}

fn get_joltage<const N: usize>(mut input: VecDeque<u32>) -> u64 {
    let mut joltage: [u64; N] = [0; N];
    let len = input.len();
    for i in (1..=len).rev() {
        let candidate = input.pop_front().expect("Tried to pop empty vector!") as u64;
        for j in N.saturating_sub(i)..N {
            if candidate > joltage[j] {
                println!("Found candidate: {candidate}");
                joltage[j] = candidate;
                println!("New joltage: {:?}", joltage);
                for k in j+1..N {
                    joltage[k] = 0;
                }
                break
            }
        }
    };
    let mut result = 0;
    for digit in joltage {
        result = result * 10 + digit
    }
    println!("Result: {result}");
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.lines() {
        sum += get_joltage::<2>(parse_line(line))
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.lines() {
        sum += get_joltage::<12>(parse_line(line))
    }
    Some(sum)
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
