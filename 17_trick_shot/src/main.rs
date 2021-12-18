use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i64, i64);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Velocity(i64, i64);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct ShotTarget {
    start: Pos,
    end: Pos,
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct TraceResult {
    initial_velocity: Velocity,
    positions: Vec<Pos>,
    hit: bool,
}

fn compare_to_target(pos: &Pos, shot_target: &ShotTarget) -> Ordering {
    if pos.0 > shot_target.end.0 || pos.1 < shot_target.end.1 {
        Ordering::Greater
    } else if pos.0 >= shot_target.start.0 && pos.1 <= shot_target.start.1 {
        Ordering::Equal
    } else {
        Ordering::Less
    }
}

fn take_move(pos: &Pos, velocity: &Velocity) -> (Pos, Velocity) {
    (
        Pos(pos.0 + velocity.0, pos.1 + velocity.1),
        Velocity(velocity.0 - velocity.0.signum(), velocity.1 - 1),
    )
}

fn trace_velocity(initial_velocity: Velocity, shot_target: &ShotTarget) -> TraceResult {
    let mut state = initial_velocity.clone();
    let mut positions = vec![Pos(0, 0)];
    let mut comp = Ordering::Less;
    while comp == Ordering::Less {
        let after_move = take_move(positions.last().unwrap(), &state);
        comp = compare_to_target(&after_move.0, shot_target);
        positions.push(after_move.0);
        state = after_move.1;
    }
    TraceResult {
        initial_velocity,
        positions,
        hit: comp == Ordering::Equal,
    }
}

fn search_velocities(shot_target: &ShotTarget) -> (i64, usize) {
    let results = (0..=600)
        .into_iter()
        .flat_map(|x| {
            (-900..=900)
                .into_iter()
                .map(|y| trace_velocity(Velocity(x, y), shot_target))
                .collect::<Vec<TraceResult>>()
        })
        .collect::<Vec<TraceResult>>();

    (
        results
            .iter()
            .filter(|t| t.hit)
            .flat_map(|t| t.positions.iter().map(|p| p.1).max())
            .max()
            .unwrap_or(0),
        results.iter().filter(|t| t.hit).count(),
    )
}

fn main() {
    let shot_target = ShotTarget {
        start: Pos(195, -67),
        end: Pos(238, -93),
    };
    let (max_y, hit_count) = search_velocities(&shot_target);
    println!("For: {:?}", shot_target);
    println!("Max y: {}", max_y);
    println!("Hit count: {}", hit_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn check_search_velocities() {
        let (max_y, hit_count) = search_velocities(&ShotTarget {
            start: Pos(20, -5),
            end: Pos(30, -10),
        });
        assert_eq!(max_y, 45);
        assert_eq!(hit_count, 112);
    }
}
