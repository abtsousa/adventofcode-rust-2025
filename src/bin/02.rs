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
    if !length.is_multiple_of(2) || length < 2 {
        return 0
    };
    let s = format!("1{}1", "0".repeat(length as usize/2-1));
    s.parse::<u64>().unwrap()
}

fn check_repeating(int: u64) -> bool {
    let len = int.ilog10()+1;
    for chunk_len in 1..=len/2 {
        if len.is_multiple_of(chunk_len) {                       //123123123 => int
            let chunk = int / 10u64.pow(len - chunk_len);  // / 10^6 = 123 => chunk
            let mut check = 0;
            for i in (0..=(len-chunk_len)).step_by(chunk_len as usize) {
                check += chunk * 10u64.pow(i);
            }
            if check == int {
                return true;
            }
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
