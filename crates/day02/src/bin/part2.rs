use advent_of_utils::bail;
use std::io::BufRead;

fn solution(input: impl BufRead) -> Result<u32, advent_of_utils::Error> {
    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;
    let lines = advent_of_utils::lines_as::<String, _>(input)?;
    for line in lines {
        let mut parts = line.split(' ');
        let dir = parts.next().ok_or("Missing direction part")?;
        let count = parts.next().ok_or("Missing count part")?;
        let count = count.parse::<u32>()?;
        match dir {
            "forward" => {
                horiz += count;
                depth += count * aim;
            }
            "up" => aim -= count,
            "down" => aim += count,
            other => bail!("Unknown direction {}", other),
        }
    }
    Ok(horiz * depth)
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day02_part1_example() {
    advent_of_utils::check_example(
        solution,
        "forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2",
        900,
    )
}
