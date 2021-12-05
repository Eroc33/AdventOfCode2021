use day05::*;
use std::{convert::TryInto, io::BufRead};

fn solution(input: impl BufRead) -> Result<usize, advent_of_utils::Error> {
    let lines = input
        .lines()
        .map(|line| {
            let line = line?;
            let line = line.trim();
            let (from, to) = advent_of_utils::split_parse::<String, String>(line, " -> ")?;
            let from = advent_of_utils::split_parse::<usize, usize>(&from, ",")?;
            let to = advent_of_utils::split_parse::<usize, usize>(&to, ",")?;
            Ok::<_, advent_of_utils::Error>((from, to))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let candidates = lines.iter().collect::<Vec<_>>();

    let width = candidates
        .iter()
        .map(|(from, to)| from.0.max(to.0))
        .max()
        .ok_or("No max x")?
        + 1;
    let height = candidates
        .iter()
        .map(|(from, to)| from.1.max(to.1))
        .max()
        .ok_or("No max y")?
        + 1;

    let mut grid = vec![0; width * height];

    for &((x1, y1), (x2, y2)) in candidates {
        let x_step: isize = match x1.cmp(&x2) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };

        let y_step: isize = match y1.cmp(&y2) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };

        let (x1, x2, y1, y2, width, height) = (
            x1 as isize,
            x2 as isize,
            y1 as isize,
            y2 as isize,
            width as isize,
            height as isize,
        );

        let mut x = x1 as isize;
        let mut y = y1 as isize;

        let x_range = if x1 > x2 { x2..=x1 } else { x1..=x2 };
        let y_range = if y1 > y2 { y2..=y1 } else { y1..=y2 };

        while x_range.contains(&x) && y_range.contains(&y) {
            let index: usize = (x + (width * y))
                .try_into()
                .map_err(|_| format!("point out of range: {:?}", (x, y)))?;
            grid[index] += 1;
            x += x_step;
            y += y_step;
        }
    }

    let count = grid.iter().filter(|value| **value >= 2).count();

    Ok(count)
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day05_part2_example() {
    advent_of_utils::check_example(
        solution,
        "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2",
        12,
    )
}
