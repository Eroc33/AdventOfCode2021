use day10::*;
use std::{io::BufRead};

fn solution(input: impl BufRead) -> Result<u64, advent_of_utils::Error> {
    let mut scores = vec![];
    for line in input.lines(){
        let line = line?;
        let chunk = line.trim();

        let mut stack = vec![];
        let mut chars = chunk.chars();
        let remaining_stack = loop{
            let char = if let Some(char) = chars.next(){
                char
            }else{
                break Some(stack);
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
                    break None;
                }
            }  
        };

        if let Some(remaining_stack) = remaining_stack{
            scores.push(remaining_stack.into_iter().rev().fold(0,|acc, char| acc*5+char_score(char)));
        }
    }
    scores.sort();
    let winner = scores[(scores.len()-1)/2];
    Ok(winner)
}

fn char_score(char: char) -> u64{
    match char {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
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
            288957,
    )
}
