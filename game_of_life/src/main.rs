// Input: Table = [[1,1],[1,0]]
// Output: [[1,1],[1,1]]
//
// Any live cell with fewer than two live neighbors dies as if caused by under-population.
// Any live cell with two or three live neighbors lives on to the next generation.
// Any live cell with more than three live neighbors dies, as if by over-population.
// Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.

use core::fmt;
use std::{collections::HashSet, fmt::Display, vec};

#[derive(PartialEq, Eq, Debug)]
enum Binary {
    Zero,
    One,
}

impl Binary {
    fn flip(&mut self) {
        let new = match self {
            Binary::Zero => Binary::One,
            Binary::One => Binary::Zero,
        };

        *self = new;
    }
}

// y: up/down, x: left right.
#[derive(PartialEq, Eq, Debug, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq)]
struct RelativeCoordinate {
    x: i8,
    y: i8,
}

impl From<RelativeCoordinate> for Coordinate {
    fn from(r: RelativeCoordinate) -> Self {
        Coordinate {
            x: r.x as usize,
            y: r.y as usize,
        }
    }
}

impl From<Vec<Vec<u8>>> for Table {
    fn from(value: Vec<Vec<u8>>) -> Self {
        let mut ret = vec![];
        for y in value {
            let mut buf = vec![];
            for x in y {
                buf.push(match x {
                    0 => Binary::Zero,
                    1 => Binary::One,
                    _ => panic!("invalid table input"),
                })
            }

            ret.push(buf);
        }

        Table(ret)
    }
}

fn input_table_to_table(input: Vec<Vec<u8>>) -> Table {
    input.into()
}

#[derive(Debug)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

fn direction_to_coordinates(d: &Direction) -> RelativeCoordinate {
    match d {
        Direction::North => RelativeCoordinate { x: -1, y: 0 }, //(-1, 0),
        Direction::NorthEast => RelativeCoordinate { x: -1, y: 1 }, //(-1, 1),
        Direction::East => RelativeCoordinate { x: 0, y: 1 },   //(0, 1),
        Direction::SouthEast => RelativeCoordinate { x: 1, y: 1 }, //(1, 1),
        Direction::South => RelativeCoordinate { x: 1, y: 0 },  //(1, 0),
        Direction::SouthWest => RelativeCoordinate { x: 1, y: -1 }, //(1, -1),
        Direction::West => RelativeCoordinate { x: 0, y: -1 },  //(0, -1),
        Direction::NorthWest => RelativeCoordinate { x: -1, y: -1 }, //(-1, -1),
    }
}

const ONE: Binary = Binary::One;
const ZERO: Binary = Binary::Zero;

const FULL_BLOCK: &str = "██";
const EMPTY_BLOCK: &str = "▒▒";

const NEWLINE: &str = "\n";
const DIRECTIONS: &[Direction] = &[
    Direction::North,
    Direction::NorthEast,
    Direction::East,
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest,
];

// const FPS: u32 = 10; // Pottentially for latter if I want to expand

#[derive(PartialEq, Eq, Debug)]
pub struct Table(Vec<Vec<Binary>>);

fn convert_relative_to_abs(r: RelativeCoordinate, abs: &Coordinate) -> Coordinate {
    let new_x = abs.x.checked_add_signed(r.x as isize).unwrap_or(0);
    let new_y = abs.y.checked_add_signed(r.y as isize).unwrap_or(0);

    Coordinate { x: new_x, y: new_y }
}

impl Table {
    fn get_cell_status(&self, c: &Coordinate) -> Binary {
        if c.y < self.0.len() && c.x < self.0[c.y].len() {
            return match self.0[c.y][c.x] {
                Binary::One => Binary::One,
                Binary::Zero => Binary::Zero,
            };
        }

        Binary::Zero
    }

    fn count_neighboring_live_cells(&self, c: &Coordinate) -> u8 {
        let mut neighboring_live_cells: HashSet<Coordinate> = HashSet::new();

        for direction in DIRECTIONS {
            let cell_coordinate = convert_relative_to_abs(direction_to_coordinates(direction), &c);

            if self.get_cell_status(&cell_coordinate) == Binary::One
                && <RelativeCoordinate as Into<Coordinate>>::into(direction_to_coordinates(
                    direction,
                )) != (Coordinate { x: 0, y: 0 })
            {
                neighboring_live_cells.insert(cell_coordinate);
            }
        }

        // dbg!(&neighboring_live_cells);
        neighboring_live_cells.len() as u8
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for t in self.0.iter() {
            for c in t {
                if *c == ONE {
                    write!(f, "{FULL_BLOCK}")?
                } else if *c == ZERO {
                    write!(f, "{EMPTY_BLOCK}")?
                }
            }

            write!(f, "{NEWLINE}")?;
        }

        Ok(())
    }
}

macro_rules! display {
    ($value:expr) => {
        println!("{}", $value);
    };
}

fn main() {
    let input_table_2: Vec<Vec<u8>> =
        vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
    display!(input_table_to_table(input_table_2.clone()));
    println!("|\nv");
    display!(game_of_life(input_table_to_table(input_table_2.clone())));

    // [[0,1,0],[0,0,1],[1,1,1],[0,0,0]] INPUT
    // [[0,0,0],[1,0,1],[0,1,1],[0,1,0]] OUTPUT
}

pub fn game_of_life(mut t: Table) -> Table {
    let mut flip_these_cells: HashSet<Coordinate> = HashSet::new();

    // ------ Any live cell with two or three live neighbors lives on to the next generation.
    for (y_count, y) in t.0.iter().enumerate() {
        for (x_count, _) in y.iter().enumerate() {
            let c = Coordinate {
                x: x_count,
                y: y_count,
            };
            let cell_neighbor_count = t.count_neighboring_live_cells(&c);
            let status = t.get_cell_status(&c);

            if status == Binary::Zero {
                // Dead
                if cell_neighbor_count == 3 {
                    // Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
                    flip_these_cells.insert(c);
                }
            } else {
                // Alive

                if cell_neighbor_count < 2 {
                    // ------ Any live cell with fewer than two live neighbors dies as if caused by under-population.
                    flip_these_cells.insert(c);
                } else if cell_neighbor_count > 3 {
                    // ------ Any live cell with more than three live neighbors dies, as if by over-population.
                    flip_these_cells.insert(c);
                }
            }
        }
    }

    for cell in flip_these_cells {
        t.0[cell.y][cell.x].flip();
    }

    t
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_relive_to_abs_conversion_1() {
        assert_eq!(
            convert_relative_to_abs(
                RelativeCoordinate { x: 1, y: 1 },
                &Coordinate { x: 100, y: 100 }
            ),
            Coordinate { x: 101, y: 101 }
        );
    }

    #[test]
    fn test_relive_to_abs_conversion_2() {
        assert_eq!(
            convert_relative_to_abs(
                RelativeCoordinate { x: -1, y: 5 },
                &Coordinate { x: 0, y: 2 }
            ),
            Coordinate { x: 0, y: 7 }
        );
    }

    #[test]
    fn test_game_of_life() {
        assert_eq!(
            game_of_life(input_table_to_table(vec![
                vec![0, 1, 0],
                vec![0, 0, 1],
                vec![1, 1, 1],
                vec![0, 0, 0]
            ])),
            input_table_to_table(vec![
                vec![0, 0, 0],
                vec![1, 0, 1],
                vec![0, 1, 1],
                vec![0, 1, 0]
            ])
        )
    }
}
