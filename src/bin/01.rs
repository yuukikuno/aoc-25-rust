advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut count = 0;
    let mut pos = 50;
    for line in input.lines() {
        let mut chars = line.chars();
        let direction = chars.next()?;
        let distance: i32 = chars.as_str().parse().unwrap();
        let sign = match direction {
            'L' => -1,
            'R' => 1,
            _ => panic!(),
        };
        pos += distance * sign;
        pos = pos %100;
        if pos == 0 {
            count += 1;
        }
    }
    Some(count)
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
        assert_eq!(result.unwrap(), 3);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
