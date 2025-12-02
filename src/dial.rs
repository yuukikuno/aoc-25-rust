use std::convert::TryFrom;
use std::str::FromStr;
pub struct Dial {
    position: i32,
    pub point_password: u64,
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
