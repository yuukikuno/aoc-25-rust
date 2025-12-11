use std::collections::{HashMap, HashSet};
use std::mem::forget;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let split = line.split_once(':')?;
        let origin = split.0;
        let destinations: Vec<&str> = split.1.trim().split(' ').collect();
        map.insert(origin, destinations);
    }

    let mut result = 0;
    let mut traversals: Vec<Vec<&str>> = vec![vec!["you"]];
    loop {
        result += traversals
            .iter()
            .filter(|traversal| traversal.last() == Some(&"out"))
            .count();
        let mut next_traversals: Vec<Vec<&str>> = vec![];
        for traversal in &traversals {
            let last_node = traversal.last().unwrap();
            if let Some(next_nodes) = map.get(last_node) {
                for next_node in next_nodes {
                    next_traversals.push([&traversal[..], &[next_node]].concat());
                }
            }
        }
        if next_traversals.is_empty() {
            break;
        }
        std::mem::swap(&mut traversals, &mut next_traversals);
    }

    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let split = line.split_once(':')?;
        let origin = split.0;
        let destinations: HashMap<&str, u64> = split
            .1
            .trim()
            .split(' ')
            .map(|destination| (destination, 1))
            .collect();
        map.insert(origin, destinations);
    }

    loop {
        let clone = map.clone();
        if let Some((to_reduce, exploded)) = clone
            .iter()
            .find(|(to_reduce, _exploded)| !["svr", "fft", "dac"].contains(to_reduce))
        {
            for (_, line_destinations) in map.iter_mut() {
                if let Some(count) = line_destinations.remove(to_reduce) {
                    for (destination, exploded_count) in exploded {
                        line_destinations
                            .entry(destination)
                            .and_modify(|prev_count| {
                                *prev_count += exploded_count * count;
                            })
                            .or_insert(exploded_count * count);
                    }
                }
            }
            map.remove(to_reduce);
        } else {
            break;
        }
    }

    let result =
        map.get("svr")?.get("fft")? * map.get("fft")?.get("dac")? * map.get("dac")?.get("out")?;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
