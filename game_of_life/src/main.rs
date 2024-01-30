// INPUT:
// Input: Table = [[1,1],[1,0]]
// Output: [[1,1],[1,1]]

use core::fmt;
use std::fmt::Display;

#[derive(PartialEq, Eq)]
enum Binary {
    Zero,
    One,
}

#[derive(Debug)]
enum Directions {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

const ONE: Binary = Binary::One;
const ZERO: Binary = Binary::Zero;

const FULL_BLOCK: &'static str = "██";
const EMPTY_BLOCK: &'static str = "▒▒";

const NEWLINE: &'static str = "\n";
const DIRECTIONS: &'static [Directions] = &[Directions::North, Directions::NorthEast, Directions::East, Directions::SouthEast, Directions::South, Directions::SouthWest, Directions::West, Directions::NorthWest];

// const FPS: u32 = 10; // Pottentially for latter if I want to expand

pub struct Table (Vec<Vec<Binary>>);

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
    display!(Table(vec![vec![ONE, ONE], vec![ONE, ZERO]]));
}

pub fn game_of_life(t: Table) -> Table {
    for d in DIRECTIONS {
        // maybe use coordinate system instead of words??
    }

    todo!();
}
