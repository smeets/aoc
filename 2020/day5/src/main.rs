use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
struct Seat {
    row: usize,
    col: usize,
}

/// Create a binary partition from (lo, hi), keeping upper or lower range
fn bsp((lo, hi): (usize, usize), upper: bool) -> (usize, usize) {
    if upper {
        (hi - (hi - lo) / 2, hi)
    } else {
        (lo, lo + (hi - lo) / 2)
    }
}

impl FromStr for Seat {
    type Err = ParseIntError;

    /// Parse BFFFBBFRRR into a 2D array ([row][col])
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row = (0, 127);
        for c in s[0..7].chars() {
            row = bsp(row, c == 'B');
        }

        let mut col = (0, 7);
        for c in s[7..].chars() {
            col = bsp(col, c == 'R');
        }

        Ok(Seat {
            row: row.0,
            col: col.0,
        })
    }
}

fn main() {
    let mut seats = include_str!("../input.txt")
        .lines()
        .map(|l| l.parse::<Seat>().unwrap())
        .map(|s| s.row * 8 + s.col)
        .collect::<Vec<usize>>();

    println!("{}", seats.iter().max().unwrap_or(&0));

    seats.sort_unstable();

    println!(
        "{}",
        seats
            .windows(2)
            .find(|seat_no| seat_no[0] == seat_no[1] - 2)
            .unwrap()[0]
            + 1
    );
}
