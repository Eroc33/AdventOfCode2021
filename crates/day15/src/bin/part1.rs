use day15::*;
use std::{
    io::BufRead, collections::{HashMap, HashSet}, convert::TryInto, slice::SliceIndex, cmp::Ordering,
};

fn solution(input: impl BufRead) -> Result<u64, advent_of_utils::Error> {
    let (grid, width, height) = advent_of_utils::parse_grid(input, |c| c.to_digit(10).unwrap() as u64)?;

    let dist = dijkstra(&grid, width, height, [width-1, height-1], |_, neighbour| grid.get(neighbour).copied().unwrap_or(u64::MAX)).ok_or("No path")?;

    Ok(dist)
}

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

fn dijkstra(grid: &HashMap<[usize; 2], u64>, width: usize, height: usize, target: [usize;2], length: impl Fn(&[usize;2],&[usize;2]) -> u64) -> Option<u64> {
    #[derive(Clone, Copy, Debug)]
    struct State{
        dist: u64,
        prev: Option<[usize;2]>,
    }
    let mut states: HashMap<_,_> = grid.iter().map(|(k,_v)|{
        (k, State{dist: u64::MAX, prev: None})
    }).collect();
    let mut open: HashSet<_> = grid.iter().map(|(k,_v)|{
        *k
    }).collect();

    states.get_mut(&[0,0]).unwrap().dist = 0;

    while !open.is_empty(){
        let u = open.iter().min_by_key(|k| states.get(k).unwrap().dist).copied().unwrap();
        open.remove(&u);

        let u_state = *states.get(&u).unwrap();

        if u == target{
            return Some(u_state.dist);
        }

        for neighbour in neighbours(u){
            if let Some(v) = states.get_mut(&neighbour){
                let alt = u_state.dist + length(&u,&neighbour);
                if alt < v.dist{
                    v.dist = alt;
                    v.prev = Some(u);
                }
            }
        }
    }
    dbg!(&states);
    None
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day15_part1_example() {
    advent_of_utils::check_example(
        solution,
        "1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581",
        40,
    );
}

#[cfg(test)]
#[test]
fn day15_part1_reddit_example() {
    advent_of_utils::check_example(
        solution,
        "19999
        19111
        11191",
        8,
    );
}