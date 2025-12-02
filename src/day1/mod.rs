use std::{convert::Infallible, str::FromStr};

const UPPER_LIMIT: i32 = 99;
const LOWER_LIMIT: i32 = 0;

#[derive(Debug)]
enum Direction {
    Left,
    Right
}

impl FromStr for Direction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match s {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Unhandled direction")
        };

        Ok(direction)
    }
}

impl Direction {
    fn apply_movement(&self, pos: i32, dist: i32) -> u32 {
        match &self {
            Direction::Left => {
                return (pos - dist).rem_euclid(100) as u32;
            },
            Direction::Right => {
                return (pos + dist).rem_euclid(100) as u32;
            }
        }
    }
}

pub fn day_one_pt_one() {
    let input = std::fs::read_to_string("src/day1/input")
        .unwrap()
        .lines()
        .map(|l| {
            let (direction, distance) = l.split_at(1);
            let distance: i32 = distance.parse().unwrap();
            let direction = direction.parse::<Direction>().unwrap();

            return (direction, distance)
        })
        .collect::<Vec<(Direction, i32)>>();

    let mut pos: u32 = 50;
    let mut ct_zero = 0;

    for (dir, dist) in input.iter() {
        pos = dir.apply_movement(pos as i32, *dist as i32);

        if pos == 0 {
            ct_zero += 1;
        }
    }

    println!("{}", ct_zero);
}

// TODO: Refactor to utilize rem_euclid
pub fn day_one_pt_two() {
    let input = std::fs::read_to_string("src/day1/input")
        .unwrap()
        .lines()
        .map(|l| {
            let (direction, distance) = l.split_at(1);
            let distance: i32 = distance.parse().unwrap();
            let direction = direction.parse::<Direction>().unwrap();

            return (direction, distance)
        })
        .collect::<Vec<(Direction, i32)>>();

    let mut pos: u32 = 50;
    let mut ct_zero = 0;

    for (dir, dist) in input.iter() {
        let mut revolutions = 0;

        match dir {
            Direction::Left => {
                let result = sub_wrapping(pos as i32, *dist);
                pos = result.pos;
                revolutions += result.revolutions;
            },
            Direction::Right => {
                let result = add_wrapping(pos as i32, *dist);
                pos = result.pos;
                revolutions += result.revolutions;
            }
        };

        if revolutions > 0 {
            ct_zero += revolutions;
        }

        if pos == 0 {
            ct_zero += 1;
        }
    }

    println!("{}", ct_zero);
}

#[derive(Debug)]
struct RotationResult {
    revolutions: i32,
    pos: u32
}

fn sub_wrapping(x: i32, y: i32) -> RotationResult {
    let mut revolutions = y / 100;
    let rmndr = y % 100;

    let diff = x - rmndr;
    let mut pos = 0;

    if diff < LOWER_LIMIT {
        pos = (diff + 100).try_into().unwrap();

        // if we passed lower limit and our new position is not zero and we didn't start at 0
        if pos != 0 && x != 0 {
            revolutions += 1;
        }
    } else {
        pos = diff.try_into().unwrap();
    }

    RotationResult { revolutions, pos }
}

fn add_wrapping(x: i32, y: i32) -> RotationResult {
    let mut revolutions = y / 100;
    let rmndr = y % 100;

    let sum = x + rmndr;
    let mut pos = 0;

    if sum > UPPER_LIMIT {
        pos = (sum - 100).try_into().unwrap();

        // if we pass upper limit and our new position is greater than 0
        if pos > 0 {
            revolutions += 1;
        }
    } else {
        pos = sum.try_into().unwrap();
    }

    RotationResult { revolutions, pos }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapping_sub_pos() {
        let result = sub_wrapping(30, 60);
        assert_eq!(result.pos, 70);

        let result = sub_wrapping(30, 260);
        assert_eq!(result.pos, 70);

        let result = sub_wrapping(50, 330);
        assert_eq!(result.pos, 20);

        let result = sub_wrapping(19, 20);
        assert_eq!(result.pos, 99);

        let result = sub_wrapping(0, 100);
        assert_eq!(result.pos, 0);
    }

    #[test]
    fn wrapping_add_pos() {
        let result = add_wrapping(30, 70);
        assert_eq!(result.pos, 0);

        let result = add_wrapping(30, 270);
        assert_eq!(result.pos, 0);

        let result = add_wrapping(99, 3);
        assert_eq!(result.pos, 2);
    }
}
