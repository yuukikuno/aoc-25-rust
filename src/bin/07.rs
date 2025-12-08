use std::collections::{BTreeSet, HashSet};
use std::path::Component::ParentDir;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines().map(|line| line.as_bytes()).step_by(2);
    let mut first = lines.next()?.iter();
    let width = first.len();
    let starting_pos = first.position(|&char| char == b'S').expect("missing start");
    let mut beams_pos: Vec<usize> = vec![starting_pos];
    let mut next_beams_pos = Vec::new();

    let mut bitmap = vec![false; width];
    let mut result = 0;

    for line in lines {
        next_beams_pos.clear();
        bitmap.fill(false);

        for &x in &beams_pos {
            let targets = if line[x] == b'^' {
                result += 1;
                [x - 1, x + 1]
            } else {
                [x, x]
            };
            for &t in &targets {
                if !bitmap[t] {
                    bitmap[t] = true;
                    next_beams_pos.push(t);
                }
            }
        }

        std::mem::swap(&mut beams_pos, &mut next_beams_pos);
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines().map(|line| line.as_bytes()).step_by(2);
    let first_line = lines.next()?;
    let width = first_line.len();
    let starting_pos = first_line
        .iter()
        .position(|&char| char == b'S')
        .expect("missing start");

    let mut map: Vec<u64> = vec![0; width];
    map[starting_pos] += 1;

    let mut result = 1;
    let mut next_map = vec![0; width];
    for line in lines {
        next_map.fill(0);

        for x in 0..width{
            let count = map[x];
            if count == 0{
                continue
            }
            if line[x] == b'^' {
                result += count;
                next_map[x + 1] += count;
                next_map[x - 1] += count;
            } else {
                next_map[x] += count
            }
        }

        std::mem::swap(&mut map, &mut next_map);
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
