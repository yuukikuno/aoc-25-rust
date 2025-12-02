use advent_of_code::dial::{Dial, Rotation};

advent_of_code::solution!(1);
pub fn part_one(input: &str) -> Option<u64> {
    let mut dial = Dial::new(50);
    for line in input.lines() {
        let rotation: Rotation = line.parse().unwrap();
        dial.turn(rotation);
    }
    Some(dial.point_password)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial = Dial::new(50);
    for line in input.lines() {
        let rotation: Rotation = line.parse().unwrap();
        dial.turn(rotation);
    }
    Some(dial.click_password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
