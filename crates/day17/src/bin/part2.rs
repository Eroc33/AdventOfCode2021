use day17::*;
use std::{
    io::BufRead,
};

fn solution(mut input_reader: impl BufRead) -> Result<usize, advent_of_utils::Error> {
    let mut input = String::new();
    input_reader.read_to_string(&mut input)?;
    let input = input.trim();
    let input = input.trim_start_matches("target area: ");
    let (x_part, y_part) = advent_of_utils::split_parse::<String,String>(input, ",")?;
    let x_part = x_part.trim().trim_start_matches("x=");
    let y_part = y_part.trim().trim_start_matches("y=");

    let (x_min,x_max) = advent_of_utils::split_parse::<i32,i32>(x_part, "..")?;
    let (y_min,y_max) = advent_of_utils::split_parse::<i32,i32>(y_part, "..")?;

    let y_velocities = (y_min..=-y_min).into_iter().map(|initial_y_vel|{
        let mut y_vel = initial_y_vel;
        let mut y = 0;
        loop{
            if y < y_min {
                break None;
            }
            if (y_min..=y_max).contains(&y){
                break Some(initial_y_vel);
            }
            y += y_vel;
            y_vel -= 1;
        }
    })
    .filter_map(|landing| landing)
    .collect::<Vec<_>>();

    let x_velocities = (0..=x_max).into_iter().map(|initial_x_vel|{
        let mut x_vel = initial_x_vel;
        let mut x = 0;
        loop{
            if x > x_max {
                break None;
            }
            if (x_min..=x_max).contains(&x){
                break Some(initial_x_vel);
            }
            if x_vel == 0 {
                break None;
            }
            x += x_vel;
            x_vel -= x_vel.signum();
        }
    })
    .filter_map(|landing| landing)
    .collect::<Vec<_>>();

    let mut velocity_pairs = vec![];
    for &initial_x_vel in &x_velocities{
        for &initial_y_vel in &y_velocities{
            let mut y_vel = initial_y_vel;
            let mut y = 0;
            let mut x_vel = initial_x_vel;
            let mut x = 0;
            let landing = loop{
                if x > x_max {
                    break None;
                }
                if y < y_min {
                    break None;
                }
                if (y_min..=y_max).contains(&y) && (x_min..=x_max).contains(&x){
                    break Some([initial_x_vel, initial_y_vel]);
                }
                x += x_vel;
                x_vel -= x_vel.signum();
                y += y_vel;
                y_vel -= 1;
            };
            if let Some(landing) = landing{
                velocity_pairs.push(landing);
            }
        }
    }

    dbg!(&x_velocities,&y_velocities);

    Ok(velocity_pairs.len())
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day17_part2_example() {
    advent_of_utils::check_example(
        solution,
        "target area: x=20..30, y=-10..-5",
        112,
    );
}