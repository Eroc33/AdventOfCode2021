use std::io::BufRead;

use day04::*;

fn solution(input: impl BufRead) -> Result<u32, advent_of_utils::Error> {
    let mut state = parse_input(input)?;
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

        if let Some(i) = find_winner_index(&state.boards) {
            println!("winners: {:?}", &state.boards[i]);
            return Ok(score_board(&state.boards[i]) * drawn as u32);
        }
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
        4512,
    );
}

#[cfg(test)]
#[test]
fn column_winner_logic() {
    advent_of_utils::check_example(
        solution,
        "1,2,3,4,5

     1 13 17 11  0
     2  2 23  4 24
     3  9 14 16  7
     4 10  3 18  5
     5 12 20 15 19",
        1140,
    );
}

#[cfg(test)]
#[test]
fn row_winner_logic() {
    advent_of_utils::check_example(
        solution,
        "15,62,2,39,49,25,65,28,84,59,75,24,20,76,60,55,17,7,93,69,32,23,44,81,8,67,41,56,43,89,95,97,61,77,64,37,29,10,79,26,51,48,5,86,71,58,78,90,57,82,45,70,11,14,13,50,68,94,99,22,47,12,1,74,18,46,4,6,88,54,83,96,63,66,35,27,36,72,42,98,0,52,40,91,33,21,34,85,3,38,31,92,9,87,19,73,30,16,53,80

     15 56 39 57 40
     67 59 26 30 90
     84  2 41 25  7
     96 23 79 99 85
     13 10 86 51 53",
     35711,
    );
}
