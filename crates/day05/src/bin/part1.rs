use day05::*;
use std::io::BufRead;

fn solution(input: impl BufRead) -> Result<usize, advent_of_utils::Error> {
    let lines = parse_input(input)?;

    let candidates = lines
        .iter()
        .filter(|(from, to)| from.0 == to.0 || from.1 == to.1)
        .collect::<Vec<_>>();

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
        if x1 == x2 {
            let x = x1;
            let y_min = y1.min(y2);
            let y_max = y1.max(y2);
            for y in y_min..=y_max {
                grid[x + (width * y)] += 1;
            }
        } else if y1 == y2 {
            let y = y1;
            let x_min = x1.min(x2);
            let x_max = x1.max(x2);
            for x in x_min..=x_max {
                grid[x + (width * y)] += 1;
            }
        } else {
            return Err("Not a horizontal or vertical line".into());
        }
    }

    let count = grid.iter().filter(|value| **value >= 2).count();

    Ok(count)
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day05_part1_example() {
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
        5,
    )
}
