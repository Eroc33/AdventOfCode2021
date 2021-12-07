use day06::*;
use std::io::BufRead;

fn solution(input: impl BufRead) -> Result<usize, advent_of_utils::Error> {
    let mut fish = input
        .lines()
        .next()
        .ok_or("Missing input line")??
        .split(",")
        .map(|s| s.parse::<u8>())
        .collect::<Result<Vec<_>, _>>()?;
    let mut new_fish = vec![];
    for day in 0..80 {
        for fish in &mut fish {
            if *fish == 0 {
                *fish = 6;
                new_fish.push(8);
            } else {
                *fish -= 1;
            }
        }
        fish.append(&mut new_fish);
    }
    Ok(fish.len())
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day06_part1_example() {
    advent_of_utils::check_example(solution, "3,4,3,1,2", 5934)
}
