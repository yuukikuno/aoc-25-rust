use std::ops::RangeInclusive;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    let grid = &input.lines().collect::<Vec<&str>>();
    for (y, line) in grid.iter().enumerate() {
        for (x, &char) in line.as_bytes().iter().enumerate() {
            if char == b'.' {
                continue;
            }
            let adjacent_count = grid[neighbor_range(grid, y)]
                .iter()
                .flat_map(|neighbor_row| {
                    let bytes = neighbor_row.as_bytes();
                    bytes[neighbor_range(bytes, x)].iter()
                })
                .filter(|&&adjacent_cell| adjacent_cell == b'@')
                .count()
                - 1; //self
            if adjacent_count < 4 {
                result += 1
            }
        }
    }
    Some(result)
}

fn neighbor_range<T>(slice: &[T], index: usize) -> RangeInclusive<usize> {
    index.saturating_sub(1)..=(index + 1).min(slice.len() - 1)
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
