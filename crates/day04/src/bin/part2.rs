use std::{borrow::Borrow, convert::TryInto, fmt::Display, io::BufRead};

struct Board {
    numbers: [u8; 25],
    state: [bool; 25],
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (numbers, state) in self.numbers.chunks_exact(5).zip(self.state.chunks_exact(5)) {
            for number in numbers {
                f.write_fmt(format_args!("{:>3}", number))?;
            }

            f.write_str("[")?;

            for &state in state {
                if state {
                    f.write_str("#")?;
                } else {
                    f.write_str(" ")?;
                }
            }

            f.write_str("]")?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}

struct Input {
    draw_order: Vec<u8>,
    boards: Vec<Board>,
}

fn parse_input(input: impl BufRead) -> Result<Input, advent_of_utils::Error> {
    let mut parsed = Input {
        draw_order: vec![],
        boards: vec![],
    };
    let mut active_board: Option<Vec<u8>> = None;
    for (i, line) in input.lines().enumerate() {
        let line = line?.trim().to_owned();
        if i == 0 {
            parsed.draw_order = line
                .split(',')
                .map(|s| s.parse::<u8>())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("First line error: {:?}", e))?;
            continue;
        }
        if line.is_empty() {
            if let Some(active_board) = active_board.take() {
                parsed.boards.push(Board {
                    numbers: active_board
                        .try_into()
                        .map_err(|_| format!("board line is wrong length (on line {})", i))?,
                    state: [false; 25],
                });
            }
        }
        let mut board_line: Vec<u8> = line
            .split_whitespace()
            .map(|s| s.parse::<u8>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("board line error: {:?} (line {})", e, i))?;
        active_board
            .get_or_insert_with(Vec::new)
            .append(&mut board_line);
    }
    if let Some(active_board) = active_board.take() {
        parsed.boards.push(Board {
            numbers: active_board
                .try_into()
                .map_err(|_| "last board line is wrong length".to_string())?,
            state: [false; 25],
        });
    }
    Ok(parsed)
}

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

fn score_board(board: &Board) -> u32 {
    let mut sum = 0u32;
    for (&number, &state) in board.numbers.iter().zip(board.state.iter()) {
        if !state {
            sum += number as u32;
        }
    }
    sum
}

fn find_winner_index(boards: &[Board]) -> Option<usize> {
    for (i, Board { numbers: _, state }) in boards.iter().enumerate() {
        for row in 0..5 {
            if state[(row * 5)..(row * 5) + 5].iter().all(|&s| s) {
                return Some(i);
            }
        }
        for col in 0..5 {
            if state.iter().skip(col).step_by(5).all(|&s| s) {
                return Some(i);
            }
        }
    }
    None
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
