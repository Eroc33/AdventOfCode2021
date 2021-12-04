use day03::*;
use std::io::BufRead;

fn solution(input: impl BufRead) -> Result<u64, advent_of_utils::Error> {
    let lines = advent_of_utils::lines_as::<String, _>(input)?;
    let mut bit_counts = [0u32; 32];
    let mut line_count = 0;
    let mut line_width = 0;
    for line in lines {
        let binary = u32::from_str_radix(&line[..], 2)?;
        for (i, count) in bit_counts.iter_mut().enumerate() {
            if binary & (0b1 << i) != 0 {
                *count += 1;
            }
        }
        line_count += 1;
        line_width = line_width.max(line.len());
    }
    let mut gamma = 0u64;
    for (i, &count) in bit_counts.iter().enumerate() {
        if count > line_count - count {
            //more 1s
            gamma |= 0b1 << i;
        }
    }
    let epsilon = (!gamma) & (u64::MAX >> (64 - line_width));
    dbg!(epsilon, gamma);
    Ok(epsilon * gamma)
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day03_part1_example() {
    advent_of_utils::check_example(
        solution,
        "00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010",
        198,
    )
}
