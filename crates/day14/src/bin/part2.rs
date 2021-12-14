use day14::*;
use std::{
    io::BufRead, collections::{HashMap},
};

fn solution(input: impl BufRead) -> Result<usize, advent_of_utils::Error> {

    let mut lines = input.lines();
    let template = lines.next().ok_or("No template")??;
    let template: Vec<char> = template.trim().chars().collect();

    let mut insertions = HashMap::new();
    for line in lines {
        let line = line?;
        let line = line.trim();
        if line.is_empty(){
            continue;
        }
        let (pair,sub) = advent_of_utils::split_parse::<String,char>(line, " -> ")?;
        insertions.insert(match pair.chars().collect::<Vec<_>>().as_slice(){
            &[a,b] => [a,b],
            _other => panic!("Window size should be 2"),
        }, sub);
    }

    let mut pair_counts = template.windows(2)
    .fold(HashMap::new(),|mut acc, c|{
        let c = match c{
            &[a,b] => [a,b],
            _other => panic!("Window size should be 2"),
        };
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    dbg!(&pair_counts);
    dbg!(&insertions);

    for _step in 0..40{
        for (pair,count) in std::mem::take(&mut pair_counts){
            if let Some(replacement) = insertions.get(&pair){
                let new_left = [pair[0],*replacement];
                let new_right = [*replacement,pair[1]];
                *pair_counts.entry(new_left).or_insert(0) += count;
                *pair_counts.entry(new_right).or_insert(0) += count;
            }else{
                *pair_counts.entry(pair).or_insert(0) += count;
            }
        }
    }

    let mut element_counts = HashMap::new();
    for (pair,count) in pair_counts{
        *element_counts.entry(pair[0]).or_insert(0) += count;
        *element_counts.entry(pair[1]).or_insert(0) += count;
    }

    let element_counts: HashMap<_,_> = element_counts.into_iter().map(|(k,v)| (k, v/2)).collect();

    let most_common = element_counts.values().max().ok_or("No most common element")?;
    let least_common = element_counts.values().min().ok_or("No least common element")?;

    Ok((most_common - least_common) + 1)
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day14_part2_example() {
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
        2188189693529,
    )
}