advent_of_code::solution!(3);
fn find_largest(batteries: &[u8], n: usize) -> (usize, u8) {
    let (position, &value) = batteries
        .iter()
        .enumerate()
        .rev()
        .skip(n)
        .max_by_key(|&(_i, x)| x)
        .unwrap();
    (position, value)
}

fn parse_digits(s: &str) -> Vec<u8> {
    s.bytes().map(|b| b - b'0').collect()
}

fn compute_joltage(batteries: &[u8], count: usize) -> u64 {
    let mut remaining = batteries;
    (0..count).rev().fold(0, |joltage, n| {
        let (pos, battery) = find_largest(remaining, n);
        remaining = &remaining[pos + 1..];
        joltage + battery as u64 * 10_u64.pow(n as u32)
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|batteries| {
                let batteries: Vec<u8> = parse_digits(batteries);
                compute_joltage(&batteries, 2)
            })
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|batteries| {
                let batteries: Vec<u8> = parse_digits(batteries);
                compute_joltage(&batteries, 12)
            })
            .sum::<u64>(),
    )
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
