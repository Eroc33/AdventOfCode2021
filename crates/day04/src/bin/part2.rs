use std::io::BufRead;

use day04::*;

fn solution(input: impl BufRead) -> Result<u32, advent_of_utils::Error> {
    let mut state = parse_input(input)?;
    let mut last_winner = None;
    dbg!(&state.boards);
    for drawn in state.draw_order {
        for Board { numbers, state } in &mut state.boards {
            for (number, state) in numbers.iter().zip(state.iter_mut()) {
                if *number == drawn {
                    *state = true;
                }
            }
        }
        dbg!(&state.boards);

        while let Some(i) = find_winner_index(&state.boards) {
            last_winner = Some((state.boards.remove(i), drawn));
        }
        if state.boards.is_empty() {
            break;
        }
    }
    if let Some((last_winner, last_winner_drawn)) = last_winner {
        let score = score_board(&last_winner);
        println!("winner: {:?}", &last_winner);
        return Ok(score * last_winner_drawn as u32);
    }
    Err("No winner".into())
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day04_part1_example() {
    advent_of_utils::check_example(
        solution,
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
    
     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6
    
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7",
        1924,
    );
}
