use day14::*;
use std::{
    io::BufRead, collections::HashMap,
};

fn solution(input: impl BufRead) -> Result<usize, advent_of_utils::Error> {

    let mut lines = input.lines();
    let template = lines.next().ok_or("No template")??;
    let mut template: Vec<char> = template.trim().chars().collect();

    let mut insertions = HashMap::new();
    for line in lines {
        let line = line?;
        let line = line.trim();
        if line.is_empty(){
            continue;
        }
        let (pair,sub) = advent_of_utils::split_parse::<String,char>(line, " -> ")?;
        insertions.insert(pair.chars().collect::<Vec<_>>(), sub);
    }

    for _step in 0..10{
        let insertions = template.windows(2).map(|pair| insertions.get(pair));
        template = template.iter()
        .zip(insertions.into_iter().chain(std::iter::repeat(None)))
        .flat_map(|(&a,b)|{
            if let Some(&b) = b{
                vec![a,b]
            }else{
                vec![a]
            }
        }).collect();
    }

    let counts = template.into_iter()
    .fold(HashMap::new(),|mut acc, c|{
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    let most_common = counts.values().max().ok_or("No most common element")?;
    let least_common = counts.values().min().ok_or("No least common element")?;

    Ok(most_common - least_common)
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day14_part1_example() {
    advent_of_utils::check_example(
        solution,
        "NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C",
        1588,
    )
}