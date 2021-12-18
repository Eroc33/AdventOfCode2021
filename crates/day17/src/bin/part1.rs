use day17::*;
use std::{
    io::BufRead,
};

fn solution(mut input_reader: impl BufRead) -> Result<i32, advent_of_utils::Error> {
    let mut input = String::new();
    input_reader.read_to_string(&mut input)?;
    let input = input.trim();
    let input = input.trim_start_matches("target area: ");
    let (x_part, y_part) = advent_of_utils::split_parse::<String,String>(input, ",")?;
    let x_part = x_part.trim().trim_start_matches("x=");
    let y_part = y_part.trim().trim_start_matches("y=");

    let (x_min,x_max) = advent_of_utils::split_parse::<i32,i32>(x_part, "..")?;
    let (y_min,y_max) = advent_of_utils::split_parse::<i32,i32>(y_part, "..")?;

    let max_y_vel = (0..=-y_min).rev().into_iter().map(|initial_y_vel|{
        let mut y_vel = initial_y_vel;
        let mut y = 0;
        let mut max_y = 0;
        loop{
            if y < y_min {
                break None;
            }
            if (y_min..=y_max).contains(&y){
                break Some(max_y);
            }
            y += y_vel;
            y_vel -= 1;

            max_y = max_y.max(y);
        }
    })
    .filter_map(|landing| landing)
    .max();

    Ok(max_y_vel.unwrap())
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day17_part1_example() {
    advent_of_utils::check_example(
        solution,
        "target area: x=20..30, y=-10..-5",
        45,
    );
}