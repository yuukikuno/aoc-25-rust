use std::ops::{Index, IndexMut};

advent_of_code::solution!(4);

type Cell = u8;

#[rustfmt::skip]
const NEIGHBORS: [(isize, isize); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Index<(usize, usize)> for Grid {
    type Output = Cell;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.cells[x + y * self.width]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(
        &mut self,
        (x, y): (usize, usize),
    ) -> &mut <Self as Index<(usize, usize)>>::Output {
        &mut self.cells[x + y * self.width]
    }
}

impl Grid {
    pub fn new(contents: &str) -> Self {
        let lines: Vec<&str> = contents.lines().collect();
        Grid {
            width: lines.first().unwrap().len(),
            height: lines.len(),
            cells: lines.iter().flat_map(|line| line.bytes()).collect(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &Cell)> {
        (0..self.height).flat_map(move |y| (0..self.width).map(move |x| (x, y, &self[(x, y)])))
    }

    pub fn count_adjacent(&self, x: usize, y: usize) -> u8 {
        NEIGHBORS
            .iter()
            .filter_map(|&(x_delta, y_delta)| {
                let neighbor_x = x.wrapping_add_signed(x_delta);
                let neighbor_y = y.wrapping_add_signed(y_delta);
                if neighbor_x < self.width && neighbor_y < self.height {
                    Some(self[(neighbor_x, neighbor_y)])
                } else {
                    None
                }
            })
            .filter(|&neighbor| neighbor == b'@')
            .count() as u8
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::new(input);
    Some(
        grid.iter()
            .filter(|&(x, y, &cell)| cell == b'@' && grid.count_adjacent(x, y) < 4)
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::new(input);
    let mut count: u64 = 0;
    loop {
        let to_clear: Vec<(usize, usize)> = grid
            .iter()
            .filter(|&(x, y, &cell)| cell == b'@' && grid.count_adjacent(x, y) < 4)
            .map(|(x, y, _)| (x, y))
            .collect();

        if to_clear.is_empty() {
            break;
        }

        for (x, y) in to_clear {
            grid[(x, y)] = b'.';
            count += 1;
        }
    }
    Some(count)
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
        assert_eq!(result, Some(43));
    }
}
