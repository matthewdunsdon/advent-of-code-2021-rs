use itertools::Itertools;
use std::{
    cmp,
    io::{BufRead, BufReader},
    num::ParseIntError,
};

#[derive(Debug)]
enum FoldAlong {
    X(u16),
    Y(u16),
}

type Point = (u16, u16);

fn parse(lines: Vec<String>) -> Result<(Vec<Point>, Vec<FoldAlong>), String> {
    let mut points: Vec<Point> = Vec::default();
    let mut folds: Vec<FoldAlong> = Vec::default();

    let mut iter = lines.into_iter();
    while let Some(val) = iter.next() {
        if val == "" {
            break;
        }
        let parts = val
            .split(",")
            .map(|s| s.parse())
            .collect::<Result<Vec<u16>, _>>()
            .map_err(|e| e.to_string())?;

        match parts[..] {
            [a, b] => points.push((a, b)),
            _ => return Err(format!("Could not parse point from '{}'", val)),
        }
    }
    while let Some(val) = iter.next() {
        let parts = val.split("=").collect::<Vec<&str>>();

        let (f_action, amount) = match parts[..] {
            [a, b] => (a, b),
            _ => return Err(format!("Could not parse fold from '{}'", val)),
        };

        let amount: u16 = amount.parse().map_err(|e: ParseIntError| e.to_string())?;

        match f_action {
            "fold along x" => folds.push(FoldAlong::X(amount)),
            "fold along y" => folds.push(FoldAlong::Y(amount)),
            _ => return Err(format!("Could not parse fold instruction '{}'", f_action)),
        }
    }

    Ok((points, folds))
}

fn fold_points(fold: &FoldAlong, points: &[Point]) -> Vec<Point> {
    match fold {
        &FoldAlong::X(x) => points
            .into_iter()
            .map(|p| (if p.0 > x { 2 * x - p.0 } else { p.0 }, p.1))
            .unique()
            .collect(),
        &FoldAlong::Y(y) => points
            .into_iter()
            .map(|p| (p.0, if p.1 > y { 2 * y - p.1 } else { p.1 }))
            .unique()
            .collect(),
    }
}

fn generate_drawing(points: &[Point]) -> Vec<String> {
    let (max_x, max_y) = points.iter().fold((0, 0), |agg, p| {
        (cmp::max(p.0, agg.0), cmp::max(p.1, agg.1))
    });

    (0..=max_y)
        .into_iter()
        .map(|y| {
            (0..=max_x)
                .into_iter()
                .fold(String::with_capacity(usize::from(max_x)), |mut s, x| {
                    if points.contains(&(x, y)) {
                        s.push('🌍')
                    } else {
                        s.push('🦙')
                    }
                    s
                })
        })
        .collect()
}

fn main() -> Result<(), String> {
    let lines: Vec<String> = BufReader::new(std::io::stdin())
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .map_err(|e| e.to_string())?;

    let (points, folds) = parse(lines)?;

    let mut p = points;
    println!("Initially there were {} points", p.len());
    for fold_along in folds {
        p = fold_points(&fold_along, &p);
        println!(
            "After fold along {:?} there were {} points",
            fold_along,
            p.len()
        );
    }

    let lines = generate_drawing(&p);
    for line in lines {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_fold_points_for_y() {
        let points: Vec<Point> = vec![
            (6, 10),
            (0, 14),
            (9, 10),
            (0, 3),
            (10, 4),
            (4, 11),
            (6, 0),
            (6, 12),
            (4, 1),
            (0, 13),
            (10, 12),
            (3, 4),
            (3, 0),
            (8, 4),
            (1, 10),
            (2, 14),
            (8, 10),
            (9, 0),
        ];

        let mut actual = fold_points(&FoldAlong::Y(7), &points);
        actual.sort();

        assert_eq!(
            actual,
            vec![
                (0, 0),
                (0, 1),
                (0, 3),
                (1, 4),
                (2, 0),
                (3, 0),
                (3, 4),
                (4, 1),
                (4, 3),
                (6, 0),
                (6, 2),
                (6, 4),
                (8, 4),
                (9, 0),
                (9, 4),
                (10, 2),
                (10, 4),
            ]
        );
    }

    #[test]
    fn check_fold_points_for_x() {
        let points: Vec<Point> = vec![
            (0, 0),
            (0, 1),
            (0, 3),
            (1, 4),
            (2, 0),
            (3, 0),
            (3, 4),
            (4, 1),
            (4, 3),
            (6, 0),
            (6, 2),
            (6, 4),
            (8, 4),
            (9, 0),
            (9, 4),
            (10, 2),
            (10, 4),
        ];

        let expected: Vec<Point> = vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (1, 0),
            (1, 4),
            (2, 0),
            (2, 4),
            (3, 0),
            (3, 4),
            (4, 0),
            (4, 1),
            (4, 2),
            (4, 3),
            (4, 4),
        ];

        let mut actual = fold_points(&FoldAlong::X(5), &points);
        actual.sort();

        assert_eq!(actual, expected);
    }

    #[test]
    fn check_generate_drawing() {
        let points: Vec<Point> = vec![
            (6, 10),
            (0, 14),
            (9, 10),
            (0, 3),
            (10, 4),
            (4, 11),
            (6, 0),
            (6, 12),
            (4, 1),
            (0, 13),
            (10, 12),
            (3, 4),
            (3, 0),
            (8, 4),
            (1, 10),
            (2, 14),
            (8, 10),
            (9, 0),
        ];

        assert_eq!(
            generate_drawing(&points),
            vec![
                "🦙🦙🦙🌍🦙🦙🌍🦙🦙🌍🦙",
                "🦙🦙🦙🦙🌍🦙🦙🦙🦙🦙🦙",
                "🦙🦙🦙🦙🦙🦙🦙🦙🦙🦙🦙",
                "🌍🦙🦙🦙🦙🦙🦙🦙🦙🦙🦙",
                "🦙🦙🦙🌍🦙🦙🦙🦙🌍🦙🌍",
                "🦙🦙🦙🦙🦙🦙🦙🦙🦙🦙🦙",
                "🦙🦙🦙🦙🦙🦙🦙🦙🦙🦙🦙",
                "🦙🦙🦙🦙🦙🦙🦙🦙🦙🦙🦙",
                "🦙🦙🦙🦙🦙🦙🦙🦙🦙🦙🦙",
                "🦙🦙🦙🦙🦙🦙🦙🦙🦙🦙🦙",
                "🦙🌍🦙🦙🦙🦙🌍🦙🌍🌍🦙",
                "🦙🦙🦙🦙🌍🦙🦙🦙🦙🦙🦙",
                "🦙🦙🦙🦙🦙🦙🌍🦙🦙🦙🌍",
                "🌍🦙🦙🦙🦙🦙🦙🦙🦙🦙🦙",
                "🌍🦙🌍🦙🦙🦙🦙🦙🦙🦙🦙"
            ]
        );
        let points: Vec<Point> = vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (1, 0),
            (1, 4),
            (2, 0),
            (2, 4),
            (3, 0),
            (3, 4),
            (4, 0),
            (4, 1),
            (4, 2),
            (4, 3),
            (4, 4),
        ];

        assert_eq!(
            generate_drawing(&points),
            vec![
                "🌍🌍🌍🌍🌍",
                "🌍🦙🦙🦙🌍",
                "🌍🦙🦙🦙🌍",
                "🌍🦙🦙🦙🌍",
                "🌍🌍🌍🌍🌍"
            ]
        );
    }
}
