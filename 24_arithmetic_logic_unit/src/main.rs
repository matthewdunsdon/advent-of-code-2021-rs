use std::{
    collections::HashMap,
    ops::{Add, Div, Mul, Rem},
};

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Keep(i64, i64),
    Reduce(i64, i64),
}

impl Instruction {
    fn update_state(&self, state: i64) -> i64 {
        match self {
            Instruction::Keep(_, _) => state,
            Instruction::Reduce(_, _) => state.div(26),
        }
    }

    fn val_a(&self) -> i64 {
        match self {
            Instruction::Keep(val, _) => *val,
            Instruction::Reduce(val, _) => *val,
        }
    }

    fn val_b(&self) -> i64 {
        match self {
            Instruction::Keep(_, val) => *val,
            Instruction::Reduce(_, val) => *val,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Solver {
    instructions: [Instruction; 14],
    cache: HashMap<(usize, i64), Vec<i64>>,
}

impl Solver {
    fn solve(&mut self, ndigit: usize, prev_z: i64) -> Vec<i64> {
        if ndigit >= 14 {
            if prev_z == 0 {
                vec![0]
            } else {
                vec![]
            }
        } else if let Some(cached) = self.cache.get(&(ndigit, prev_z)) {
            cached.clone()
        } else {
            let matches: Vec<i64> = (1..=9)
                .into_iter()
                .flat_map(|input_guess| {
                    let next_z = evaluate(prev_z, input_guess, &self.instructions[ndigit]);
                    self.solve(ndigit + 1, next_z)
                        .into_iter()
                        .map(move |best_suffix| {
                            let exp = 14 - ndigit - 1;
                            let new_suffix = 10_i64.pow(exp as u32) * input_guess + best_suffix;
                            new_suffix
                        })
                })
                .collect();

            self.cache.insert((ndigit, prev_z), matches.clone());
            matches
        }
    }
}

fn main() {
    let mut solver = Solver {
        instructions: [
            Instruction::Keep(12, 9),
            Instruction::Keep(12, 4),
            Instruction::Keep(12, 2),
            Instruction::Reduce(-9, 5),
            Instruction::Reduce(-9, 1),
            Instruction::Keep(14, 6),
            Instruction::Keep(14, 11),
            Instruction::Reduce(-10, 15),
            Instruction::Keep(15, 7),
            Instruction::Reduce(-2, 12),
            Instruction::Keep(11, 15),
            Instruction::Reduce(-15, 9),
            Instruction::Reduce(-9, 12),
            Instruction::Reduce(-3, 12),
        ],
        cache: HashMap::new(),
    };
    let result = solver.solve(0, 0);

    println!("Smallest: {:?}", result.first());
    println!("Largest: {:?}", result.last());
    println!("Total matches: {:?}", result.len());
}

fn evaluate(state: i64, input: i64, instruction: &Instruction) -> i64 {
    if state.rem(26).add(instruction.val_a()) != input {
        instruction
            .update_state(state)
            .mul(26)
            .add(input)
            .add(instruction.val_b())
    } else {
        instruction.update_state(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_state_for_high_value() {
        let instructions = [
            Instruction::Keep(12, 9),
            Instruction::Keep(12, 4),
            Instruction::Keep(12, 2),
            Instruction::Reduce(-9, 5),
            Instruction::Reduce(-9, 1),
            Instruction::Keep(14, 6),
            Instruction::Keep(14, 11),
            Instruction::Reduce(-10, 15),
            Instruction::Keep(15, 7),
            Instruction::Reduce(-2, 12),
            Instruction::Keep(11, 15),
            Instruction::Reduce(-15, 9),
            Instruction::Reduce(-9, 12),
            Instruction::Reduce(-3, 12),
        ];
        let input_values = [3_i64, 9, 9, 2, 4, 9, 8, 9, 4, 9, 9, 9, 6, 9];

        let state_values: Vec<_> = instructions
            .iter()
            .zip(input_values)
            .scan(0, |state, (instruction, input_value)| {
                *state = evaluate(*state, input_value, instruction);
                Some(*state)
            })
            .collect();

        assert_eq!(
            state_values,
            vec![12, 325, 8461, 325, 12, 327, 8521, 327, 8513, 327, 8526, 327, 12, 0]
        );
    }
    #[test]
    fn check_state_for_low_value() {
        let instructions = [
            Instruction::Keep(12, 9),
            Instruction::Keep(12, 4),
            Instruction::Keep(12, 2),
            Instruction::Reduce(-9, 5),
            Instruction::Reduce(-9, 1),
            Instruction::Keep(14, 6),
            Instruction::Keep(14, 11),
            Instruction::Reduce(-10, 15),
            Instruction::Keep(15, 7),
            Instruction::Reduce(-2, 12),
            Instruction::Keep(11, 15),
            Instruction::Reduce(-15, 9),
            Instruction::Reduce(-9, 12),
            Instruction::Reduce(-3, 12),
        ];
        let input_values = [1_i64, 6, 8, 1, 1, 4, 1, 2, 1, 6, 1, 1, 1, 7];
        let state_values: Vec<_> = instructions
            .iter()
            .zip(input_values)
            .scan(0, |state, (instruction, input_value)| {
                *state = evaluate(*state, input_value, instruction);
                Some(*state)
            })
            .collect();

        assert_eq!(
            state_values,
            vec![10, 270, 7030, 270, 10, 270, 7032, 270, 7028, 270, 7036, 270, 10, 0]
        );
    }
}
