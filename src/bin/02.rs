advent_of_code::solution!(2);

pub fn digits_repeat_twice(id: u64) -> bool {
    let digits = id.to_string();
    if !digits.len().is_multiple_of(2) {
        return false;
    }
    let (a, b) = digits.split_at(digits.len() / 2);
    a == b
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result: u64 = 0;
    for id_range in input.split(',') {
        let (start, end) = id_range.split_once('-').unwrap();
        let (start, end): (u64, u64) = (start.parse().unwrap(), end.parse().unwrap());

        result += (start..=end)
            .filter(|&id| digits_repeat_twice(id))
            .sum::<u64>();
    }
    Some(result)
}

pub fn digits_repeat(id: u64) -> bool {
    let digits = id.to_string();
    let len = digits.len();
    (1..=len / 2)
        .filter(|&sub_len| len.is_multiple_of(sub_len))
        .any(|sub_len| {
            let mut sequences = digits.as_bytes().chunks(sub_len);
            let first = sequences.next().unwrap();
            sequences.all(|sequence| sequence == first)
        })
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result: u64 = 0;
    for id_range in input.split(',') {
        let (start, end) = id_range.split_once('-').unwrap();
        let (start, end): (u64, u64) = (start.parse().unwrap(), end.parse().unwrap());

        result += (start..=end).filter(|&id| digits_repeat(id)).sum::<u64>();
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
