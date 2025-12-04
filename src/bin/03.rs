use std::ops::Deref;

advent_of_code::solution!(3);

fn parse_digits(s: &str) -> Vec<u8>{
    s.chars()
        .map(|char| char.to_digit(10).unwrap() as u8)
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|batteries| {
                let batteries: Vec<u8> = parse_digits(batteries);

                get_joltage(batteries, 2)
            })
            .sum::<u64>(),
    )
}

fn get_joltage(batteries: Vec<u8>, count: usize) -> u64 {
    let (_, joltage) =
        (0..count)
            .rev()
            .fold((batteries, 0), |(batteries, joltage), n| {
                let (pos, battery) = find_largest(n, batteries.clone());
                (batteries[pos + 1..].to_owned(), joltage + battery as u64 * 10_u64.pow(n as u32))
            });
    joltage
}

fn find_largest(n: usize, batteries: Vec<u8>) -> (usize, u8) {
    let (battery_pos, battery) = batteries
        .into_iter()
        .enumerate()
        .rev()
        .skip(n)
        .max_by_key(|&(_i, x)| x)
        .unwrap();
    (battery_pos, battery)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|batteries| {
                let batteries: Vec<u8> = parse_digits(batteries);
                get_joltage(batteries, 12)
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
