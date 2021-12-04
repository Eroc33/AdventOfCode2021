use std::{borrow::Borrow, convert::TryInto, fmt::Display, io::BufRead};

pub struct Board {
    pub numbers: [u8; 25],
    pub state: [bool; 25],
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

pub struct Input {
    pub draw_order: Vec<u8>,
    pub boards: Vec<Board>,
}

pub fn parse_input(input: impl BufRead) -> Result<Input, advent_of_utils::Error> {
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

pub fn score_board(board: &Board) -> u32 {
    let mut sum = 0u32;
    for (&number, &state) in board.numbers.iter().zip(board.state.iter()) {
        if !state {
            sum += number as u32;
        }
    }
    sum
}

pub fn find_winner_index(boards: &[Board]) -> Option<usize> {
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
