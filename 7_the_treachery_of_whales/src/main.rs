use std::io::{BufRead, BufReader};
use std::iter::Sum;

type Position = i16;

#[derive(Debug, PartialEq, Eq)]
struct Score {
    unit: i32,
    triangular: i32,
}

impl Sum for Score {
    fn sum<I: Iterator<Item = Score>>(iter: I) -> Score {
        let mut total_unit = 0_i32;
        let mut total_triangular = 0_i32;
        for entry in iter {
            total_unit += entry.unit;
            total_triangular += entry.triangular;
        }
        Score {
            unit: total_unit,
            triangular: total_triangular,
        }
    }
}

fn get_positions(s: String) -> Vec<Position> {
    s.split(",").filter_map(|z| z.parse::<i16>().ok()).collect()
}

fn score_distance(p1: &Position, p2: &Position) -> Score {
    let unit = i32::from((p1 - p2).abs());
    let triangular = unit * (unit + 1) / 2;
    Score { unit, triangular }
}

fn get_distance(positions: &Vec<Position>, at: &Position) -> Score {
    positions.iter().map(|p| score_distance(p, at)).sum()
}

fn task() -> Result<(), String> {
    let positions: Vec<Position> = BufReader::new(std::io::stdin())
        .lines()
        .filter_map(Result::ok)
        .flat_map(get_positions)
        .collect();

    let min_range = positions.iter().min().ok_or("No min")?.to_owned();
    let max_range = positions.iter().max().ok_or("No max")?.to_owned();

    let distances: Vec<Score> = (min_range..=max_range)
        .into_iter()
        .map(|i| get_distance(&positions, &i))
        .collect();

    let min_unit_distance = distances
        .iter()
        .map(|s| s.unit)
        .min()
        .ok_or("No min unit")?
        .to_owned();

    let min_triangular_distance = distances
        .iter()
        .map(|s| s.triangular)
        .min()
        .ok_or("No min triangular")?
        .to_owned();

    println!("Min unit distance: {}", min_unit_distance);
    println!("Min triangular distance: {}", min_triangular_distance);

    Ok(())
}

fn main() {
    if let Err(err) = task() {
        println!("[App failed] ERROR: {}", err);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_get_positions() {
        assert_eq!(
            get_positions("16,1,2,0,4,2,7,1,2,14".to_owned()),
            vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]
        );
    }

    #[test]
    fn check_get_distance() {
        let positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        assert_eq!(
            get_distance(&positions, &0),
            Score {
                unit: 49,
                triangular: 290
            }
        );
        assert_eq!(
            get_distance(&positions, &1),
            Score {
                unit: 41,
                triangular: 242
            }
        );
        assert_eq!(
            get_distance(&positions, &2),
            Score {
                unit: 37,
                triangular: 206
            }
        );
        assert_eq!(
            get_distance(&positions, &3),
            Score {
                unit: 39,
                triangular: 183
            }
        );
        assert_eq!(
            get_distance(&positions, &4),
            Score {
                unit: 41,
                triangular: 170
            }
        );
        assert_eq!(
            get_distance(&positions, &5),
            Score {
                unit: 45,
                triangular: 168
            }
        );
        assert_eq!(
            get_distance(&positions, &6),
            Score {
                unit: 49,
                triangular: 176
            }
        );
        assert_eq!(
            get_distance(&positions, &7),
            Score {
                unit: 53,
                triangular: 194
            }
        );
        assert_eq!(
            get_distance(&positions, &8),
            Score {
                unit: 59,
                triangular: 223
            }
        );
        assert_eq!(
            get_distance(&positions, &9),
            Score {
                unit: 65,
                triangular: 262
            }
        );
        assert_eq!(
            get_distance(&positions, &10),
            Score {
                unit: 71,
                triangular: 311
            }
        );
        assert_eq!(
            get_distance(&positions, &11),
            Score {
                unit: 77,
                triangular: 370
            }
        );
        assert_eq!(
            get_distance(&positions, &12),
            Score {
                unit: 83,
                triangular: 439
            }
        );
        assert_eq!(
            get_distance(&positions, &13),
            Score {
                unit: 89,
                triangular: 518
            }
        );
        assert_eq!(
            get_distance(&positions, &14),
            Score {
                unit: 95,
                triangular: 607
            }
        );
        assert_eq!(
            get_distance(&positions, &15),
            Score {
                unit: 103,
                triangular: 707
            }
        );
        assert_eq!(
            get_distance(&positions, &16),
            Score {
                unit: 111,
                triangular: 817
            }
        );
    }
}
