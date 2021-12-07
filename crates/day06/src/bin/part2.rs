use day06::*;
use std::{collections::HashMap, io::BufRead, slice::SliceIndex};

fn solution(input: impl BufRead) -> Result<usize, advent_of_utils::Error> {
    let mut fish = input
        .lines()
        .next()
        .ok_or("Missing input line")??
        .split(",")
        .map(|s| s.parse::<u16>())
        .collect::<Result<Vec<_>, _>>()?;
    let mut sum = 0;
    let mut memo = Default::default();
    for fish in &fish {
        sum += evaluate_fish(&mut memo, *fish, 256);
    }
    Ok(sum)
}

fn evaluate_fish(memo: &mut HashMap<(u16, u16), usize>, life: u16, days: u16) -> usize {
    if let Some(memoized) = memo.get(&(life, days)) {
        return *memoized;
    }
    let value = if days == 0 {
        1
    } else if life == 0 {
        evaluate_fish(memo, 8, days - 1) + evaluate_fish(memo, 6, days - 1)
    } else {
        let remaining = life.min(days);
        evaluate_fish(memo, life - remaining, days - remaining)
    };
    memo.insert((life, days), value);
    return value;
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day06_part1_example() {
    advent_of_utils::check_example(solution, "3,4,3,1,2", 26984457539)
}
