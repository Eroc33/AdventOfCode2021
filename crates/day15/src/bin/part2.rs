use day15::*;
use std::{
    io::BufRead, collections::{HashMap, HashSet, BinaryHeap}, convert::TryInto, slice::SliceIndex, cmp::Ordering, ops::Add,
};

fn solution(input: impl BufRead) -> Result<u64, advent_of_utils::Error> {
    let (grid, width, height) = advent_of_utils::parse_grid(input, |c| c.to_digit(10).unwrap() as u64)?;

    let (grid, width, height) = expand_grid(grid, width, height);

    let dist = dijkstra(&grid, width, height, [width-1, height-1], |_, neighbour| grid.get(neighbour).copied().unwrap_or(u64::MAX)).ok_or("No path")?;

    Ok(dist)
}

fn expand_grid(grid: HashMap<[usize; 2], u64>, width: usize, height: usize) -> (HashMap<[usize; 2], u64>, usize, usize) {
    let grid = &grid;
    let grid = (0..5).into_iter().flat_map(|tile_y|{
        (0..5).into_iter().flat_map(move |tile_x|{
            grid.iter().map(move |(k,v)|{
                ([k[0] + tile_x*width, k[1] + tile_y*height], wrap_to_range((v + tile_x as u64 + tile_y as u64)) )
            })
        })
    }).collect();
    let width = width*5;
    let height = height*5;
    (grid, width, height)
}

fn wrap_to_range(mut val: u64) -> u64 {
    while val > 9 {
        val -= 9;
    }
    val
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
    #[derive(PartialEq, Eq, Ord)]
    struct Node{
        dist: u64,
        pos: [usize;2],
    }
    impl PartialOrd for Node{
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.dist.cmp(&other.dist).reverse())
        }
    }
    #[derive(Clone, Copy, Debug)]
    struct State{
        dist: u64,
        prev: Option<[usize;2]>,
    }
    let mut states: HashMap<_,_> = grid.iter().map(|(k,_v)|{
        (k, State{dist: u64::MAX, prev: None})
    }).collect();
    let mut open: BinaryHeap<_> = grid.iter().map(|(k,_v)|{
        Node{
            dist: if *k == [0,0]{ 0 } else { u64::MAX },
            pos: *k
        }
    }).collect();

    states.get_mut(&[0,0]).unwrap().dist = 0;

    while !open.is_empty(){
        let u = open.pop().unwrap();

        let u_state = *states.get(&u.pos).unwrap();

        if u.pos == target{
            return Some(u_state.dist);
        }

        for neighbour in neighbours(u.pos){
            if let Some(v) = states.get_mut(&neighbour){
                let alt = u_state.dist + length(&u.pos,&neighbour);
                if alt < v.dist{
                    v.dist = alt;
                    v.prev = Some(u.pos);
                    let mut new_heap: BinaryHeap<_> = open.into_iter()
                        .filter(|x| x.pos != neighbour)
                        .collect();
                    new_heap.push(Node{ pos:neighbour, dist: alt});
                    open = new_heap;
                }
            }
        }
    }
    None
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day15_part2_example() {
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
        315,
    );
}

#[cfg(test)]
#[test]
fn expand_grid_check() {
    use std::{fmt::Write, hash};
    let mut hashmap = HashMap::new();
    hashmap.insert([0,0], 8);
    let (grid, width, height) = expand_grid(hashmap, 1, 1);
    let mut res = String::new();
    for y in 0..height{
        for x in 0..width{
            write!(res, "{}", grid.get(&[x,y]).unwrap());
        }
        writeln!(res);
    }
    assert_eq!(
        res,
        "89123
91234
12345
23456
34567
"
);
}