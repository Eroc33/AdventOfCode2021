use day19::*;
use nalgebra::{
    self, AbstractRotation, Isometry3, Point3, Quaternion, Rotation3, Transform3, UnitQuaternion,
    Vector3,
};
use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

fn solution(input: impl BufRead) -> Result<usize, advent_of_utils::Error> {
    let mut scanner_reports = vec![];
    let mut active_report = None;
    for line in input.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        } else if line.starts_with("---") {
            if let Some(report) = active_report.take() {
                scanner_reports.push(report);
            }
        } else {
            let (x, y, z) = advent_of_utils::split_parse3::<i64, i64, i64>(line, ",")?;
            active_report
                .get_or_insert_with(Vec::new)
                .push(Point3::from([x as f64, y as f64, z as f64]));
        }
    }
    if let Some(report) = active_report.take() {
        scanner_reports.push(report);
    }

    let mut transformations: HashMap<_, Vec<_>> = HashMap::new();
    'outer: for (i, report) in scanner_reports.iter().enumerate() {
        'inner: for (j, comp_report) in scanner_reports.iter().enumerate() {
            if i == j {
                continue 'inner;
            }
            if let Some(tr) = align(report, comp_report) {
                transformations
                    .entry(j)
                    .or_default()
                    .push((i, tr.inverse()));
                transformations.entry(i).or_default().push((j, tr));
            }
        }
    }

    let mut beacons = HashSet::new();
    let mut visited = HashSet::new();
    let mut open = vec![(0, Isometry3::identity())];
    visited.insert(0);

    while !open.is_empty() {
        for (i, transformation) in std::mem::take(&mut open) {
            let report = &scanner_reports[i];
            let points = report
                .iter()
                .map(|point| transformation.transform_point(point));
            beacons.extend(points.map(round));
            for (next, next_transformation) in transformations.get(&i).unwrap_or(&vec![]) {
                if !visited.insert(*next) {
                    continue;
                }
                open.push((*next, transformation * next_transformation));
            }
        }
    }

    Ok(beacons.len())
}

fn round(pt: Point3<f64>) -> Point3<i64> {
    Point3::from([
        pt[0].round() as i64,
        pt[1].round() as i64,
        pt[2].round() as i64,
    ])
}

fn align(report: &[Point3<f64>], other_report: &[Point3<f64>]) -> Option<Isometry3<f64>> {
    for pos in report {
        let offset = nalgebra::Translation3::from(-pos);
        let normalized_report: HashSet<_> = report.iter().map(|pos| round(*pos)).collect();
        for other_pos in other_report {
            let other_offset = -other_pos;
            let translation = nalgebra::Translation3::from(other_offset);
            for rotation in rotations() {
                let tr = offset.inverse() * rotation * translation;
                let count = other_report
                    .iter()
                    .filter(|p| normalized_report.contains(&round(tr.transform_point(*p))))
                    .count();
                if count >= 12 {
                    return Some(tr);
                }
            }
        }
    }
    None
}

fn rotations() -> impl Iterator<Item = UnitQuaternion<f64>> {
    [-1, 1]
        .iter()
        .flat_map(|&i| {
            [-1, 0, 1].iter().flat_map(move |&x| {
                [-1, 0, 1]
                    .iter()
                    .map(move |&y| [i as f64, x as f64, y as f64])
            })
        })
        .filter(|[i, x, y]| x != y)
        .flat_map(|[i, x, y]| {
            [
                UnitQuaternion::face_towards(
                    &Vector3::from([i, 0.0, 0.0]),
                    &Vector3::from([0.0, x, y]),
                ),
                UnitQuaternion::face_towards(
                    &Vector3::from([0.0, i, 0.0]),
                    &Vector3::from([x, 0.0, y]),
                ),
                UnitQuaternion::face_towards(
                    &Vector3::from([0.0, 0.0, i]),
                    &Vector3::from([x, y, 0.0]),
                ),
            ]
        })
}

advent_of_utils::main!(solution);

#[cfg(test)]
#[test]
fn day19_part1_example() {
    advent_of_utils::check_example(
        solution,
        "--- scanner 0 ---
        404,-588,-901
        528,-643,409
        -838,591,734
        390,-675,-793
        -537,-823,-458
        -485,-357,347
        -345,-311,381
        -661,-816,-575
        -876,649,763
        -618,-824,-621
        553,345,-567
        474,580,667
        -447,-329,318
        -584,868,-557
        544,-627,-890
        564,392,-477
        455,729,728
        -892,524,684
        -689,845,-530
        423,-701,434
        7,-33,-71
        630,319,-379
        443,580,662
        -789,900,-551
        459,-707,401
        
        --- scanner 1 ---
        686,422,578
        605,423,415
        515,917,-361
        -336,658,858
        95,138,22
        -476,619,847
        -340,-569,-846
        567,-361,727
        -460,603,-452
        669,-402,600
        729,430,532
        -500,-761,534
        -322,571,750
        -466,-666,-811
        -429,-592,574
        -355,545,-477
        703,-491,-529
        -328,-685,520
        413,935,-424
        -391,539,-444
        586,-435,557
        -364,-763,-893
        807,-499,-711
        755,-354,-619
        553,889,-390
        
        --- scanner 2 ---
        649,640,665
        682,-795,504
        -784,533,-524
        -644,584,-595
        -588,-843,648
        -30,6,44
        -674,560,763
        500,723,-460
        609,671,-379
        -555,-800,653
        -675,-892,-343
        697,-426,-610
        578,704,681
        493,664,-388
        -671,-858,530
        -667,343,800
        571,-461,-707
        -138,-166,112
        -889,563,-600
        646,-828,498
        640,759,510
        -630,509,768
        -681,-892,-333
        673,-379,-804
        -742,-814,-386
        577,-820,562
        
        --- scanner 3 ---
        -589,542,597
        605,-692,669
        -500,565,-823
        -660,373,557
        -458,-679,-417
        -488,449,543
        -626,468,-788
        338,-750,-386
        528,-832,-391
        562,-778,733
        -938,-730,414
        543,643,-506
        -524,371,-870
        407,773,750
        -104,29,83
        378,-903,-323
        -778,-728,485
        426,699,580
        -438,-605,-362
        -469,-447,-387
        509,732,623
        647,635,-688
        -868,-804,481
        614,-800,639
        595,780,-596
        
        --- scanner 4 ---
        727,592,562
        -293,-554,779
        441,611,-461
        -714,465,-776
        -743,427,-804
        -660,-479,-426
        832,-632,460
        927,-485,-438
        408,393,-506
        466,436,-512
        110,16,151
        -258,-428,682
        -393,719,612
        -211,-452,876
        808,-476,-593
        -575,615,604
        -485,667,467
        -680,325,-822
        -627,-443,-432
        872,-547,-609
        833,512,582
        807,604,487
        839,-516,451
        891,-625,532
        -652,-548,-490
        30,-46,-14",
        79,
    )
}
