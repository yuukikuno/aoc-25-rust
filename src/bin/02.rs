advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result: u64 = 0;
    for id_range in input.split(',') {
        let (start, end) = id_range.split_once('-').unwrap();
        let (start, end): (u64, u64) = (start.parse().unwrap(), end.parse().unwrap());
        for id in start..=end {
            let id_as_string = id.to_string();
            if id_as_string.len().is_multiple_of(2) {
                let middle = id_as_string.len() / 2;
                if &id_as_string[0..middle] == &id_as_string[middle..id_as_string.len()] {
                    result += id;
                }
            }
        }
    }

    Some(result)
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
