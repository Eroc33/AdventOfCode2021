use day08::*;
use std::{io::BufRead, collections::{HashMap, HashSet, BTreeSet}, iter::FromIterator};

fn solution(input: impl BufRead) -> Result<u64, advent_of_utils::Error> {
    let mut sum = 0u64;
    for line in input.lines(){
        let line = line?;
        let (in_segs,out_segs) = line
            .split_once("|")
            .ok_or("Missing output delimiter")?;

            let in_segs: Vec<_> = in_segs.split(" ").map(str::trim).filter(|s| !s.is_empty()).map(|s| BTreeSet::from_iter(s.chars())).collect();
            let out_segs: Vec<_> = out_segs.split(" ").map(str::trim).filter(|s| !s.is_empty()).map(|s| BTreeSet::from_iter(s.chars())).collect();

            let mut seg_map: HashMap<BTreeSet<char>,u32> = Default::default();
            let mut rev_seg_map: HashMap<u32,BTreeSet<char>> = Default::default();
            let mut solved: Vec<BTreeSet<char>> = Default::default();

            while solved.len() != 10 {
                let mut new_solved = vec![];
                for digit_segs in in_segs.iter()
                    .filter(|s| !solved.contains(*s)) {
                    let known = match digit_segs.len(){
                        2 => Some(1),
                        4 => Some(4),
                        3 => Some(7),
                        5 => {
                            // 2/3/5
                            match (rev_seg_map.get(&1), rev_seg_map.get(&4)) {
                                (Some(one), Some(four)) => {
                                    if digit_segs.intersection(one).count() == 2 {
                                        Some(3)
                                    } else if digit_segs.intersection(four).count() == 2 {
                                        Some(2)
                                    } else {
                                        Some(5)
                                    }
                                }
                                _ => None
                            }
                        }
                        6 => {
                            // 0/6/9
                            match (rev_seg_map.get(&1), rev_seg_map.get(&4)) {
                                (Some(one), Some(four)) => {
                                    if digit_segs.intersection(four).count() == 4 {
                                        Some(9)
                                    } else if digit_segs.intersection(one).count() == 2 {
                                        Some(0)
                                    } else {
                                        Some(6)
                                    }
                                }
                                _ => None
                            }
                        }
                        7 => Some(8),
                        other => None,
                    };

                    if let Some(known) = known{
                        if rev_seg_map.get(&known).is_some() {
                            panic!("broken: trying to reinsert {} as {:?}", known, digit_segs);
                        }
                        seg_map.insert(digit_segs.clone(), known);
                        rev_seg_map.insert(known, digit_segs.clone());
                        new_solved.push(digit_segs.clone())
                    }
                }
                solved.append(&mut new_solved);
            }

            let out_segs = out_segs.iter().map(|s| seg_map.get(s).ok_or(format!("Missing entry for {:?}", s))).collect::<Result<Vec<_>,_>>()?;
            let out_seg_digits: Vec<_> = out_segs.into_iter().map(|i| i.to_string()).collect();
            sum += out_seg_digits.concat().parse::<u64>()?;
    }
    
    Ok(sum)
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day08_part2_example1() {
    advent_of_utils::check_example(solution, "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf", 5353)
}

#[cfg(test)]
#[test]
fn day08_part2_example2() {
    advent_of_utils::check_example(solution, "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce", 61229)
}