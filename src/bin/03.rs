advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    //parse batteries: 1 line for each
    Some(
        input
            .lines()
            .map(|batteries| {
                let batteries: Vec<u32> = batteries
                    .chars()
                    .map(|char| char.to_digit(10).unwrap())
                    .collect();
                let (first_battery_pos, first_battery) = batteries
                    .clone()
                    .into_iter()
                    .enumerate()
                    .rev()
                    .skip(1)
                    .max_by_key(|&(_i, x)| x)
                    .unwrap();
                let second_battery = batteries[first_battery_pos + 1..].iter().max().unwrap();
                (first_battery * 10 + second_battery) as u64
            })
            .sum::<u64>(),
    )
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
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
