use std::str::FromStr;

advent_of_code::solution!(1);

struct Dial {
    position: i32,
    point_password: u64,
    click_password: u64,
}

enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

struct Rotation {
    direction: Direction,
    distance: u64,
}

impl FromStr for Rotation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Rotation {
            direction: Direction::try_from(s.chars().next().unwrap())?,
            distance: s[1..].parse().unwrap(),
        })
    }
}

impl Dial {
    pub fn new(position: i32) -> Self {
        Dial {
            position,
            point_password: 0,
            click_password: 0,
        }
    }

    pub fn turn(&mut self, rotation: Rotation) {
        let delta = match rotation.direction {
            Direction::Left => -(rotation.distance as i32),
            Direction::Right => rotation.distance as i32,
        };
        self.position = (self.position + delta).rem_euclid(100);
        if self.position == 0 {
            self.point_password += 1;
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial = Dial::new(50);
    for line in input.lines() {
        let rotation: Rotation = line.parse().unwrap();
        dial.turn(rotation);
    }
    Some(dial.point_password)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut count = 0;
    let mut pos = 50;
    for line in input.lines() {
        let mut chars = line.chars();
        let direction = chars.next()?;
        let distance: i32 = chars.as_str().parse().unwrap();
        let sign = match direction {
            'L' => -1,
            'R' => 1,
            _ => panic!(),
        };
        pos += distance * sign;
        //todo count when pos = 0 but dont count when going 0 -> something else (probs -1?)
        count += pos.div_euclid(100).unsigned_abs();
        pos = pos.rem_euclid(100);
    }
    Some(count as u64)
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
