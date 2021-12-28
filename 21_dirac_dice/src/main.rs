use itertools::Itertools;
use std::io::{BufRead, BufReader};
use std::iter::successors;
use std::ops::Neg;

#[derive(Clone, Debug)]
struct Player {
    score: i64,
    position: i64,
}

impl Player {
    fn new(position: usize) -> Self {
        Player {
            score: 0,
            position: position.try_into().unwrap(),
        }
    }

    fn move_forward(&mut self, move_forward_by: i64) -> i64 {
        self.position = (self.position + move_forward_by) % 10;
        self.score += if self.position == 0 {
            10
        } else {
            self.position
        };

        self.score
    }
}

fn deterministic_dice_game(players: [Player; 2], goal: i64) -> ([i64; 2], i64) {
    let dice_rolls_chunks = successors(Some(1_i64), |i| i.checked_add(1)).chunks(3);
    let mut game = players;

    let mut dice_rolls = dice_rolls_chunks.into_iter().map(|c| c.sum::<i64>());

    for turn in 0.. {
        for (pi, player) in game.iter_mut().enumerate() {
            let score = player.move_forward(dice_rolls.next().unwrap());
            if score >= goal {
                return (
                    [player.score, game[1 - pi].score],
                    ((turn * 2 + (pi + 1)) * 3).try_into().unwrap(),
                );
            }
        }
    }
    unreachable!()
}

const QUANTUM_DIE_THREE_ROLLS_MOVE_PROB: [i64; 7] = [1, 3, 6, 7, 6, 3, 1];

fn play_all_quantum_die_games(players: [Player; 2], goal: i64) -> [i64; 2] {
    let mut result = [0, 0];
    for (index, num_games_with_dice_total) in QUANTUM_DIE_THREE_ROLLS_MOVE_PROB.iter().enumerate() {
        let mut current_player = players[0].clone();

        let dice_total = index + 3;
        let score = current_player.move_forward(dice_total.try_into().unwrap());
        if score >= goal {
            result[0] += num_games_with_dice_total;
        } else {
            let next_turn_result =
                play_all_quantum_die_games([players[1].clone(), current_player], goal);
            result[0] += next_turn_result[1] * num_games_with_dice_total;
            result[1] += next_turn_result[0] * num_games_with_dice_total;
        }
    }
    result
}

fn main() {
    if let Some((player_1, player_2)) = BufReader::new(std::io::stdin())
        .lines()
        .filter_map(Result::ok)
        .filter_map(|l| {
            l.splitn(5, " ")
                .last()
                .and_then(|v| v.parse::<usize>().ok())
        })
        .collect_tuple()
    {
        let (scores, dice_rolls) =
            deterministic_dice_game([Player::new(player_1), Player::new(player_2)], 1000);
        let [winner_score, loser_score] = scores;

        println!(
            "Winner with score {} after {} dice rolls with the score of the losing player {}",
            winner_score, dice_rolls, loser_score
        );
        println!("> multiply the score of the losing player by the number of times the die was rolled during the game: {}", dice_rolls * loser_score);

        let scores = play_all_quantum_die_games([Player::new(player_1), Player::new(player_2)], 21);
        if let Some((most_wins, least_wins)) = scores
            .into_iter()
            .enumerate()
            .map(|(i, s)| (i + 1, s))
            .sorted_by_key(|v| v.1.neg())
            .collect_tuple()
        {
            println!(
                "Player {} won the most games:  winning {} games",
                most_wins.0, most_wins.1
            );
            println!(
                "Player {} won the least games: winning {} games",
                least_wins.0, least_wins.1
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_deterministic_dice_game() {
        let result = deterministic_dice_game([Player::new(4), Player::new(8)], 1000);

        assert_eq!(result, ([1000, 745], 993))
    }

    #[test]
    fn check_play_all_quantum_die_games() {
        let result = play_all_quantum_die_games([Player::new(4), Player::new(8)], 21);

        assert_eq!(result, [444356092776315, 341960390180808])
    }
}
