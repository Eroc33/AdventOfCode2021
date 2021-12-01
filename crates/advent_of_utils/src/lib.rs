use std::{
    collections::HashMap,
    env,
    fmt::{Debug, Display},
    fs,
    io::{BufRead, BufReader},
    str::FromStr,
};

pub fn input() -> Result<impl BufRead, ()> {
    let filename = match env::args().nth(1) {
        Some(v) => v,
        None => {
            eprintln!("You must pass a filename as first argument");
            return Err(());
        }
    };
    match fs::File::open(filename) {
        Ok(v) => Ok(BufReader::new(v)),
        Err(e) => {
            eprintln!("Couldn't open input file due to io error: {:?}", e);
            return Err(());
        }
    }
}

pub fn input_lines_as<T>() -> Result<Vec<T>, ()>
where
    T: FromStr,
    T::Err: Debug,
{
    let reader = input()?;

    reader
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let line = line.map_err(|e| {
                eprintln!("Failed to read line {:}: {:}", i, e);
                ()
            })?;
            line.parse::<T>().map_err(|e| {
                eprintln!("Parsing failed on line {:}: {:?}", i, e);
            })
        })
        .collect()
}

pub fn split_parse<L, R>(s: &str, delim: &str) -> Result<(L, R), String>
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
) -> Result<(HashMap<[usize; 2], T>, usize, usize), ()> {
    let mut height = 0;
    let mut width = 0;
    let mut map = HashMap::new();
    for (y, line) in input()?.lines().enumerate() {
        let line = line.map_err(|e| {
            eprintln!("Error reading line {}: {}", y, e);
        })?;
        for (x, c) in line.chars().enumerate() {
            map.insert([x, y], map_char(c));
        }
        width = width.max(line.len());
        height = height.max(y);
    }
    Ok((map, width, height + 1))
}
