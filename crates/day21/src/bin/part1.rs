use day21::*;
use std::io::BufRead;

fn solution(input: impl BufRead) -> Result<u64, advent_of_utils::Error> {
    let mut lines = input.lines();
    let player1_pos = parse_player_pos(&mut lines, 1)?;
    let player2_pos = parse_player_pos(&mut lines, 2)?;

    let mut dice = 1;
    let mut positions = [player1_pos, player2_pos];
    let mut scores = [0, 0];

    while !scores.iter().any(|&score| score >= 1000) {
        for (position, score) in positions.iter_mut().zip(scores.iter_mut()) {
            let rolls = 3 * dice + 3;
            dice += 3;
            *position = (((*position + rolls) - 1) % 10) + 1;
            *score += *position;
            if *score >= 1000 {
                break;
            }
        }
    }

    let loser_score = scores.iter().min().ok_or("No minimum score")?;
    let num_rolls = dice - 1;

    Ok(loser_score * num_rolls)
}

fn parse_player_pos(
    lines: &mut std::io::Lines<impl BufRead>,
    player_num: u32,
) -> Result<u64, advent_of_utils::Error> {
    let line = lines
        .next()
        .ok_or_else(|| format!("Missing player {} input line", player_num))?;
    let line = line?;
    let line = line.trim();
    let prefix = format!("Player {} starting position: ", player_num);
    let player1_pos: u64 = line
        .strip_prefix(&prefix)
        .ok_or("Missing input line prefix")?
        .parse()?;
    Ok(player1_pos)
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day21_part1_example() {
    advent_of_utils::check_example(
        solution,
        "Player 1 starting position: 4
        Player 2 starting position: 8",
        739785,
    )
}
