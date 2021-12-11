use day11::*;
use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    io::BufRead,
};

fn solution(input: impl BufRead) -> Result<u64, advent_of_utils::Error> {
    let (mut grid, width, height) =
        advent_of_utils::parse_grid(input, |c| c.to_digit(10).unwrap() as u8)?;

    let mut count = 0;

    for step in 0..100 {
        let mut already_flashed = HashSet::default();
        for energy in grid.values_mut() {
            *energy += 1;
        }

        while grid.values().any(|&v| v > 9) {
            for y in 0..height {
                for x in 0..width {
                    if *grid.get(&[x, y]).unwrap_or(&0) > 9 && !already_flashed.contains(&[x, y]) {
                        already_flashed.insert([x, y]);
                        count += flash([x, y], &mut grid, &mut already_flashed);
                    }
                }
            }
        }
        for pos in already_flashed {
            grid.insert(pos, 0);
        }
    }
    Ok(count)
}

fn neighbours(pos: [usize; 2]) -> impl Iterator<Item = [usize; 2]> {
    [
        [0isize, 1],
        [0, -1],
        [1, 0],
        [-1, 0],
        [1, -1],
        [1, 1],
        [-1, -1],
        [-1, 1],
    ]
    .iter()
    .filter_map(move |offset| {
        let offset_pos = [
            (pos[0] as isize + offset[0]).try_into().ok(),
            (pos[1] as isize + offset[1]).try_into().ok(),
        ];
        if let [Some(x), Some(y)] = offset_pos {
            Some([x, y])
        } else {
            None
        }
    })
}

fn flash(
    start: [usize; 2],
    grid: &mut HashMap<[usize; 2], u8>,
    already_flashed: &mut HashSet<[usize; 2]>,
) -> u64 {
    let mut flashed = vec![start];
    let mut count = 1;
    grid.insert(start, 0);

    while !flashed.is_empty() {
        for pos in std::mem::take(&mut flashed) {
            for neighbour in neighbours(pos) {
                if let Some(neighbour_val) = grid.get_mut(&neighbour) {
                    *neighbour_val += 1;
                    if already_flashed.contains(&neighbour) {
                        continue;
                    }
                    if *neighbour_val > 9 {
                        *neighbour_val = 0;
                        flashed.push(neighbour);
                        already_flashed.insert(neighbour);
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day11_part1_example() {
    advent_of_utils::check_example(
        solution,
        "5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526",
        1656,
    )
}
