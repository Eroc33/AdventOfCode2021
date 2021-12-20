use day20::*;
use std::{collections::HashMap, io::BufRead};

fn solution(input: impl BufRead) -> Result<usize, advent_of_utils::Error> {
    let mut lines = input.lines();
    let binary_line = lines.next().ok_or("Missing first input line")??;
    let lookup = binary_line
        .trim()
        .chars()
        .map(|c| c == '#')
        .collect::<Vec<_>>();
    assert_eq!(lookup.len(), 512);
    lines.next().ok_or("Missing empty line")??;
    let (grid, width, height) = advent_of_utils::parse_grid_inner(lines, |c| c == '#')?;

    let mut grid: HashMap<_, _> = grid
        .into_iter()
        .map(|([x, y], v)| ([x as isize, y as isize], v))
        .collect();

    let mut clear = false;

    let mut min_x = 0;
    let mut max_x = (width - 1) as isize;
    let mut min_y = 0;
    let mut max_y = (height - 1) as isize;
    dbg!(grid.values().into_iter().filter(|&&c| c).count());
    for _step in 0..50 {
        min_x -= 1;
        min_y -= 1;
        max_x += 1;
        max_y += 1;
        let x_range = min_x..=max_x;
        let y_range = min_y..=max_y;
        let mut new_grid =
            HashMap::with_capacity(y_range.clone().count() * x_range.clone().count());
        for y in y_range.clone() {
            for x in x_range.clone() {
                let key = get_binary_neighbourhood(&grid, [x, y], clear);
                new_grid.insert([x, y], lookup[key]);
            }
        }
        grid = new_grid;
        clear = lookup[(if clear { 0b111111111 } else { 0b000000000 })];
        dbg!(grid.values().into_iter().filter(|&&c| c).count());
    }

    Ok(grid.values().into_iter().filter(|&&c| c).count())
}

fn neighbourhood(pos: [isize; 2]) -> impl Iterator<Item = [isize; 2]> {
    (-1isize..=1).into_iter().flat_map(move |y| {
        (-1isize..=1)
            .into_iter()
            .map(move |x| [x + pos[0], y + pos[1]])
    })
}

fn get_binary_neighbourhood(
    grid: &HashMap<[isize; 2], bool>,
    pos: [isize; 2],
    clear: bool,
) -> usize {
    neighbourhood(pos)
        .map(|k| grid.get(&k).copied().unwrap_or(clear))
        .fold(0, |acc, bit| acc << 1 | bit as usize)
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day20_part1_example() {
    advent_of_utils::check_example(
        solution,
        "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
    
    #..#.
    #....
    ##..#
    ..#..
    ..###",
        35,
    )
}
