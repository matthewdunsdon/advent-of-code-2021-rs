mod burrow;

use std::io::{BufRead, BufReader};

use crate::burrow::Burrow;
use pathfinding::prelude::astar;

fn get_cost_to_solve(burrow: Burrow) {
    let result = astar(
        &burrow,
        |b| b.successors().collect::<Vec<_>>(),
        |b| b.estimated_cost(),
        |b| b.estimated_cost() == 0,
    );
    if let Some((route, cost)) = result {
        println!("Completed in {} steps with cost {}", route.len(), cost);
    } else {
        println!(
            "No solution found.

[HINT] Did you set up the input correctly?"
        );
    }
}

fn main() -> Result<(), &'static str> {
    let lines = BufReader::new(std::io::stdin())
        .lines()
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    let mut initial_lines = lines.clone();
    initial_lines.insert(4, "  #A#B#C#D#".to_string());
    initial_lines.insert(5, "  #A#B#C#D#".to_string());

    get_cost_to_solve(initial_lines.join("\n").parse()?);

    let mut unfolded_lines = lines;
    unfolded_lines.insert(3, "  #D#C#B#A#".to_string());
    unfolded_lines.insert(4, "  #D#B#A#C#".to_string());

    get_cost_to_solve(unfolded_lines.join("\n").parse()?);

    Ok(())
}
