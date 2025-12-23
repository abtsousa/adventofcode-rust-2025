use std::ops::Add;

advent_of_code::solution!(5);

fn is_between(start: u64, end: u64, x: u64) -> bool {
    x >= start && x <= end
}

#[derive(Clone, Copy, Debug)]
struct Range {
    start: u64,
    end: u64 // inclusive
}

impl Range {
    fn len(self) -> u64 {
        self.end-self.start+1
    }
}

impl From<(u64, u64)> for Range {
    fn from(value: (u64, u64)) -> Self {
        Range {
            start: value.0,
            end: value.1
        }
    }
}

impl Add for Range {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        // self contained in rhs
        if rhs.start <= self.start && rhs.end >= self.end {
            return Some(rhs)
        }

        // rhs contained in self
        if self.start <= rhs.start && self.end >= rhs.end {
            return Some(self);
        }

        // rhs left of self
        if rhs.end >= self.start && rhs.end <= self.end {
            return Some(Range {start: rhs.start, end: self.end});
        }

        // rhs right of self
        if rhs.start >= self.start && rhs.start <= self.end {
            return Some(Range {start: self.start, end: rhs.end});
        }

        // Disjoint ranges
        None
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut fresh_list = Vec::new();
    let mut lines = input.lines();
    // First part - list of fresh ids
    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let (i,j) = line.split_once("-")?;
        fresh_list.push((i.parse::<u64>().ok()?,j.parse::<u64>().ok()?));
    }

    // Second part - inputs
    let mut result = 0;
    for line in lines {
        for fresh_range in &fresh_list {
            if is_between(fresh_range.0, fresh_range.1, line.parse::<u64>().ok()?) {
                result += 1;
                break;
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut fresh_list : Vec<Range> = Vec::new();
    let mut lines = input.lines();
    // First part - list of fresh ids
    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let (i,j) = line.split_once("-")?;
        let mut rhs: Range = (i.parse::<u64>().ok()?,j.parse::<u64>().ok()?).into();

        let mut i = 0;

        while i < fresh_list.len() {
            if let Some(res) = fresh_list[i] + rhs {
                rhs = res;
                fresh_list.swap_remove(i); // remove merged range
            } else {
                i += 1;
            }
        }

        fresh_list.push(rhs);
    }

    Some(fresh_list.iter().map(|x| x.len()).sum())
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
