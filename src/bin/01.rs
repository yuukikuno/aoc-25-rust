advent_of_code::solution!(1);

mod dial {
    use std::str::FromStr;

    pub struct Dial {
        position: i32,
        point_password: u64,
        click_password: u64,
    }

    impl Dial {
        pub fn new(position: i32) -> Self {
            Dial {
                position,
                point_password: 0,
                click_password: 0,
            }
        }

        pub fn point_password(&self) -> u64 {
            self.point_password
        }
        pub fn click_password(&self) -> u64 {
            self.click_password
        }

        pub fn turn(&mut self, rotation: Rotation) {
            let initial = self.position;
            let delta = match rotation.direction {
                Direction::Left => -(rotation.distance as i32),
                Direction::Right => rotation.distance as i32,
            };
            self.click_password += get_clicks(initial, initial + delta);
            self.position = (initial + delta).rem_euclid(100);
            if self.position == 0 {
                self.point_password += 1;
            }
        }
    }

    fn get_clicks(start: i32, end: i32) -> u64 {
        (match (start, end) {
            (0, _) => end.abs().div_euclid(100),
            (_, ..=0) => end.abs().div_euclid(100) + 1,
            (_, _) => end.div_euclid(100),
        }) as u64
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

    pub struct Rotation {
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
}

use dial::{Dial, Rotation};

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial = Dial::new(50);
    for line in input.lines() {
        let rotation: Rotation = line.parse().unwrap();
        dial.turn(rotation);
    }
    Some(dial.point_password())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial = Dial::new(50);
    for line in input.lines() {
        let rotation: Rotation = line.parse().unwrap();
        dial.turn(rotation);
    }
    Some(dial.click_password())
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
