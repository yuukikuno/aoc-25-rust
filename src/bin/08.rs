use std::collections::HashSet;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let boxes: Vec<(f32, f32, f32)> = input
        .lines()
        .map(|line| {
            let mut line = line.split(",");
            (
                line.next().unwrap().parse::<f32>().unwrap(),
                line.next().unwrap().parse::<f32>().unwrap(),
                line.next().unwrap().parse::<f32>().unwrap(),
            )
        })
        .collect();
    let mut distances: Vec<(usize, usize, f32)> = boxes
        .iter()
        .enumerate()
        .flat_map(|(a, (xa, ya, za))| {
            boxes
                .iter()
                .enumerate()
                .skip(a + 1)
                .map(move |(b, (xb, yb, zb))| {
                    (
                        a,
                        b,
                        ((xa - xb).powf(2f32) + (ya - yb).powf(2f32) + (za - zb).powf(2f32)).sqrt(),
                    )
                })
        })
        .collect();
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let connections: Vec<(usize, usize, f32)> = distances.iter().take(1000).copied().collect();
    let mut circuits: Vec<Vec<usize>> = vec![];

    for (a, b, _) in connections {
        let matches: Vec<usize> = circuits
            .iter()
            .enumerate()
            .filter(|(_, circuit)| circuit.contains(&a) || circuit.contains(&b))
            .map(|(i, _)| i)
            .collect();
        match matches.len() {
            0 => circuits.push(vec![a, b]),
            1 => {
                let i = matches[0];
                circuits[i].push(a);
                circuits[i].push(b);
            }
            _ => {
                let first_match = matches[0];
                circuits[first_match].push(a);
                circuits[first_match].push(b);
                for &i in matches[1..].iter().rev() {
                    let to_add = &circuits[i].clone();
                    circuits[first_match].extend(to_add);
                    circuits.remove(i);
                }
            }
        }
    }

    for circuit in circuits.iter_mut(){
        circuit.sort();
        circuit.dedup();
    }
    circuits.sort_by_key(|a| usize::MAX - a.len());

    let mut result = 1;
    for circuit in circuits.iter_mut().take(3) {
        result *= circuit.len()
    }
    Some(result as u64)
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
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
