use day08::*;
use std::io::BufRead;

fn solution(input: impl BufRead) -> Result<usize, advent_of_utils::Error> {
    let mut counts = 0;
    for line in input.lines(){
        let line = line?;
        let (in_segs,out_segs) = line
            .split_once("|")
            .ok_or("Missing output delimiter")?;

            let in_segs: Vec<_> = in_segs.split(" ").map(str::trim).collect();
            let out_segs: Vec<_> = out_segs.split(" ").map(str::trim).collect();

            counts += out_segs.iter().filter_map(|digit_segs| match digit_segs.len(){
                2 => Some(1),
                4 => Some(4),
                3 => Some(7),
                7 => Some(8),
                other => None,
            }).count();
    }
    
    Ok(counts)
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day08_part1_example() {
    advent_of_utils::check_example(solution, "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce", 26)
}
