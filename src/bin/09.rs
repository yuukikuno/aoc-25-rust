use std::collections::BinaryHeap;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    //each line parse (x,y)
    let red_tiles: Vec<(u64, u64)> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap())
        })
        .collect();

    //calc all areas
    let mut areas = BinaryHeap::new();
    for (n, a) in red_tiles.iter().enumerate() {
        for b in red_tiles.iter().skip(n + 1) {
            let area =
                ((a.0 as i64 - b.0 as i64).abs() + 1) * ((a.1 as i64 - b.1 as i64).abs() + 1);
            areas.push(area as u64);
        }
    }
    areas.pop()
}

pub fn part_two(input: &str) -> Option<u64> {
    //each line parse (x,y)
    let red_tiles: Vec<(u64, u64)> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap())
        })
        .collect();

    let mut green_tiles: Vec<(u64, u64)> = vec![];

    //paint vertical only
    for window in red_tiles.windows(2) {
        let [(xa, ya), (xb, yb)] = window else {
            panic!()
        };
        if xa == xb {
            let &y_min = ya.min(yb);
            let &y_max = ya.max(yb);
            for y in (y_min + 1)..y_max {
                green_tiles.push((*xa, y));
            }
        }
        if ya == yb {
            let &x_min = xa.min(xb);
            let &x_max = xa.max(xb);
            for x in (x_min + 1)..x_max {
                green_tiles.push((x, *ya));
            }
        }
    }
    //last
    let ((xa, ya), (xb, yb)) = (red_tiles.last()?, red_tiles.first()?);
    if xa == xb {
        let &y_min = ya.min(yb);
        let &y_max = ya.max(yb);
        for y in (y_min + 1)..y_max {
            green_tiles.push((*xa, y));
        }
    }
    if ya == yb {
        let &x_min = xa.min(xb);
        let &x_max = xa.max(xb);
        for x in (x_min + 1)..x_max {
            green_tiles.push((x, *ya));
        }
    }

    //calc all areas w red tile indexes
    let mut areas: Vec<(usize, usize, u64)> = vec![];
    for (a, &(xa, ya)) in red_tiles.iter().enumerate() {
        for (b, &(xb, yb)) in red_tiles.iter().enumerate().skip(a + 1) {
            let area = ((xa as i64 - xb as i64).abs() + 1) * ((ya as i64 - yb as i64).abs() + 1);
            areas.push((a, b, (area as u64)));
        }
    }

    areas.sort_by_key(|&(_, _, size)| u64::MAX - size);

    for (a, b, area) in areas {
        let x_min = red_tiles[a].0.min(red_tiles[b].0);
        let x_max = red_tiles[a].0.max(red_tiles[b].0);
        let y_min = red_tiles[a].1.min(red_tiles[b].1);
        let y_max = red_tiles[a].1.max(red_tiles[b].1);
        if red_tiles
            .iter()
            .any(|&(x, y)| x > x_min && x < x_max && y > y_min && y < y_max)
        {
            continue
        }
        if green_tiles
            .iter()
            .any(|&(x, y)| x > x_min && x < x_max && y > y_min && y < y_max)
        {
            continue
        }
        return Some(area);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
