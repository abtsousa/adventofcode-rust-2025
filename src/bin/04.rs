advent_of_code::solution!(4);

fn get_convolutional_layer(input: &Vec<Vec<char>>) -> Vec<Vec<isize>> {
    let m = input.len();
    let n = input.first().map(|l| l.len()).unwrap_or(0);

    let mut grid = vec![vec![0; n]; m];

    for (i, line) in input.iter().enumerate() {
        for (j, &c) in line.iter().enumerate() {
            if c == '@' {
                for di in [-1, 0, 1] {
                    for dj in [-1, 0, 1] {
                        let _i = i as isize + di;
                        let _j = j as isize + dj;
                        if (di == 0 && dj == 0)
                            || _i < 0
                            || _i >= m as isize
                            || _j < 0
                            || _j >= n as isize
                            || grid[_i as usize][_j as usize] == -1
                        {
                            continue;
                        }
                        grid[_i as usize][_j as usize] += 1;
                    }
                }
            } else {
                grid[i][j] = -1;
            }
        }
    }

    grid
}


pub fn part_one(input: &str) -> Option<u64> {
    let input = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let layer = get_convolutional_layer(&input);
    let result = layer.iter().flatten().fold(0, |acc, x| acc + (*x != -1 && *x < 4) as u64);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;
    let mut input = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    loop {
        let mut count = 0;
        let layer = get_convolutional_layer(&input);
        for (m, row) in layer.iter().enumerate() {
            for (n, &v) in row.iter().enumerate() {
                if v != -1 && v < 4 {
                    count += 1;
                    input[m][n] = 'x';
                }
            }
        }
        if count == 0 {
            break;
        }
        result += count;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
