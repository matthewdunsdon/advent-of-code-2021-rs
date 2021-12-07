use std::collections::HashMap;
use std::io::{BufRead, BufReader};

type Point = (i16, i16);
type Line = (Point, Point);

fn change_delta(x: &i16, y: &i16) -> i16 {
    match y - x {
        n if n > 0 => 1,
        n if n < 0 => -1,
        _ => 0,
    }
}

fn to_points((start, end): &Line) -> Vec<Point> {
    let mut points = Vec::new();
    let change_delta_for_x = change_delta(&start.0, &end.0);
    let change_delta_for_y = change_delta(&start.1, &end.1);

    let mut p = start.clone();
    while &p != end {
        points.push(p);
        p = (p.0 + change_delta_for_x, p.1 + change_delta_for_y)
    }
    points.push(p);
    points
}

fn parse_line(line: String) -> Option<Line> {
    let numbers: Vec<i16> = line
        .split(" -> ")
        .flat_map(|p| p.split(","))
        .filter_map(|z| z.parse::<i16>().ok())
        .collect();

    match numbers[..] {
        [x1, y1, x2, y2] => Some(((x1, y1), (x2, y2))),
        _ => None,
    }
}

fn track_points(lines: &Vec<Line>) -> Vec<Point> {
    let points: HashMap<Point, i16> =
        lines
            .iter()
            .flat_map(to_points)
            .fold(HashMap::new(), |mut x, p| {
                let count = x.entry(p).or_insert(0);
                *count += 1;
                x
            });

    points
        .into_iter()
        .filter(|(_, x)| *x > 1)
        .map(|(p, _)| p)
        .collect()
}

fn main() {
    let lines: Vec<Line> = BufReader::new(std::io::stdin())
        .lines()
        .filter_map(Result::ok)
        .filter_map(parse_line)
        .collect();

    let non_diagonal_lines: Vec<Line> = lines
        .clone()
        .into_iter()
        .filter(|(start, end)| start.0 == end.0 || start.1 == end.1)
        .collect();

    let count_without_diagonals = track_points(&non_diagonal_lines);
    let count = track_points(&lines);

    println!(
        "Found cells without diagonals: {}",
        count_without_diagonals.len()
    );
    println!("Found cells: {}", count.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn check_parse_line() {
        assert_eq!(parse_line("0,9 -> 5,9".to_owned()), Some(((0, 9), (5, 9))));
        assert_eq!(parse_line("2,2 -> 2,1".to_owned()), Some(((2, 2), (2, 1))));
    }

    #[test]
    fn check_to_points() {
        assert_eq!(
            to_points(&((0, 9), (5, 9))),
            vec!((0, 9), (1, 9), (2, 9), (3, 9), (4, 9), (5, 9))
        );
        assert_eq!(
            to_points(&((7, 0), (7, 4))),
            vec!((7, 0), (7, 1), (7, 2), (7, 3), (7, 4))
        );

        assert_eq!(
            to_points(&((9, 4), (3, 4))),
            vec!((9, 4), (8, 4), (7, 4), (6, 4), (5, 4), (4, 4), (3, 4))
        );

        assert_eq!(
            to_points(&((6, 4), (2, 0))),
            vec!((6, 4), (5, 3), (4, 2), (3, 1), (2, 0))
        );
    }

    #[test]
    fn check_track_points() {
        let lines = [
            ((0, 9), (5, 9)), // Line 1
            ((9, 4), (3, 4)), // Line 2
            ((2, 2), (2, 1)), // Line 3
            ((7, 0), (7, 4)), // Line 4
            ((0, 9), (2, 9)), // Line 5
            ((3, 4), (1, 4)), // Line 6
            ((8, 0), (0, 8)), // Line 7
            ((6, 4), (2, 0)), // Line 8
            ((0, 0), (8, 8)), // Line 9
            ((5, 5), (8, 2)), // Line 10
        ];
        let total_matches = [
            (7, 4), // From line: 4
            (0, 9), // From line: 5
            (1, 9), // From line: 5
            (2, 9), // From line: 5
            (3, 4), // From line: 6
            (4, 4), // From line: 7
            (7, 1), // From line: 7
            (5, 3), // From line: 8
            (6, 4), // From line: 8
            (2, 2), // From line: 9
            (5, 5), // From line: 10
            (7, 3), // From line: 10
        ];

        iters_equal_anyorder(&track_points(&lines[..1].to_vec()), &Vec::new());
        iters_equal_anyorder(&track_points(&lines[..2].to_vec()), &Vec::new());
        iters_equal_anyorder(&track_points(&lines[..3].to_vec()), &Vec::new());
        iters_equal_anyorder(&track_points(&lines[..4].to_vec()), &total_matches[..1]);
        iters_equal_anyorder(&track_points(&lines[..5].to_vec()), &total_matches[..4]);
        iters_equal_anyorder(&track_points(&lines[..6].to_vec()), &total_matches[..5]);
        iters_equal_anyorder(&track_points(&lines[..7].to_vec()), &total_matches[..7]);
        iters_equal_anyorder(&track_points(&lines[..8].to_vec()), &total_matches[..9]);
        iters_equal_anyorder(&track_points(&lines[..9].to_vec()), &total_matches[..10]);
        iters_equal_anyorder(&track_points(&lines[..10].to_vec()), &total_matches[..12]);
    }

    fn iters_equal_anyorder(actual: &[Point], expected: &[Point]) {
        let m = HashSet::<_>::from_iter(actual.iter());
        let n = HashSet::<_>::from_iter(expected.iter());
        assert_eq!(m, n);
    }
}
