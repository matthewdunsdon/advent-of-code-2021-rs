use bitvec::prelude::*;

use std::{
    fmt::Display,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq)]
struct Image {
    width: i64,
    height: i64,
    edge_value: bool,
    rows: Vec<BitVec>,
}

impl Image {
    fn new(rows: Vec<BitVec>, edge_value: bool) -> Self {
        Image {
            width: i64::try_from(rows[0].len()).unwrap(),
            height: i64::try_from(rows.len()).unwrap(),
            edge_value,
            rows,
        }
    }

    fn true_value_count(&self) -> usize {
        self.rows.iter().map(|row| row.count_ones()).sum()
    }

    fn get_cell(&self, row: i64, cell: i64, otherwise: usize) -> usize {
        if row < 0 || cell < 0 || row >= self.height as i64 || cell >= self.width as i64 {
            otherwise
        } else {
            if self.rows[row as usize][cell as usize] {
                1
            } else {
                0
            }
        }
    }

    fn generate_next_image(&self, image_enhancement_algorithm: &BitVec) -> Image {
        let next_height = self.height + 2;
        let next_width = self.width + 2;
        let edge_value = if self.edge_value { 1 } else { 0 };
        let invert_edge = image_enhancement_algorithm[0];

        let next_image = (0..next_height)
            .map(|row_index| {
                let mut row = BitVec::with_capacity(usize::try_from(self.width).unwrap());
                for cell_index in 0..next_width {
                    let cells = [
                        self.get_cell(row_index - 2, cell_index - 2, edge_value),
                        self.get_cell(row_index - 2, cell_index - 1, edge_value),
                        self.get_cell(row_index - 2, cell_index + 0, edge_value),
                        self.get_cell(row_index - 1, cell_index - 2, edge_value),
                        self.get_cell(row_index - 1, cell_index - 1, edge_value),
                        self.get_cell(row_index - 1, cell_index + 0, edge_value),
                        self.get_cell(row_index + 0, cell_index - 2, edge_value),
                        self.get_cell(row_index + 0, cell_index - 1, edge_value),
                        self.get_cell(row_index + 0, cell_index + 0, edge_value),
                    ];
                    let score = cells.into_iter().fold(0, |v, n| (v << 1) + n);
                    row.push(image_enhancement_algorithm[score]);
                }
                row
            })
            .collect();

        Image::new(
            next_image,
            if invert_edge { !self.edge_value } else { false },
        )
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter() {
            writeln!(f, "{}", row)?;
        }
        Ok(())
    }
}

fn to_bit_vec(line: String) -> BitVec<LocalBits, usize> {
    let mut result = BitVec::with_capacity(line.len());
    for b in line.bytes() {
        result.push(b == b'#')
    }
    result
}

fn main() -> Result<(), &'static str> {
    let mut lines = BufReader::new(std::io::stdin())
        .lines()
        .map(|r| r.map_err(|_| "Can't read line"));

    let image_enhancement_algorithm = to_bit_vec(lines.next().unwrap()?);

    lines.next();

    let mut image = Image::new(
        lines
            .filter_map(Result::ok)
            .map(to_bit_vec)
            .collect::<Vec<_>>(),
        false,
    );

    println!("{}", image);
    println!("Initial count: {}", image.true_value_count());

    for i in 1..=2 {
        image = image.generate_next_image(&image_enhancement_algorithm);
        println!("After {} count: {}", i, image.true_value_count());
    }
    let goal = 50;
    for _ in 3..=goal {
        image = image.generate_next_image(&image_enhancement_algorithm);
    }

    println!("After {} count: {}", goal, image.true_value_count());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_all_rotations_of_point_in_scanner_reading() {
        let image_enhancement_algorithm = to_bit_vec("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#".to_string());

        let mut image = Image::new(
            vec![
                to_bit_vec("#..#.".to_string()),
                to_bit_vec("#....".to_string()),
                to_bit_vec("##..#".to_string()),
                to_bit_vec("..#..".to_string()),
                to_bit_vec("..###".to_string()),
            ],
            false,
        );

        image = image.generate_next_image(&image_enhancement_algorithm);
        assert_eq!(
            image,
            Image::new(
                vec![
                    to_bit_vec(".##.##.".to_string()),
                    to_bit_vec("#..#.#.".to_string()),
                    to_bit_vec("##.#..#".to_string()),
                    to_bit_vec("####..#".to_string()),
                    to_bit_vec(".#..##.".to_string()),
                    to_bit_vec("..##..#".to_string()),
                    to_bit_vec("...#.#.".to_string()),
                ],
                false
            )
        );
        image = image.generate_next_image(&image_enhancement_algorithm);
        assert_eq!(
            image,
            Image::new(
                vec![
                    to_bit_vec(".......#.".to_string()),
                    to_bit_vec(".#..#.#..".to_string()),
                    to_bit_vec("#.#...###".to_string()),
                    to_bit_vec("#...##.#.".to_string()),
                    to_bit_vec("#.....#.#".to_string()),
                    to_bit_vec(".#.#####.".to_string()),
                    to_bit_vec("..#.#####".to_string()),
                    to_bit_vec("...##.##.".to_string()),
                    to_bit_vec("....###..".to_string()),
                ],
                false
            )
        );
    }
}
