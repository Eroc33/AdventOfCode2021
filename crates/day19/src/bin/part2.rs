use day19::*;
use std::io::BufRead;

fn solution(input: impl BufRead) -> Result<u32, advent_of_utils::Error> {
    let _ = advent_of_utils::lines_as::<String, _>(input)?;
    Ok(todo!())
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day19_part2_example() {
    advent_of_utils::check_example(solution, todo!(), todo!())
}
