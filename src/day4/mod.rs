const RADIX: u32 = 10;

#[derive(Debug)]
enum GridItem {
    Roll,
    Empty,
    OutOfBounds
}


#[derive(Debug)]
enum Direction {
    None,
    Up,
    UpRight,
    UpLeft,
    Down,
    DownRight,
    DownLeft,
    Left,
    Right
}

pub fn day_four_pt_one() { 
    let input = parse_input();

    let sum: usize = input
        .iter()
        .enumerate()
        .map(|(y, r)| {
            r.iter().enumerate().map(|(x, _)| {
                let is_roll = direction_check(Direction::None, &input, x, y);

                if is_roll == 1 {
                    let mut ct_adj_rolls = 0;

                    ct_adj_rolls += direction_check(Direction::Up, &input, x, y);
                    ct_adj_rolls += direction_check(Direction::UpRight, &input, x, y);
                    ct_adj_rolls += direction_check(Direction::UpLeft, &input, x, y);
                    ct_adj_rolls += direction_check(Direction::Down, &input, x, y);
                    ct_adj_rolls += direction_check(Direction::DownRight, &input, x, y);
                    ct_adj_rolls += direction_check(Direction::DownLeft, &input, x, y);
                    ct_adj_rolls += direction_check(Direction::Right, &input, x, y);
                    ct_adj_rolls += direction_check(Direction::Left, &input, x, y);

                    if ct_adj_rolls < 4 {
                        println!("found good roll");
                        1
                    } else {
                        println!("found bad roll");
                        0
                    }

                } else {
                    0
                }
            }).sum::<usize>()
        }).sum();

    println!("{}", sum);
}

pub fn day_four_pt_two() { 
    let mut input = parse_input();
    let mut coords_to_remove: Vec<(usize, usize)> = vec![];
    let mut ct = 0;

    loop {
        // remove any coords_to_remove then process again
        for (x, y) in coords_to_remove.iter() {
            remove_roll(&mut input, *x, *y);            
        }

        coords_to_remove.clear();

        for (y, r) in input.iter().enumerate() {
            for (x, _) in r.iter().enumerate() {
                let is_roll = direction_check(Direction::None, &input, x, y);

                if is_roll == 1 {
                    let mut ct_adj_rolls = 0;

                    ct_adj_rolls += direction_check(Direction::Up, &input, x, y);
                    ct_adj_rolls += direction_check(Direction::UpRight, &input, x, y);
                    ct_adj_rolls += direction_check(Direction::UpLeft, &input, x, y);
                    ct_adj_rolls += direction_check(Direction::Down, &input, x, y);
                    ct_adj_rolls += direction_check(Direction::DownRight, &input, x, y);
                    ct_adj_rolls += direction_check(Direction::DownLeft, &input, x, y);
                    ct_adj_rolls += direction_check(Direction::Right, &input, x, y);
                    ct_adj_rolls += direction_check(Direction::Left, &input, x, y);

                    if ct_adj_rolls < 4 {
                        ct += 1;
                        coords_to_remove.push((x, y));
                    }
                }
            }
        }

        if coords_to_remove.len() == 0 {
            break;
        }
    }

    println!("{}", ct);
}

fn remove_roll(
    grid: &mut Vec<Vec<GridItem>>,
    x: usize,
    y: usize,
) {
    grid[y][x] = GridItem::Empty;  
}

fn direction_check(
    direction: Direction,
    grid: &Vec<Vec<GridItem>>,
    x: usize,
    y: usize,
) -> usize {
    let pos = match direction {
        Direction::None => {
            match grid.get(y) {
                Some(row) => {
                    match row.get(x) {
                        Some(i) => i,
                        None => &GridItem::OutOfBounds
                    }
                },
                None => &GridItem::OutOfBounds
            }
        },
        Direction::Up => {
            let Some(idx_y) = y.checked_sub(1) else {
                return 0;
            };

            match grid.get(idx_y) {
                Some(row) => {
                    &row[x]
                },
                None => &GridItem::OutOfBounds
            }
        },
        Direction::UpRight => {
            let Some(idx_y) = y.checked_sub(1) else {
                return 0;
            };

            match grid.get(idx_y) {
                Some(row) => {
                    match row.get(x + 1) {
                        Some(i) => i,
                        None => &GridItem::OutOfBounds
                    }
                },
                None => &GridItem::OutOfBounds
            }
        },
        Direction::UpLeft => {
            let Some(idx_y) = y.checked_sub(1) else {
                return 0;
            };
            let Some(idx_x) = x.checked_sub(1) else {
                return 0;
            };

            match grid.get(idx_y) {
                Some(row) => {
                    match row.get(idx_x) {
                        Some(i) => i,
                        None => &GridItem::OutOfBounds
                    }
                },
                None => &GridItem::OutOfBounds
            }
        }
        Direction::Down => {
            match grid.get(y + 1) {
                Some(row) => {
                    &row[x]
                },
                None => &GridItem::OutOfBounds
            }
        }
        Direction::DownRight => {
            match grid.get(y + 1) {
                Some(row) => {
                    match row.get(x + 1) {
                        Some(i) => i,
                        None => &GridItem::OutOfBounds
                    }
                },
                None => &GridItem::OutOfBounds
            }
        },
        Direction::DownLeft => {
            let Some(idx_x) = x.checked_sub(1) else {
                return 0;
            };

            match grid.get(y + 1) {
                Some(row) => {
                    match row.get(idx_x) {
                        Some(i) => i,
                        None => &GridItem::OutOfBounds
                    }
                },
                None => &GridItem::OutOfBounds
            }
        },
        Direction::Left => {
            let Some(idx_x) = x.checked_sub(1) else {
                return 0;
            };

            match &grid[y].get(idx_x) {
                Some(i) => i,
                None => &GridItem::OutOfBounds
            }
        }, 
        Direction::Right => {
            match &grid[y].get(x + 1) {
                Some(i) => i,
                None => &GridItem::OutOfBounds
            }
        }
    };

    // println!("Checking: {:?} for ({}, {}) found {:?}", direction, x, y, pos);

    match pos {
        GridItem::Roll => 1,
        _ => 0
    }
}

fn parse_input() -> Vec<Vec<GridItem>> {
    std::fs::read_to_string("src/day4/input")
        .unwrap()
        .lines()
        .map(|l| {
            return l.chars().map(|c| {
                match c {
                    '.' => GridItem::Empty,
                    '@' => GridItem::Roll,
                    _ => panic!("Invalid character")
                }
            }).collect::<Vec<GridItem>>();
        }).collect::<Vec<Vec<GridItem>>>()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_coordinates() {
        let input = parse_input();

        let is_roll = direction_check(Direction::Up, &input, 2, 0);
        assert_eq!(is_roll, 0);

        let is_roll = direction_check(Direction::Up, &input, 2, 1);
        assert_eq!(is_roll, 1);

        let is_roll = direction_check(Direction::UpRight, &input, 1, 1);
        assert_eq!(is_roll, 1);

        let is_roll = direction_check(Direction::UpLeft, &input, 4, 1);
        assert_eq!(is_roll, 1);

        let is_roll = direction_check(Direction::Down, &input, 2, 2);
        assert_eq!(is_roll, 1);

        let is_roll = direction_check(Direction::DownRight, &input, 2, 2);
        assert_eq!(is_roll, 1);

        let is_roll = direction_check(Direction::DownLeft, &input, 3, 2);
        assert_eq!(is_roll, 1);

        let is_roll = direction_check(Direction::Right, &input, 0, 2);
        assert_eq!(is_roll, 1);

        let is_roll = direction_check(Direction::Left, &input, 1, 2);
        assert_eq!(is_roll, 1);
    }
}
