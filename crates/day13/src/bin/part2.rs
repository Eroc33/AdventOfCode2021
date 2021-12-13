use advent_of_utils::{Error, bail};
use day13::*;
use std::{
    io::BufRead, collections::HashMap, fmt::Write,
};

fn solution(input: impl BufRead) -> Result<String, advent_of_utils::Error> {

    let mut grid = HashMap::new();
    let mut folds = vec![];
    let mut parsing_folds = false;
    for line in input.lines(){
        let line = line?;
        let line = line.trim();
        if line.is_empty(){
            parsing_folds = true;
            continue;
        }
        if parsing_folds {
            let fold = line.strip_prefix("fold along ").ok_or("fold line bad prefix")?;
            let (dir,pos) = advent_of_utils::split_parse::<String,u32>(fold, "=")?;
            let dir = match &dir[..]{
                "x" => [pos,0],
                "y" => [0,pos],
                other => bail!("Unexpected fold axis: {}", other)
            };
            folds.push(dir);
        }else{
            let (x,y) = advent_of_utils::split_parse::<u32,u32>(line, ",")?;
            grid.insert([x,y], true);
        }
    }

    for fold in folds.into_iter(){
        let fold_is_x = fold[0] != 0;
        let fold_is_y = fold[1] != 0;
        grid = grid.into_iter().map(|(mut pos,val)|{
            if fold_is_x && pos[0] > fold[0]{
                pos[0] = 2*fold[0]-pos[0];
            }
            if fold_is_y && pos[1] > fold[1]{
                pos[1] = 2*fold[1]-pos[1];
            }
            (pos,val)
        }).collect();
    }

    let min_x = grid.iter().map(|(pos,_)| pos[0]).min().ok_or("No min x")?;
    let max_x = grid.iter().map(|(pos,_)| pos[0]).max().ok_or("No max x")?;
    let min_y = grid.iter().map(|(pos,_)| pos[1]).min().ok_or("No min y")?;
    let max_y = grid.iter().map(|(pos,_)| pos[1]).max().ok_or("No max y")?;

    let mut result = String::new();
    for y in min_y..=max_y
    {
        for x in min_x..=max_x
        {
            write!(result,"{}", grid.get(&[x,y]).map_or(' ', |&val| if val {'#'} else {'.'}))?;
        }
        writeln!(result)?
    }

    Ok(result)
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day13_part2_example() {
    advent_of_utils::check_example(
        solution,
        "6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0
        
        fold along y=7
        fold along x=5",
        r#"#####
#...#
#...#
#...#
#####
"#.into(),
    )
}