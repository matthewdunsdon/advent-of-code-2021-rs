use std::collections::HashMap;
use std::io::{BufRead, BufReader};

struct BingoBoard {
    id: usize,
    mapping: HashMap<i16, (usize, usize)>,
    score: i16,
    columns: [i8; 5],
    rows: [i8; 5],
    won: bool,
}

impl BingoBoard {
    fn new(id: usize, cells: [i16; 25]) -> BingoBoard {
        let mut score = 0i16;
        let mut mapping = HashMap::new();
        for (index, cell) in cells.iter().enumerate() {
            score = score + cell;
            mapping.insert(cell.to_owned(), (index / 5, index % 5));
        }
        BingoBoard {
            id,
            columns: [5; 5],
            mapping,
            rows: [5; 5],
            score,
            won: false,
        }
    }

    fn number_drawn(&mut self, value: i16) -> Option<i32> {
        if self.won {
            None
        } else if let Some((x, y)) = self.mapping.get(&value) {
            self.score -= value;
            self.columns[*x] -= 1;
            self.rows[*y] -= 1;
            self.won = (self.columns[*x] == 0) || (self.rows[*y] == 0);

            Some(i32::from(self.score) * i32::from(value))
        } else {
            Some(i32::from(self.score) * i32::from(value))
        }
    }
}

enum GameDefinition {
    Initial,
    WithNumbersDrawn(Vec<i16>),
    WithGameboardCells(Vec<i16>, Vec<i16>),
}

fn build_game_definition(game_definition: GameDefinition, line: String) -> GameDefinition {
    match game_definition {
        GameDefinition::Initial => {
            let numbers_drawn = line
                .split(",")
                .filter_map(|z| z.parse::<i16>().ok())
                .collect();
            GameDefinition::WithNumbersDrawn(numbers_drawn)
        }
        GameDefinition::WithNumbersDrawn(n) => {
            let cells = line
                .split(" ")
                .filter_map(|i| i.parse::<i16>().ok())
                .collect();
            GameDefinition::WithGameboardCells(n, cells)
        }
        GameDefinition::WithGameboardCells(n, mut c) => {
            c.extend(line.split(" ").filter_map(|i| i.parse::<i16>().ok()));
            GameDefinition::WithGameboardCells(n, c)
        }
    }
}

fn run_board(board: BingoBoard, numbers_drawn: &Vec<i16>) -> (usize, usize, i32) {
    let id = board.id;
    let scores: Vec<i32> = numbers_drawn
        .iter()
        .scan(board, |b, number_drawn| b.number_drawn(*number_drawn))
        .collect();

    scores
        .last()
        .map(|s| (id, scores.len(), s.to_owned()))
        .unwrap()
}

fn main() {
    let game_definition = BufReader::new(std::io::stdin())
        .lines()
        .filter_map(Result::ok)
        .fold(GameDefinition::Initial, build_game_definition);

    match game_definition {
        GameDefinition::WithGameboardCells(numbers_drawn, cells) => {
            let boards: Vec<BingoBoard> = cells
                .chunks_exact(25)
                .enumerate()
                .map(|(i, x)| {
                    let mut cells = [0; 25];
                    for index in 0..25 {
                        cells[index] = x[index];
                    }
                    BingoBoard::new(i, cells)
                })
                .collect();

            let mut results: Vec<(usize, usize, i32)> = boards
                .into_iter()
                .map(|b| run_board(b, &numbers_drawn))
                .collect();

            results.sort_by(|(_, a, _), (_, b, _)| a.partial_cmp(b).unwrap());

            if let Some((id, moves, score)) = results.first() {
                println!(
                    "Game no.{} completed the first in {} moves, with score {}",
                    id, moves, score
                );
            }
            if let Some((id, moves, score)) = results.last() {
                println!(
                    "Game no.{} completed the last in {} moves, with score {}",
                    id, moves, score
                );
            }
        }
        _ => {
            println!("Could not parse input");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_build_game_definition_for_initial_state() {
        let result = build_game_definition(GameDefinition::Initial, "7,4,9,5,11".to_owned());
        let expected = vec![7, 4, 9, 5, 11];

        match result {
            GameDefinition::WithNumbersDrawn(numbers_drawn) => {
                assert_eq!(numbers_drawn, expected);
            }
            _ => {
                assert!(false, "Invalid Game Definition state");
            }
        }
    }

    #[test]
    fn check_build_game_definition_for_with_numbers_drawn_state() {
        let numbers_drawn = vec![7, 4, 9, 5, 11];
        let result = build_game_definition(
            GameDefinition::WithNumbersDrawn(numbers_drawn.clone()),
            "2 14 19 25 11".to_owned(),
        );

        match result {
            GameDefinition::WithGameboardCells(n, c) => {
                assert_eq!(n, numbers_drawn);
                assert_eq!(c, vec![2, 14, 19, 25, 11]);
            }
            _ => {
                assert!(false, "Invalid Game Definition state");
            }
        }
    }

    #[test]
    fn check_build_game_definition_for_with_gameboard_cells_state() {
        let numbers_drawn = vec![7, 4, 9, 5, 11];
        let cells = vec![2, 14, 19, 25, 11];
        let result = build_game_definition(
            GameDefinition::WithGameboardCells(numbers_drawn.clone(), cells),
            "18 8 23 26 20".to_owned(),
        );

        match result {
            GameDefinition::WithGameboardCells(n, c) => {
                assert_eq!(n, numbers_drawn);
                assert_eq!(c, vec![2, 14, 19, 25, 11, 18, 8, 23, 26, 20]);
            }
            _ => {
                assert!(false, "Invalid Game Definition state");
            }
        }
    }

    #[test]
    fn check_board_score() {
        let mut boards = get_simple_case();
        assert_eq!(
            boards.iter().map(|a| a.score).collect::<Vec<i16>>(),
            vec!(300, 324, 325)
        );

        boards.iter_mut().for_each(|b| {
            b.number_drawn(7);
        });
        assert_eq!(
            boards.iter().map(|a| a.score).collect::<Vec<i16>>(),
            vec!(293, 317, 318)
        );

        boards.iter_mut().for_each(|b| {
            b.number_drawn(1);
        });
        assert_eq!(
            boards.iter().map(|a| a.score).collect::<Vec<i16>>(),
            vec!(292, 317, 318)
        );
    }

    #[test]
    fn check_board_won_for_horizontal() {
        let [board, _, _] = get_simple_case();
        let (id, steps, score) = run_board(board, &vec![8_i16, 2, 23, 4, 24]);

        assert_eq!(id, 0);
        assert_eq!(steps, 5);
        assert_eq!(score, 24 * 239);
    }

    #[test]
    fn check_board_won_for_vertical() {
        let [_, board, _] = get_simple_case();
        let (id, steps, score) = run_board(board, &vec![15_i16, 18, 100, 8, 11, 21]);

        assert_eq!(id, 1);
        assert_eq!(steps, 6);
        assert_eq!(score, 21 * 251);
    }

    fn get_simple_case() -> [BingoBoard; 3] {
        [
            BingoBoard::new(
                0,
                [
                    22, 13, 17, 11, 0, // |
                    8, 2, 23, 4, 24, // |
                    21, 9, 14, 16, 7, // |
                    6, 10, 3, 18, 5, // |
                    1, 12, 20, 15, 19, // |
                ],
            ),
            BingoBoard::new(
                1,
                [
                    3, 15, 0, 2, 22, // |
                    9, 18, 13, 17, 5, // |
                    19, 8, 7, 25, 23, // |
                    20, 11, 10, 24, 4, // |
                    14, 21, 16, 12, 6, // |
                ],
            ),
            BingoBoard::new(
                2,
                [
                    14, 21, 17, 24, 4, // |
                    10, 16, 15, 9, 19, // |
                    18, 8, 23, 26, 20, // |
                    22, 11, 13, 6, 5, // |
                    2, 0, 12, 3, 7, // |
                ],
            ),
        ]
    }
}
