
advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (id_ranges, ids) = input.split_once("\n\n").unwrap();
    let id_ranges = id_ranges.split('\n').map(|id_range| {
        let (start, end) = id_range.split_once('-').unwrap();
        start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap()
    });
    Some(
        ids.split('\n')
            .map(|id| id.parse::<u64>().unwrap())
            .filter(|id| id_ranges.clone().any(|id_range| id_range.contains(id)))
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (id_ranges, _ids) = input.split_once("\n\n").unwrap();
    let mut id_ranges: Vec<(u64, u64)> = id_ranges
        .split('\n')
        .map(|id_range| {
            let (start, end) = id_range.split_once('-').unwrap();
            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .collect();
    id_ranges.sort_by(|(a, _), (b, _)| a.cmp(b));
    let mut merged_ranges: Vec<(u64, u64)> = vec![];
    for id_range in id_ranges {
        if merged_ranges.is_empty() {
            merged_ranges.push(id_range);
            continue;
        }
        let last = merged_ranges.last_mut().unwrap();
        if last.1 + 1 >= id_range.0 {
            last.1 = last.1.max(id_range.1);
        } else {
            merged_ranges.push(id_range);
        }
    }
    Some(
        merged_ranges
            .into_iter()
            .map(|id_range| (id_range.0..=id_range.1).count())
            .sum::<usize>() as u64,
    )
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
        assert_eq!(result, Some(14));
    }
}
