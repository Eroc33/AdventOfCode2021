use day09::*;
use std::{io::BufRead, convert::TryInto};

fn solution(input: impl BufRead) -> Result<u32, advent_of_utils::Error> {
    let (grid, width, height) = advent_of_utils::parse_grid(input, |c| c.to_digit(10).unwrap())?;
    let mut minimums = vec![];
    'outer: for (&pos, &val) in &grid{
        for offset in &[[0isize,1],[0,-1],[1,0],[-1,0]]{
            let offset_pos = [
                (pos[0] as isize+offset[0]).try_into().ok(),
                (pos[1] as isize+offset[1]).try_into().ok(),
            ];
            let offset_pos = if let [Some(x),Some(y)] = offset_pos{
                [x,y]
            }else{
                continue;
            };
            if let Some(&neighbour) = grid.get(&offset_pos){
                if neighbour <= val {
                    continue 'outer;
                }
            }
        }
        minimums.push(val);
    }
    let risk_sum = minimums.into_iter().map(|val| val+1).sum();
    Ok(risk_sum)
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day09_part1_example() {
    advent_of_utils::check_example(
        solution,
        "2199943210
    3987894921
    9856789892
    8767896789
    9899965678",
        15,
    )
}
