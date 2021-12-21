use day21::*;
use std::{collections::HashMap, io::BufRead, slice::SliceIndex};

fn solution(input: impl BufRead) -> Result<u64, advent_of_utils::Error> {
    let mut lines = input.lines();
    let player1_pos = parse_player_pos(&mut lines, 1)?;
    let player2_pos = parse_player_pos(&mut lines, 2)?;

    let positions = [player1_pos, player2_pos];

    let mut cache = HashMap::new();
    let results = turn([positions[0], positions[1]], [0; 2], &mut cache);

    Ok(IntoIterator::into_iter(results).max().ok_or("No maximum")?)
}

fn turn(
    positions: [u64; 2],
    scores: [u64; 2],
    cache: &mut HashMap<([u64; 2], [u64; 2]), [u64; 2]>,
) -> [u64; 2] {
    fn turn_inner(
        positions: [u64; 2],
        scores: [u64; 2],
        cache: &mut HashMap<([u64; 2], [u64; 2]), [u64; 2]>,
    ) -> [u64; 2] {
        if scores[1] >= 21 {
            return [0, 1];
        }

        let mut wins = [0; 2];
        for (roll, ways) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
            let positions_0 = wrap_1_10(positions[0] + roll);
            let [win_1, win_0] = turn(
                [positions[1], positions_0],
                [scores[1], scores[0] + positions_0],
                cache,
            );
            wins[0] += ways * win_0;
            wins[1] += ways * win_1;
        }
        wins
    }

    let key = (positions, scores);

    if let Some(cached) = cache.get(&key) {
        return *cached;
    }

    let wins = turn_inner(positions, scores, cache);

    cache.insert(key, wins);
    wins
}

fn wrap_1_10(num: u64) -> u64 {
    let num = num % 10;
    if num == 0 {
        10
    } else {
        num
    }
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
fn day21_part2_example() {
    advent_of_utils::check_example(
        solution,
        "Player 1 starting position: 4
        Player 2 starting position: 8",
        444356092776315,
    )
}
