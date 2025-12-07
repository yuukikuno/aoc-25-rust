use std::collections::{BTreeSet, HashSet};
use std::path::Component::ParentDir;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines().map(|line| line.as_bytes()).step_by(2);;
    let starting_pos = lines.next()?.iter().position(|&char| char == b'S').expect("missing start");
    let mut beams_pos: Vec<usize> = [starting_pos].into();
    let mut result = 0;

    let mut next_beams_pos = Vec::new();
    for line in lines {
        next_beams_pos.clear();
        for &x in &beams_pos{
            if line[x] == b'^'{
                result += 1;
                next_beams_pos.push(x-1);
                next_beams_pos.push(x+1);
            } else {
                next_beams_pos.push(x);
            }
        }
        next_beams_pos.sort();
        next_beams_pos.dedup();
        std::mem::swap(&mut beams_pos, &mut next_beams_pos);
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
