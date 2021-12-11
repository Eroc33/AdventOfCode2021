use day10::*;
use std::{io::BufRead};

fn solution(input: impl BufRead) -> Result<u64, advent_of_utils::Error> {
    let mut scores = vec![];
    for line in input.lines(){
        let line = line?;
        let chunk = line.trim();

        let mut stack = vec![];
        let mut chars = chunk.chars();
        let score = loop{
            let char = if let Some(char) = chars.next(){
                char
            }else{
                break None;
            };

            let closer = match char {
                '(' => Some(')'),
                '[' => Some(']'),
                '{' => Some('}'),
                '<' => Some('>'),
                ')' => None,
                ']' => None,
                '}' => None,
                '>' => None,
                other => panic!("Unexpected character: {}", other)
            };
            if let Some(closer) = closer{
                stack.push(closer);
            }else{
                let closer = stack.pop();
                if closer != Some(char) {
                    break Some(char_score(char));
                }
            }  
        };

        if let Some(score) = score{
            scores.push(score);
        }
    }
    Ok(scores.into_iter().sum::<u64>())
}

fn char_score(char: char) -> u64{
    match char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        other => panic!("Unexpected character: {}", other)
    }
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day10_part1_example() {
    advent_of_utils::check_example(
        solution,
        "[({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]",
            26397,
    )
}
