use itertools::Itertools;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
struct Point {
    row: usize,
    col: usize,
}

fn get_neighbouring_points(
    p: &Point,
    col_range: &impl std::ops::RangeBounds<usize>,
    row_range: &impl std::ops::RangeBounds<usize>,
) -> Vec<Point> {
    let mut points = Vec::default();
    if p.row > 0 {
        points.push(Point {
            row: p.row - 1,
            col: p.col,
        });
    }
    if p.col > 0 {
        points.push(Point {
            row: p.row,
            col: p.col - 1,
        });
    }
    if row_range.contains(&(p.row + 1)) {
        points.push(Point {
            row: p.row + 1,
            col: p.col,
        });
    }
    if col_range.contains(&(p.col + 1)) {
        points.push(Point {
            row: p.row,
            col: p.col + 1,
        });
    }
    points
}

fn extract_low_points(grid: &[Vec<char>]) -> Vec<Point> {
    let row_range = 0..grid.len();
    let column_range = 0..grid[0].len();

    row_range
        .clone()
        .cartesian_product(column_range.clone())
        .map(|(row, col)| Point { row, col })
        .filter(|p| {
            get_neighbouring_points(p, &column_range, &row_range)
                .into_iter()
                .all(|n| grid[n.row][n.col] > grid[p.row][p.col])
        })
        .collect()
}

fn get_basin(grid: &[Vec<char>], point: &Point) -> Vec<Point> {
    get_basin_acc(
        grid,
        vec![Point {
            row: point.row,
            col: point.col,
        }],
        Vec::default(),
    )
}

fn get_basin_acc(
    grid: &[Vec<char>],
    mut candidates: Vec<Point>,
    mut basin: Vec<Point>,
) -> Vec<Point> {
    if candidates.is_empty() {
        basin
    } else {
        let next_points = candidates
            .iter()
            .flat_map(|c| get_neighbouring_points(c, &(0..grid[0].len()), &(0..grid.len())))
            .filter(|n| grid[n.row][n.col] != '9')
            .filter(|n| !basin.contains(n))
            .unique_by(|n| (n.row, n.col))
            .collect_vec();

        basin.append(&mut candidates);
        get_basin_acc(grid, next_points, basin)
    }
}

fn main() -> Result<(), String> {
    let grid = BufReader::new(std::io::stdin())
        .lines()
        .map(|r| r.map(|s| s.chars().collect::<Vec<char>>()))
        .collect::<Result<Vec<Vec<char>>, _>>()
        .map_err(|err| err.to_string())?;

    let points = extract_low_points(&grid);
    let risk_level: u32 = points
        .iter()
        .map(|p| grid[p.row][p.col].to_digit(10).unwrap() + 1)
        .sum();

    println!("risk level {:?}", risk_level);

    let basins = points
        .iter()
        .map(|p| get_basin(&grid, p))
        .sorted_by(|basin_a, basin_b| Ord::cmp(&basin_b.len(), &basin_a.len()))
        .collect::<Vec<_>>();

    match &basins[..3] {
        [basin_1, basin_2, basin_3] => {
            let basin_1_len = basin_1.len();
            let basin_2_len = basin_2.len();
            let basin_3_len = basin_3.len();
            println!(
                "The three largest basins had the follwoing sizes: {}, {}, {}",
                basin_1.len(),
                basin_2.len(),
                basin_3.len()
            );
            println!(
                "Multplied together we get: {}",
                basin_1_len * basin_2_len * basin_3_len
            );
        }
        _ => {
            return Err(format!(
                "There were less than three basins: {} found",
                basins.len()
            ))
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_grid() -> [Vec<char>; 5] {
        [
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ]
        .map(|s| s.chars().collect::<Vec<char>>())
    }

    #[test]
    fn check_extract_low_points() {
        assert_eq!(
            extract_low_points(&get_sample_grid()),
            vec![
                Point { row: 0, col: 1 },
                Point { row: 0, col: 9 },
                Point { row: 2, col: 2 },
                Point { row: 4, col: 6 },
            ]
        );
    }

    #[test]
    fn check_get_basin() {
        let grid = get_sample_grid();

        assert_eq!(
            get_basin(&grid, &Point { row: 0, col: 1 }),
            vec![
                Point { row: 0, col: 1 },
                Point { row: 0, col: 0 },
                Point { row: 1, col: 0 },
            ]
        );

        assert_eq!(
            get_basin(&grid, &Point { row: 0, col: 9 }),
            vec![
                Point { row: 0, col: 9 },
                Point { row: 0, col: 8 },
                Point { row: 1, col: 9 },
                Point { row: 0, col: 7 },
                Point { row: 1, col: 8 },
                Point { row: 2, col: 9 },
                Point { row: 0, col: 6 },
                Point { row: 0, col: 5 },
                Point { row: 1, col: 6 },
            ]
        );

        assert_eq!(
            get_basin(&grid, &Point { row: 2, col: 2 }),
            vec![
                Point { row: 2, col: 2 },
                Point { row: 1, col: 2 },
                Point { row: 2, col: 1 },
                Point { row: 3, col: 2 },
                Point { row: 2, col: 3 },
                Point { row: 1, col: 3 },
                Point { row: 3, col: 1 },
                Point { row: 3, col: 3 },
                Point { row: 2, col: 4 },
                Point { row: 1, col: 4 },
                Point { row: 3, col: 0 },
                Point { row: 4, col: 1 },
                Point { row: 3, col: 4 },
                Point { row: 2, col: 5 },
            ]
        );

        assert_eq!(
            get_basin(&grid, &Point { row: 4, col: 6 }),
            vec![
                Point { row: 4, col: 6 },
                Point { row: 3, col: 6 },
                Point { row: 4, col: 5 },
                Point { row: 4, col: 7 },
                Point { row: 3, col: 7 },
                Point { row: 4, col: 8 },
                Point { row: 2, col: 7 },
                Point { row: 3, col: 8 },
                Point { row: 4, col: 9 },
            ]
        );
    }
}
