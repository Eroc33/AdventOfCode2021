use std::{
    collections::HashMap,
    env,
    fmt::{Debug, Display},
    fs,
    io::{BufRead, BufReader},
    str::FromStr,
};

macro_rules! bail {
    ($($fmt_args:tt)+) => {
        return Err(format!($($fmt_args)+).into());
    };
}

#[macro_export]
macro_rules! main {
    ($solution_fn:ident) => {
        fn main() -> Result<(), advent_of_utils::Error> {
            let solution = $solution_fn(advent_of_utils::input()?)?;
            println!("{}", solution);
            Ok(())
        }
    };
}

pub type Error = Box<dyn std::error::Error>;

pub fn input() -> Result<impl BufRead, Error> {
    let filename = match env::args().nth(1) {
        Some(v) => v,
        None => {
            bail!("You must pass a filename as first argument");
        }
    };
    match fs::File::open(filename) {
        Ok(v) => Ok(BufReader::new(v)),
        Err(e) => {
            bail!("Couldn't open input file due to io error: {:?}", e);
        }
    }
}

pub fn input_lines_as<T>() -> Result<Vec<T>, Error>
where
    T: FromStr,
    T::Err: Debug,
{
    let reader = input()?;
    lines_as(reader)
}

pub fn lines_as<T, R>(reader: R) -> Result<Vec<T>, Error>
where
    T: FromStr,
    T::Err: Debug,
    R: BufRead,
{
    reader
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let line = line.map_err(|e| format!("Failed to read line {:}: {:}", i, e))?;
            line.trim()
                .parse::<T>()
                .map_err(|e| Error::from(format!("Parsing failed on line {:}: {:?}", i, e)))
        })
        .collect()
}

pub fn split_parse<L, R>(s: &str, delim: &str) -> Result<(L, R), Error>
where
    L: FromStr,
    L::Err: Display,
    R: FromStr,
    R::Err: Display,
{
    let split = s
        .find(delim)
        .ok_or_else(|| format!("Expected {} in range", delim))?;

    let (left, right) = s.split_at(split);
    let right = &right[delim.len()..];

    let left = left
        .parse::<L>()
        .map_err(|e| format!("Left could not be parsed: {}", e))?;
    let right = right
        .parse::<R>()
        .map_err(|e| format!("Right could not be parsed: {}", e))?;
    Ok((left, right))
}

pub fn input_grid<T>(
    map_char: impl Fn(char) -> T,
) -> Result<(HashMap<[usize; 2], T>, usize, usize), Error> {
    let mut height = 0;
    let mut width = 0;
    let mut map = HashMap::new();
    for (y, line) in input()?.lines().enumerate() {
        let line = line.map_err(|e| format!("Error reading line {}: {}", y, e))?;
        for (x, c) in line.chars().enumerate() {
            map.insert([x, y], map_char(c));
        }
        width = width.max(line.len());
        height = height.max(y);
    }
    Ok((map, width, height + 1))
}

pub fn check_example<'a, F, T>(solution: F, input: &'a str, value: T)
where
    F: FnOnce(std::io::Cursor<&'a str>) -> Result<T, Error>,
    T: PartialEq + Debug,
{
    assert_eq!(solution(std::io::Cursor::new(input)).unwrap(), value)
}
