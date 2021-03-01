use std::{num::ParseIntError, str::FromStr, usize};

#[derive(Debug, PartialEq)]
struct Entry {
    x: usize,
    y: usize,
    c: char,
    password: String,
}

impl Entry {
    fn check(&self) -> bool {
        let s = self.password.chars().filter(|c| c == &self.c).count();
        s >= self.x && s <= self.y
    }

    fn check2(&self) -> bool {
        let x = self.password.chars().nth(self.x - 1).unwrap() == self.c;
        let y = self.password.chars().nth(self.y - 1).unwrap() == self.c;
        x ^ y
    }
}

impl FromStr for Entry {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ss = s.to_string();
        // s = x-y c: password
        let space = ss.chars().position(|c| c == ' ').unwrap();
        let colon = ss.chars().position(|c| c == ':').unwrap();
        assert_eq!(space + 2, colon);

        let mut range = s[0..space].split('-').map(|i| i.parse::<usize>().unwrap());

        Ok(Entry {
            x: range.next().unwrap(),
            y: range.next().unwrap(),
            c: ss.chars().nth(space + 1).unwrap(),
            password: ss[colon + 2..].to_string(),
        })
    }
}

fn main() {
    let entries = include_str!("../input.txt")
        .lines()
        .map(|i| i.parse::<Entry>().unwrap())
        .collect::<Vec<Entry>>();

    println!(
        "{:?}",
        entries
            .iter()
            .filter(|e| e.check())
            .count()
    );

    println!(
        "{:?}",
        entries
            .iter()
            .filter(|e| e.check2())
            .count()
    );
}
