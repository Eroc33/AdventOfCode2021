use std::io::BufRead;

pub fn parse_input(
    input: impl BufRead,
) -> Result<Vec<((usize, usize), (usize, usize))>, advent_of_utils::Error> {
    input
        .lines()
        .map(|line| {
            let line = line?;
            let line = line.trim();
            let (from, to) = advent_of_utils::split_parse::<String, String>(line, " -> ")?;
            let from = advent_of_utils::split_parse::<usize, usize>(&from, ",")?;
            let to = advent_of_utils::split_parse::<usize, usize>(&to, ",")?;
            Ok::<_, advent_of_utils::Error>((from, to))
        })
        .collect::<Result<Vec<_>, _>>()
}
