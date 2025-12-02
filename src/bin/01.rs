advent_of_code::solution!(1);

fn parse_line(line: &str) -> Option<i32> {

    if let Some(dig) = line.strip_prefix('L') {
        return match dig.parse::<i32>() {
            Ok(v) => Some(-v),
            Err(_) => None,
        }
    } else if let Some(dig) = line.strip_prefix('R') {
        return dig.parse::<i32>().ok()
    }
    None
}


pub fn part_one(input: &str) -> Option<u64> {
    let closure = |(acc, count): (i32, i32), x: Option<i32>| {
        let res = (acc + x.unwrap_or(0)).rem_euclid(100);
        (res, count + (res==0) as i32)
    };
    let (_, count) = input.lines().map(parse_line).fold((50, 0), closure);
    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let closure = |(acc, count): (i32, i32), x: Option<i32>| {
        let turn = x.unwrap_or(0);
        let res = acc + turn;
        (res.rem_euclid(100), count +
         if turn >= 0 {(acc + turn).div_euclid(100)}
         else {(acc-1).div_euclid(100) - (res-1).div_euclid(100)}) // if it hits zero from the right we should count it so we check for res-1 = -1 instead
    };
    let (_, count) = input.lines().map(parse_line).fold((50, 0), closure);
    Some(count as u64)
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
