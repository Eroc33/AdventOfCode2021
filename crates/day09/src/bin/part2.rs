use day09::*;
use std::{io::BufRead, convert::TryInto, collections::{HashSet, HashMap}};

fn neighbours(pos: [usize;2]) -> impl Iterator<Item=[usize;2]>
{
    [[0isize,1],[0,-1],[1,0],[-1,0]].iter().filter_map(move |offset|{
        let offset_pos = [
            (pos[0] as isize+offset[0]).try_into().ok(),
            (pos[1] as isize+offset[1]).try_into().ok(),
        ];
        if let [Some(x),Some(y)] = offset_pos{
            Some([x,y])
        }else{
            None
        }
    })
}

fn solution(input: impl BufRead) -> Result<u32, advent_of_utils::Error> {
    let (grid, width, height) = advent_of_utils::parse_grid(input, |c| c.to_digit(10).unwrap())?;
    let mut minimums = vec![];
    'outer: for (&pos, &val) in &grid{
        for offset_pos in neighbours(pos){
            if let Some(&neighbour) = grid.get(&offset_pos){
                if neighbour <= val {
                    continue 'outer;
                }
            }
        }
        minimums.push(pos);
    }
    let mut visited = HashSet::new();
    let mut basins = vec![];
    for pos in minimums {
        if visited.contains(&pos) {
            continue;
        }
        let basin = flood_fill(pos, &grid);
        visited.extend(basin.iter().copied());
        basins.push(basin);
    }
    let mut basin_sizes: Vec<_> = basins.iter().map(|b| b.len()).collect();
    basin_sizes.sort();
    basin_sizes.reverse();
    let largest_three = basin_sizes.into_iter().take(3);
    Ok(largest_three.product::<usize>() as u32)
}

fn flood_fill(start: [usize;2], grid: &HashMap<[usize;2],u32>) -> HashSet<[usize;2]>
{
    let mut open = vec![start];
    let mut basin = HashSet::new();
    basin.insert(start);
    while !open.is_empty(){
        for pos in std::mem::take(&mut open){
            let val = *grid.get(&pos).unwrap();
            for neighbour in neighbours(pos){
                if basin.contains(&neighbour){
                    continue;
                }
                if let Some(&new_val) = grid.get(&neighbour){
                    if new_val > val && new_val != 9 {
                        open.push(neighbour);
                        basin.insert(neighbour);
                    }
                }
            }
        }
    }
    basin
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day09_part2_example() {
    advent_of_utils::check_example(
        solution,
        "2199943210
    3987894921
    9856789892
    8767896789
    9899965678",
        1134,
    )
}
