use itertools::Itertools;

use std::{
    collections::HashSet,
    fmt::Debug,
    io::{BufRead, BufReader},
    ops::{Add, Neg, Sub},
    str::FromStr,
};

#[derive(Debug, Default, PartialEq)]
struct Vector(i64, i64, i64);

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
struct Point(i64, i64, i64);

impl Point {
    fn rotate_x(&self) -> Self {
        Point(self.0, self.2, self.1.neg())
    }
    fn rotate_x2(&self) -> Self {
        Point(self.0, self.1.neg(), self.2.neg())
    }
    fn rotate_x3(&self) -> Self {
        Point(self.0, self.2.neg(), self.1)
    }
    fn rotate_y(&self) -> Self {
        Point(self.2, self.1, self.0.neg())
    }
    fn rotate_y2(&self) -> Self {
        Point(self.0.neg(), self.1, self.2.neg())
    }
    fn rotate_y3(&self) -> Self {
        Point(self.2.neg(), self.1, self.0)
    }
    fn rotate_z(&self) -> Self {
        Point(self.1, self.0.neg(), self.2)
    }
    fn rotate_z3(&self) -> Self {
        Point(self.1.neg(), self.0, self.2)
    }
    fn manhattan_distance(&self, other: &Self) -> i64 {
        self.0.sub(other.0).abs() + self.1.sub(other.1).abs() + self.2.sub(other.2).abs()
    }
}

impl FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed_numbers: Vec<i64> = s
            .split(",")
            .map(|s| s.parse::<i64>().map_err(|_| "Can't parse number"))
            .try_collect()?;
        match parsed_numbers[..] {
            [a, b, c] => Ok(Point(a, b, c)),
            _ => Err("Bad line"),
        }
    }
}

impl Add<&Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Self::Output {
        Point(self.0.add(rhs.0), self.1.add(rhs.1), self.2.add(rhs.2))
    }
}

impl Sub for &Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector(self.0.sub(rhs.0), self.1.sub(rhs.1), self.2.sub(rhs.2))
    }
}

#[derive(Debug, PartialEq, Clone)]
struct BeaconPoints(HashSet<Point>);

impl BeaconPoints {
    fn new<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        BeaconPoints(HashSet::from_iter(iter))
    }
}

impl Add<&Vector> for &BeaconPoints {
    type Output = BeaconPoints;

    fn add(self, rhs: &Vector) -> Self::Output {
        BeaconPoints(HashSet::from_iter(self.0.iter().map(|p| p + rhs)))
    }
}

#[derive(Debug)]
struct BeaconPointCases([BeaconPoints; 24]);

impl BeaconPointCases {
    fn new(points: &Vec<Point>) -> Self {
        BeaconPointCases([
            BeaconPoints::new(points.iter().map(|p| p.clone())),
            BeaconPoints::new(points.iter().map(|p| p.rotate_x())),
            BeaconPoints::new(points.iter().map(|p| p.rotate_x2())),
            BeaconPoints::new(points.iter().map(|p| p.rotate_x3())),
            BeaconPoints::new(points.iter().map(|p| p.rotate_y())),
            BeaconPoints::new(points.iter().map(|p| p.rotate_y().rotate_x())),
            BeaconPoints::new(points.iter().map(|p| p.rotate_y().rotate_x2())),
            BeaconPoints::new(points.iter().map(|p| p.rotate_y().rotate_x3())),
            BeaconPoints::new(points.iter().map(|p| p.rotate_y2())),
            BeaconPoints::new(points.iter().map(|p| p.rotate_y2().rotate_x())),
            BeaconPoints::new(points.iter().map(|p| p.rotate_y2().rotate_x2())),
            BeaconPoints::new(points.iter().map(|p| p.rotate_y2().rotate_x3())),
            BeaconPoints::new(points.iter().map(|p| p.rotate_y3())),
            BeaconPoints::new(points.iter().map(|p| p.rotate_y3().rotate_x())),
            BeaconPoints::new(points.iter().map(|p| p.rotate_y3().rotate_x2())),
            BeaconPoints::new(points.iter().map(|p| p.rotate_y3().rotate_x3())),
            BeaconPoints::new(points.iter().map(|p| p.rotate_z())),
            BeaconPoints::new(points.iter().map(|p| p.rotate_z().rotate_x())),
            BeaconPoints::new(points.iter().map(|p| p.rotate_z().rotate_x2())),
            BeaconPoints::new(points.iter().map(|p| p.rotate_z().rotate_x3())),
            BeaconPoints::new(points.iter().map(|p| p.rotate_z3())),
            BeaconPoints::new(points.iter().map(|p| p.rotate_z3().rotate_x())),
            BeaconPoints::new(points.iter().map(|p| p.rotate_z3().rotate_x2())),
            BeaconPoints::new(points.iter().map(|p| p.rotate_z3().rotate_x3())),
        ])
    }
    fn get_unrotated_points(self) -> BeaconPoints {
        self.0[0].clone()
    }
    fn try_resolve_relative_to_scanner(
        &self,
        positioned_scanner: &PositionedScanner,
        goal: usize,
    ) -> Option<PositionedScanner> {
        for (positioned_point, rotated_points) in positioned_scanner
            .beacons
            .0
            .iter()
            .cartesian_product(self.0.iter())
        {
            for rotated_point in rotated_points.0.iter() {
                let change = positioned_point - rotated_point;
                let new_beacon_positions = rotated_points + &change;
                let count = positioned_scanner
                    .beacons
                    .0
                    .intersection(&new_beacon_positions.0)
                    .count();
                if count >= goal {
                    return Some(PositionedScanner {
                        position: &Point::default() + &change,
                        beacons: new_beacon_positions,
                    });
                }
            }
        }
        None
    }
}

fn generate_beacon_point_cases() -> Result<Vec<BeaconPointCases>, &'static str> {
    let mut beacon_cases: Vec<BeaconPointCases> = Vec::default();
    let mut points: Vec<Point> = Vec::default();
    for line in BufReader::new(std::io::stdin()).lines() {
        let s = line.map_err(|_| "Bad line")?;
        if s.contains("---") {
            if !points.is_empty() {
                beacon_cases.push(BeaconPointCases::new(&points));
                points = Vec::default()
            }
        } else if !s.is_empty() {
            points.push(s.parse::<Point>()?)
        }
    }
    beacon_cases.push(BeaconPointCases::new(&points));

    Ok(beacon_cases)
}

#[derive(Debug, PartialEq)]
struct PositionedScanner {
    position: Point,
    beacons: BeaconPoints,
}
struct World {
    scanners: Vec<PositionedScanner>,
    unresolved: Vec<(usize, BeaconPointCases)>,
}

fn generate_world(mut beacon_point_cases: Vec<BeaconPointCases>) -> Option<Vec<PositionedScanner>> {
    let start_scanner = PositionedScanner {
        position: Point::default(),
        beacons: beacon_point_cases.pop()?.get_unrotated_points(),
    };
    let mut world = World {
        scanners: vec![start_scanner],
        unresolved: beacon_point_cases.into_iter().enumerate().collect_vec(),
    };
    while !world.unresolved.is_empty() {
        world = world.unresolved.into_iter().fold(
            World {
                scanners: world.scanners,
                unresolved: Vec::new(),
            },
            |mut w, (case_index, bpc)| {
                let generate_positioned_scanner =
                    w.scanners
                        .iter()
                        .enumerate()
                        .find_map(|(i, positioned_scanner)| {
                            bpc.try_resolve_relative_to_scanner(positioned_scanner, 12)
                                .map(|ps| (i, ps))
                        });
                if let Some((i, new_scanner)) = generate_positioned_scanner {
                    println!("Adding scanner for case: {} matching: {}", case_index, i);
                    w.scanners.push(new_scanner);
                } else {
                    println!("Unresolved scanner for case: {}", case_index);
                    w.unresolved.push((case_index, bpc));
                }
                w
            },
        );
    }
    Some(world.scanners)
}

fn main() -> Result<(), &'static str> {
    let readings = generate_beacon_point_cases()?;

    let world = generate_world(readings).unwrap();

    let mut points: HashSet<Point> = HashSet::new();
    let mut scanners: Vec<Point> = Vec::new();
    for positioned_scanner in world {
        points.extend(positioned_scanner.beacons.0);
        scanners.push(positioned_scanner.position);
    }
    println!("Total unique beacons: {}", points.len());

    let max_manhattan_distance = scanners
        .iter()
        .cartesian_product(scanners.iter())
        .map(|(a, b)| a.manhattan_distance(b))
        .max()
        .unwrap();
    println!("Max manhattan distance: {}", max_manhattan_distance);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_all_rotations_of_point_in_scanner_reading() {
        let reading = BeaconPointCases::new(&vec![Point(1, 2, 3)]);
        let points: [Point; 24] = reading.0.map(|h| h.0.into_iter().next().unwrap());
        assert_eq!(
            points,
            [
                Point(1, 2, 3),
                Point(1, 3, -2),
                Point(1, -2, -3),
                Point(1, -3, 2),
                Point(3, 2, -1),
                Point(3, -1, -2),
                Point(3, -2, 1),
                Point(3, 1, 2),
                Point(-1, 2, -3),
                Point(-1, -3, -2),
                Point(-1, -2, 3),
                Point(-1, 3, 2),
                Point(-3, 2, 1),
                Point(-3, 1, -2),
                Point(-3, -2, -1),
                Point(-3, -1, 2),
                Point(2, -1, 3),
                Point(2, 3, 1),
                Point(2, 1, -3),
                Point(2, -3, -1),
                Point(-2, 1, 3),
                Point(-2, 3, -1),
                Point(-2, -1, -3),
                Point(-2, -3, 1),
            ]
        );
    }

    #[test]

    fn check_point_subtraction() {
        assert_eq!(Point(4, 1, 5).sub(&Point(-1, -1, 5)), Vector(5, 2, 0));
        assert_eq!(Point(4, 1, 5).sub(&Point(-2, 1, 5)), Vector(6, 0, 0));
        assert_eq!(Point(4, 1, 5).sub(&Point(4, 0, 5)), Vector(0, 1, 0));
    }

    #[test]
    fn check_find_best_overlap() {
        let start_scanner = PositionedScanner {
            position: Point::default(),
            beacons: BeaconPoints::new(vec![
                Point(0, 2, 5),
                Point(4, 1, 5),
                Point(3, 3, 5),
                Point(-2, 2, -2),
            ]),
        };

        let scan = BeaconPointCases::new(&vec![
            Point(-1, -1, 5),
            Point(-2, 1, 5),
            Point(4, 0, 5),
            Point(-5, 0, 5),
        ]);

        let resolved_scanner = scan.try_resolve_relative_to_scanner(&start_scanner, 3);

        assert_eq!(
            resolved_scanner,
            Some(PositionedScanner {
                position: Point(5, 2, 0),
                beacons: BeaconPoints::new(vec![
                    Point(0, 2, 5),
                    Point(4, 1, 5),
                    Point(3, 3, 5),
                    Point(9, 2, 5)
                ])
            })
        );
    }

    #[test]
    fn check_find_best_overlap_with_rotation() {
        let start_scanner = PositionedScanner {
            position: Point::default(),
            beacons: BeaconPoints::new(vec![
                Point(-1, -1, 1),
                Point(-2, -2, 2),
                Point(-3, -3, 3),
                Point(-2, -3, 1),
                Point(5, 6, -4),
                Point(8, 0, 7),
            ]),
        };

        let scans = vec![
            BeaconPointCases::new(&vec![
                Point(-1, -1, 1),
                Point(-2, -2, 2),
                Point(-3, -3, 3),
                Point(-2, -3, 1),
                Point(5, 6, -4),
                Point(8, 0, 7),
            ]),
            BeaconPointCases::new(&vec![
                Point(1, -1, 1),
                Point(2, -2, 2),
                Point(3, -3, 3),
                Point(2, -1, 3),
                Point(-5, 4, -6),
                Point(-8, -7, 0),
            ]),
            BeaconPointCases::new(&vec![
                Point(-1, -1, -1),
                Point(-2, -2, -2),
                Point(-3, -3, -3),
                Point(-1, -3, -2),
                Point(4, 6, 5),
                Point(-7, 0, 8),
            ]),
            BeaconPointCases::new(&vec![
                Point(1, 1, -1),
                Point(2, 2, -2),
                Point(3, 3, -3),
                Point(1, 3, -2),
                Point(-4, -6, 5),
                Point(7, 0, 8),
            ]),
            BeaconPointCases::new(&vec![
                Point(1, 1, 1),
                Point(2, 2, 2),
                Point(3, 3, 3),
                Point(3, 1, 2),
                Point(-6, -4, -5),
                Point(0, 7, -8),
            ]),
        ];

        assert_eq!(
            &scans[0]
                .try_resolve_relative_to_scanner(&start_scanner, 6)
                .unwrap(),
            &start_scanner
        );

        assert_eq!(
            &scans[1]
                .try_resolve_relative_to_scanner(&start_scanner, 6)
                .unwrap(),
            &start_scanner
        );

        assert_eq!(
            &scans[2]
                .try_resolve_relative_to_scanner(&start_scanner, 6)
                .unwrap(),
            &start_scanner
        );

        assert_eq!(
            &scans[3]
                .try_resolve_relative_to_scanner(&start_scanner, 6)
                .unwrap(),
            &start_scanner
        );

        assert_eq!(
            &scans[4]
                .try_resolve_relative_to_scanner(&start_scanner, 6)
                .unwrap(),
            &start_scanner
        );
    }

    #[test]
    fn check_find_best_overlap_larger_example() {
        let start_scanner = PositionedScanner {
            position: Point::default(),
            beacons: BeaconPoints::new(vec![
                Point(-345, -311, 381),
                Point(-447, -329, 318),
                Point(-485, -357, 347),
                Point(-537, -823, -458),
                Point(-584, 868, -557),
                Point(-618, -824, -621),
                Point(-661, -816, -575),
                Point(-689, 845, -530),
                Point(-789, 900, -551),
                Point(-838, 591, 734),
                Point(-876, 649, 763),
                Point(-892, 524, 684),
                Point(390, -675, -793),
                Point(404, -588, -901),
                Point(423, -701, 434),
                Point(443, 580, 662),
                Point(455, 729, 728),
                Point(459, -707, 401),
                Point(474, 580, 667),
                Point(528, -643, 409),
                Point(544, -627, -890),
                Point(553, 345, -567),
                Point(564, 392, -477),
                Point(630, 319, -379),
                Point(7, -33, -71),
            ]),
        };

        let scan = BeaconPointCases::new(&vec![
            Point(-322, 571, 750),
            Point(-328, -685, 520),
            Point(-336, 658, 858),
            Point(-340, -569, -846),
            Point(-355, 545, -477),
            Point(-364, -763, -893),
            Point(-391, 539, -444),
            Point(-429, -592, 574),
            Point(-460, 603, -452),
            Point(-466, -666, -811),
            Point(-476, 619, 847),
            Point(-500, -761, 534),
            Point(413, 935, -424),
            Point(515, 917, -361),
            Point(553, 889, -390),
            Point(567, -361, 727),
            Point(586, -435, 557),
            Point(605, 423, 415),
            Point(669, -402, 600),
            Point(686, 422, 578),
            Point(703, -491, -529),
            Point(729, 430, 532),
            Point(755, -354, -619),
            Point(807, -499, -711),
            Point(95, 138, 22),
        ]);

        let resolved_scanner = scan.try_resolve_relative_to_scanner(&start_scanner, 12);

        assert_eq!(
            resolved_scanner,
            Some(PositionedScanner {
                position: Point(68, -1246, -43),
                beacons: BeaconPoints::new(vec![
                    Point(-27, -1108, -65),
                    Point(-345, -311, 381),
                    Point(-447, -329, 318),
                    Point(-485, -357, 347),
                    Point(-499, -1607, -770),
                    Point(-518, -1681, -600),
                    Point(-537, -823, -458),
                    Point(-601, -1648, -643),
                    Point(-618, -824, -621),
                    Point(-635, -1737, 486),
                    Point(-661, -816, -575),
                    Point(-687, -1600, 576),
                    Point(-739, -1745, 668),
                    Point(390, -675, -793),
                    Point(396, -1931, -563),
                    Point(404, -588, -901),
                    Point(408, -1815, 803),
                    Point(423, -701, 434),
                    Point(432, -2009, 850),
                    Point(459, -707, 401),
                    Point(497, -1838, -617),
                    Point(528, -643, 409),
                    Point(534, -1912, 768),
                    Point(544, -627, -890),
                    Point(568, -2007, -577),
                ])
            })
        );
    }
}
