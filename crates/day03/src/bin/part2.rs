use day03::*;
use std::io::BufRead;

fn bit_counts(values: &[u32]) -> ([u32; 32], u32) {
    let mut bit_counts = [0u32; 32];
    let mut line_count = 0;
    for value in values {
        for (i, count) in bit_counts.iter_mut().enumerate() {
            if value & (0b1 << i) != 0 {
                *count += 1;
            }
        }
        line_count += 1;
    }
    (bit_counts, line_count)
}

fn solution(input: impl BufRead) -> Result<u64, advent_of_utils::Error> {
    let lines = advent_of_utils::lines_as::<String, _>(input)?;
    let mut numbers = vec![];
    let mut line_width = 0;
    for line in lines {
        let binary = u32::from_str_radix(&line[..], 2)?;
        line_width = line_width.max(line.len());
        numbers.push(binary);
    }
    dbg!("oxygen");
    //oxygen
    let mut candidates = numbers.clone();
    for i in (0..line_width).rev() {
        let (bit_counts, line_count) = bit_counts(&candidates);
        eprintln!("candidates= {:#?}", &candidates);
        let keep_ones = bit_counts[i] >= line_count - bit_counts[i];
        let target = if keep_ones { 1 } else { 0 };
        candidates = candidates
            .into_iter()
            .filter(|c| ((c >> i) & 0b1) == target)
            .collect();
        if candidates.len() == 1 {
            break;
        }
    }
    let oxygen = candidates[0];

    dbg!("carbon");
    let mut candidates = numbers.clone();
    for i in (0..line_width).rev() {
        let (bit_counts, line_count) = bit_counts(&candidates);
        eprintln!("candidates= {:#?}", &candidates);
        let keep_ones = bit_counts[i] < line_count - bit_counts[i];
        let target = if keep_ones { 1 } else { 0 };
        candidates = candidates
            .into_iter()
            .filter(|c| ((c >> i) & 0b1) == target)
            .collect();
        if candidates.len() == 1 {
            break;
        }
    }
    let carbon = candidates[0];
    dbg!(carbon, oxygen);

    Ok(oxygen as u64 * carbon as u64)
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
        230,
    )
}
