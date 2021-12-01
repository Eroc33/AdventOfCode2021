use std::io::BufRead;

fn solution(input: impl BufRead) -> Result<u32, advent_of_utils::Error> {
    let numbers = advent_of_utils::lines_as::<u32, _>(input)?;
    let mut increases = 0;
    for window in numbers.windows(4) {
        if window[3] > window[0] {
            increases += 1;
        }
    }
    Ok(increases)
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day1_part2_example() {
    advent_of_utils::check_example(
        solution,
        "199
        200
        208
        210
        200
        207
        240
        269
        260
        263",
        5,
    )
}
