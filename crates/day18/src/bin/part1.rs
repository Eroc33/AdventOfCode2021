use advent_of_utils::bail;
use day18::*;
use std::{fmt::Display, io::BufRead, iter::Sum, ops::Add, str::FromStr};

#[derive(Debug, PartialEq)]
enum SFNum {
    Pair(Box<SFNum>, Box<SFNum>),
    Regular(u64),
}

impl Display for SFNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SFNum::Pair(left, right) => {
                write!(f, "[")?;
                left.fmt(f)?;
                write!(f, ",")?;
                right.fmt(f)?;
                write!(f, "]")?;
                Ok(())
            }
            SFNum::Regular(reg) => reg.fmt(f),
        }
    }
}

fn parse_u64(s: &str) -> Result<(u64, &str), advent_of_utils::Error> {
    let len = s.chars().take_while(|s| s.is_digit(10)).count();
    Ok((s[..len].parse::<u64>()?, &s[len..]))
}

impl Add for SFNum {
    type Output = SFNum;

    fn add(self, rhs: Self) -> Self::Output {
        SFNum::new_pair(self, rhs).reduce()
    }
}

impl Sum for SFNum {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap()
    }
}

impl SFNum {
    fn new_pair(left: SFNum, right: SFNum) -> Self {
        SFNum::Pair(Box::new(left), Box::new(right))
    }

    fn parse(s: &str) -> Result<(Self, &str), advent_of_utils::Error> {
        match s.chars().next() {
            Some(c) if c.is_digit(10) => {
                let (num, rest) = parse_u64(&s[..])?;
                return Ok((SFNum::Regular(num), rest));
            }
            _ => {}
        }
        match s.chars().next() {
            Some('[') => {}
            other => bail!("Unexpected char: {:?}, expected \"[\"", other),
        }

        let (left, rest) = SFNum::parse(&s[1..])?;

        match rest.chars().next() {
            Some(',') => {}
            other => bail!("Unexpected char: {:?}, expected \",\"", other),
        }

        let (right, rest) = SFNum::parse(&rest[1..])?;

        match rest.chars().next() {
            Some(']') => {}
            other => bail!("Unexpected char: {:?} expected \"]\"", other),
        }

        Ok((SFNum::new_pair(left, right), &rest[1..]))
    }

    fn magnitude(self) -> u64 {
        match self {
            SFNum::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
            SFNum::Regular(num) => num,
        }
    }

    fn as_regular(&self) -> Option<u64> {
        match self {
            SFNum::Pair(_, _) => panic!("called as_regular on pair"),
            SFNum::Regular(reg) => Some(*reg),
        }
    }

    fn explode(self, nesting: u32) -> (SFNum, Option<u64>, Option<u64>, bool) {
        match self {
            SFNum::Regular(_) => (self, None, None, false),
            SFNum::Pair(left, right) => {
                if nesting >= 4 {
                    return (
                        SFNum::Regular(0),
                        left.as_regular(),
                        right.as_regular(),
                        true,
                    );
                }
                let (left_replacement, left_left, left_right, left_exploded) =
                    left.explode(nesting + 1);
                if left_exploded {
                    (
                        SFNum::new_pair(left_replacement, right.add_left(left_right)),
                        left_left,
                        None,
                        true,
                    )
                } else {
                    let (right_replacement, right_left, right_right, right_exploded) =
                        right.explode(nesting + 1);
                    (
                        SFNum::new_pair(left_replacement.add_right(right_left), right_replacement),
                        None,
                        right_right,
                        right_exploded,
                    )
                }
            }
        }
    }

    fn add_left(self, val: Option<u64>) -> Self {
        match val {
            Some(v) => match self {
                SFNum::Pair(left, right) => SFNum::new_pair(left.add_left(val), *right),
                SFNum::Regular(reg) => SFNum::Regular(reg + v),
            },
            None => self,
        }
    }

    fn add_right(self, val: Option<u64>) -> Self {
        match val {
            Some(v) => match self {
                SFNum::Pair(left, right) => SFNum::new_pair(*left, right.add_right(val)),
                SFNum::Regular(reg) => SFNum::Regular(reg + v),
            },
            None => self,
        }
    }

    fn split(self) -> (Self, bool) {
        match self {
            SFNum::Regular(reg) if reg >= 10 => (
                SFNum::new_pair(
                    SFNum::Regular(((reg as f64) / 2.0).floor() as u64),
                    SFNum::Regular(((reg as f64) / 2.0).ceil() as u64),
                ),
                true,
            ),
            SFNum::Pair(left, right) => {
                let (new_left, did_split) = left.split();
                if did_split {
                    (SFNum::new_pair(new_left, *right), true)
                } else {
                    let (new_right, did_split) = right.split();
                    (SFNum::new_pair(new_left, new_right), did_split)
                }
            }
            _ => (self, false),
        }
    }

    fn reduce(self) -> Self {
        let mut pair = self;
        loop {
            let (new_pair, _, _, exploded) = pair.explode(0);
            pair = new_pair;
            if exploded {
                continue;
            }
            let (new_pair, has_split) = pair.split();
            pair = new_pair;
            if has_split {
                continue;
            }
            break;
        }
        pair
    }
}

fn solution(input: impl BufRead) -> Result<u64, advent_of_utils::Error> {
    let mut nums = vec![];
    for line in input.lines() {
        let line = line?;
        let line = line.trim();
        let (num, rest) = SFNum::parse(line)?;
        assert_eq!(rest, "");
        nums.push(num);
    }
    let num: SFNum = nums.into_iter().sum();
    Ok(num.magnitude())
}

advent_of_utils::main!(solution);

#[cfg(test)]
mod tests {
    use super::{solution, SFNum};
    #[test]
    fn day18_part1_example() {
        advent_of_utils::check_example(
            solution,
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
            4140,
        )
    }
}
