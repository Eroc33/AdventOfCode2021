use day07::*;
use std::io::BufRead;

fn solution(input: impl BufRead) -> Result<u64, advent_of_utils::Error> {
    let mut positions = input
        .lines()
        .next()
        .ok_or("Missing input line")??
        .split(",")
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;
    positions.sort();

    let min = *positions.iter().min().ok_or("No minimum pos")?;
    let max = *positions.iter().max().ok_or("No minimum pos")?;

    let mut best: Option<(u32, u32, u64)> = None;

    for pos in min..=max{
        let cost = evaluate_cost(&positions,pos);
        if best.is_none() || cost < best.unwrap().2 {
            best = Some((pos,positions[pos as usize], cost));
        }
    }

    Ok(best.ok_or("No best")?.2)
}

fn evaluate_cost(positions: &[u32], point: u32) -> u64{
    positions.iter().map(|&p| sum_to((p as i64-point as i64).abs() as u64)).sum()
}

fn sum_to(n: u64) -> u64{
    (n*(n+1))/2
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day07_part2_example() {
    advent_of_utils::check_example(solution, "16,1,2,0,4,2,7,1,2,14", 168)
}
