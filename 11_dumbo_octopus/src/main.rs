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
    let support_top = p.row > 0;
    let support_left = p.col > 0;
    let support_bottom = row_range.contains(&(p.row + 1));
    let support_right = col_range.contains(&(p.col + 1));
    if support_top {
        points.push(Point {
            row: p.row - 1,
            col: p.col,
        });
        if support_left {
            points.push(Point {
                row: p.row - 1,
                col: p.col - 1,
            });
        }
        if support_right {
            points.push(Point {
                row: p.row - 1,
                col: p.col + 1,
            });
        }
    }
    if support_bottom {
        points.push(Point {
            row: p.row + 1,
            col: p.col,
        });
        if support_left {
            points.push(Point {
                row: p.row + 1,
                col: p.col - 1,
            });
        }
        if support_right {
            points.push(Point {
                row: p.row + 1,
                col: p.col + 1,
            });
        }
    }
    if support_left {
        points.push(Point {
            row: p.row,
            col: p.col - 1,
        });
    }
    if support_right {
        points.push(Point {
            row: p.row,
            col: p.col + 1,
        });
    }
    points
}

fn handle_flashes(mut grid: Vec<Vec<u32>>, flash_candidates: Vec<Point>) -> Vec<Vec<u32>> {
    if flash_candidates.is_empty() {
        grid
    } else {
        let mut next_candidates = Vec::default();
        let row_range = 0..grid.len();
        let column_range = 0..grid[0].len();

        flash_candidates
            .into_iter()
            .flat_map(|p| get_neighbouring_points(&p, &column_range, &row_range))
            .for_each(|p| {
                let next_val = grid[p.row][p.col] + 1;
                grid[p.row][p.col] = next_val;
                if next_val == 10 {
                    next_candidates.push(p)
                }
            });
        handle_flashes(grid, next_candidates)
    }
}

fn reset_zeros(mut grid: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .map(|(row, col)| Point { row, col })
        .for_each(|p| {
            if grid[p.row][p.col] > 9 {
                grid[p.row][p.col] = 0;
            }
        });

    grid
}

fn count_grid(grid: &[Vec<u32>]) -> (i32, i32) {
    let mut count = (0, 0);
    (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .map(|(row, col)| Point { row, col })
        .for_each(|p| {
            if grid[p.row][p.col] == 0 {
                count = (count.0, count.1 + 1);
            } else {
                count = (count.0 + 1, count.1);
            }
        });

    count
}

fn take_step(mut grid: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut flash_candidates = Vec::default();

    (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .map(|(row, col)| Point { row, col })
        .for_each(|p| {
            let next_val = grid[p.row][p.col] + 1;
            grid[p.row][p.col] = next_val;
            if next_val == 10 {
                flash_candidates.push(p)
            }
        });

    reset_zeros(handle_flashes(grid, flash_candidates))
}

fn main() -> Result<(), String> {
    let mut grid = BufReader::new(std::io::stdin())
        .lines()
        .map(|r| {
            r.map_err(|e| e.to_string()).and_then(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).ok_or(format!("Non digit found: {}", c)))
                    .collect::<Result<Vec<u32>, _>>()
            })
        })
        .collect::<Result<Vec<Vec<u32>>, _>>()?;
    let mut zero_count = count_grid(&grid).1;
    let mut all_octopuses_flash = None;
    let mut index = 0;
    for _ in 0..100 {
        index += 1;
        grid = take_step(grid);
        let (non_zeros, zeros) = count_grid(&grid);
        if non_zeros == 0 {
            all_octopuses_flash = Some(index);
        }

        zero_count += zeros;
    }
    while all_octopuses_flash.is_none() {
        index += 1;
        grid = take_step(grid);
        let (non_zeros, _) = count_grid(&grid);
        if non_zeros == 0 {
            all_octopuses_flash = Some(index);
        }
    }
    println!("Zero count at step 100: {:?}", zero_count);
    println!("All zero first count: {:?}", all_octopuses_flash.unwrap());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inital_grid() -> Vec<Vec<u32>> {
        vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ]
    }

    #[test]
    fn check_parse_line() {
        let mut grid = inital_grid();

        grid = take_step(grid);
        assert_eq!(
            grid,
            vec![
                vec![6, 5, 9, 4, 2, 5, 4, 3, 3, 4],
                vec![3, 8, 5, 6, 9, 6, 5, 8, 2, 2],
                vec![6, 3, 7, 5, 6, 6, 7, 2, 8, 4],
                vec![7, 2, 5, 2, 4, 4, 7, 2, 5, 7],
                vec![7, 4, 6, 8, 4, 9, 6, 5, 8, 9],
                vec![5, 2, 7, 8, 6, 3, 5, 7, 5, 6],
                vec![3, 2, 8, 7, 9, 5, 2, 8, 3, 2],
                vec![7, 9, 9, 3, 9, 9, 2, 2, 4, 5],
                vec![5, 9, 5, 7, 9, 5, 9, 6, 6, 5],
                vec![6, 3, 9, 4, 8, 6, 2, 6, 3, 7],
            ]
        );

        grid = take_step(grid);
        assert_eq!(
            grid,
            vec![
                vec![8, 8, 0, 7, 4, 7, 6, 5, 5, 5],
                vec![5, 0, 8, 9, 0, 8, 7, 0, 5, 4],
                vec![8, 5, 9, 7, 8, 8, 9, 6, 0, 8],
                vec![8, 4, 8, 5, 7, 6, 9, 6, 0, 0],
                vec![8, 7, 0, 0, 9, 0, 8, 8, 0, 0],
                vec![6, 6, 0, 0, 0, 8, 8, 9, 8, 9],
                vec![6, 8, 0, 0, 0, 0, 5, 9, 4, 3],
                vec![0, 0, 0, 0, 0, 0, 7, 4, 5, 6],
                vec![9, 0, 0, 0, 0, 0, 0, 8, 7, 6],
                vec![8, 7, 0, 0, 0, 0, 6, 8, 4, 8],
            ]
        );

        grid = take_step(grid);
        assert_eq!(
            grid,
            vec![
                vec![0, 0, 5, 0, 9, 0, 0, 8, 6, 6],
                vec![8, 5, 0, 0, 8, 0, 0, 5, 7, 5],
                vec![9, 9, 0, 0, 0, 0, 0, 0, 3, 9],
                vec![9, 7, 0, 0, 0, 0, 0, 0, 4, 1],
                vec![9, 9, 3, 5, 0, 8, 0, 0, 6, 3],
                vec![7, 7, 1, 2, 3, 0, 0, 0, 0, 0],
                vec![7, 9, 1, 1, 2, 5, 0, 0, 0, 9],
                vec![2, 2, 1, 1, 1, 3, 0, 0, 0, 0],
                vec![0, 4, 2, 1, 1, 2, 5, 0, 0, 0],
                vec![0, 0, 2, 1, 1, 1, 9, 0, 0, 0],
            ]
        );

        grid = take_step(grid);
        assert_eq!(
            grid,
            vec![
                vec![2, 2, 6, 3, 0, 3, 1, 9, 7, 7],
                vec![0, 9, 2, 3, 0, 3, 1, 6, 9, 7],
                vec![0, 0, 3, 2, 2, 2, 1, 1, 5, 0],
                vec![0, 0, 4, 1, 1, 1, 1, 1, 6, 3],
                vec![0, 0, 7, 6, 1, 9, 1, 1, 7, 4],
                vec![0, 0, 5, 3, 4, 1, 1, 1, 2, 2],
                vec![0, 0, 4, 2, 3, 6, 1, 1, 2, 0],
                vec![5, 5, 3, 2, 2, 4, 1, 1, 2, 2],
                vec![1, 5, 3, 2, 2, 4, 7, 2, 1, 1],
                vec![1, 1, 3, 2, 2, 3, 0, 2, 1, 1],
            ]
        );
    }
}
