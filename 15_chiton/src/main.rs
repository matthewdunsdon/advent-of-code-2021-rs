use pathfinding::prelude::dijkstra;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

fn cost_large_map_edge(p: Pos, lines: &Vec<Vec<usize>>) -> (Pos, usize) {
    let quot_x = p.0 / lines.len();
    let quot_y = p.1 / lines[0].len();
    let rem_x = p.0 % lines.len();
    let rem_y = p.1 % lines[0].len();

    let cost = lines[rem_x][rem_y] + quot_x + quot_y;
    if cost > 9 {
        (p, cost - 9)
    } else {
        (p, cost)
    }
}

fn cost_edge(p: Pos, lines: &Vec<Vec<usize>>) -> (Pos, usize) {
    let cost = lines[p.0][p.1];
    (p, cost)
}

fn successors<FN>(p: &Pos, edge: &Pos, cost: FN) -> Vec<(Pos, usize)>
where
    FN: Fn(Pos) -> (Pos, usize),
{
    let mut successors = Vec::default();
    if p.0 < edge.0 {
        successors.push(Pos(p.0 + 1, p.1))
    }
    if p.1 < edge.1 {
        successors.push(Pos(p.0, p.1 + 1))
    }
    if p.0 > 0 {
        successors.push(Pos(p.0 - 1, p.1))
    }
    if p.1 > 0 {
        successors.push(Pos(p.0, p.1 - 1))
    }
    successors.into_iter().map(|p| cost(p)).collect()
}

fn main() {
    let lines: Vec<Vec<usize>> = BufReader::new(std::io::stdin())
        .lines()
        .filter_map(|f| f.ok())
        .map(|f| {
            f.chars()
                .into_iter()
                .filter_map(|c| c.to_digit(10).and_then(|z| usize::try_from(z).ok()))
                .collect()
        })
        .collect();

    let (width, height) = (lines.len(), lines[0].len());

    let goal: Pos = Pos(width - 1, height - 1);
    let shortest_path = dijkstra(
        &Pos(0, 0),
        |p| successors(&p, &goal, |po| cost_edge(po, &lines)),
        |p| *p == goal,
    );

    if let Some(result) = shortest_path {
        println!("Lowest total risk: {}", result.1);
    }

    let goal: Pos = Pos(width * 5 - 1, height * 5 - 1);
    let shortest_path = dijkstra(
        &Pos(0, 0),
        |p| successors(&p, &goal, |po| cost_large_map_edge(po, &lines)),
        |p| *p == goal,
    );

    if let Some(result) = shortest_path {
        println!("Lowest total risk for bigger map: {}", result.1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_risk_levels() -> Vec<Vec<usize>> {
        vec![
            vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            vec![1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            vec![2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            vec![3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            vec![7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            vec![1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            vec![1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            vec![3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            vec![1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            vec![2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ]
    }

    #[test]

    fn check_cost_edge() {
        let risk_levels = sample_risk_levels();
        assert_eq!(cost_edge(Pos(0, 0), &risk_levels), (Pos(0, 0), 1));
        assert_eq!(cost_edge(Pos(1, 2), &risk_levels), (Pos(1, 2), 8));
        assert_eq!(cost_edge(Pos(5, 5), &risk_levels), (Pos(5, 5), 2));
        assert_eq!(cost_edge(Pos(9, 8), &risk_levels), (Pos(9, 8), 8));
        assert_eq!(cost_edge(Pos(9, 9), &risk_levels), (Pos(9, 9), 1));
    }

    #[test]

    fn check_cost_large_map_edge() {
        let risk_levels = sample_risk_levels();
        assert_eq!(cost_large_map_edge(Pos(0, 0), &risk_levels), (Pos(0, 0), 1));
        assert_eq!(
            cost_large_map_edge(Pos(11, 2), &risk_levels),
            (Pos(11, 2), 8 + 1)
        );
        assert_eq!(
            cost_large_map_edge(Pos(5, 45), &risk_levels),
            (Pos(5, 45), 2 + 4)
        );
        assert_eq!(
            cost_large_map_edge(Pos(49, 48), &risk_levels),
            (Pos(49, 48), (8 + 8) - 9)
        );
        assert_eq!(
            cost_large_map_edge(Pos(49, 49), &risk_levels),
            (Pos(49, 49), 1 + 8)
        );
    }

    #[test]

    fn check_successors() {
        let edge = Pos(9, 9);
        let cost = |po: Pos| (po, 1);

        assert_eq!(
            successors(&Pos(0, 0), &edge, cost),
            vec![(Pos(1, 0), 1), (Pos(0, 1), 1)]
        );
        assert_eq!(
            successors(&Pos(2, 2), &edge, cost),
            vec![
                (Pos(3, 2), 1),
                (Pos(2, 3), 1),
                (Pos(1, 2), 1),
                (Pos(2, 1), 1)
            ]
        );
        assert_eq!(
            successors(&Pos(0, 9), &edge, cost),
            vec![(Pos(1, 9), 1), (Pos(0, 8), 1)]
        );
        assert_eq!(
            successors(&Pos(9, 0), &edge, cost),
            vec![(Pos(9, 1), 1), (Pos(8, 0), 1)]
        );
        assert_eq!(
            successors(&Pos(9, 9), &edge, cost),
            vec![(Pos(8, 9), 1), (Pos(9, 8), 1)]
        );
    }

    #[test]

    fn check_shortest_paths() {
        let risk_levels = sample_risk_levels();
        let (width, height) = (risk_levels.len(), risk_levels[0].len());

        let goal: Pos = Pos(width - 1, height - 1);
        let shortest_path = dijkstra(
            &Pos(0, 0),
            |p| successors(&p, &goal, |po| cost_edge(po, &risk_levels)),
            |p| *p == goal,
        );

        assert_eq!(
            shortest_path,
            Some((
                vec![
                    Pos(0, 0),
                    Pos(1, 0),
                    Pos(2, 0),
                    Pos(2, 1),
                    Pos(2, 2),
                    Pos(2, 3),
                    Pos(2, 4),
                    Pos(2, 5),
                    Pos(2, 6),
                    Pos(3, 6),
                    Pos(3, 7),
                    Pos(4, 7),
                    Pos(5, 7),
                    Pos(5, 8),
                    Pos(6, 8),
                    Pos(7, 8),
                    Pos(8, 8),
                    Pos(8, 9),
                    Pos(9, 9)
                ],
                40
            ))
        );

        let goal: Pos = Pos(width * 5 - 1, height * 5 - 1);
        let shortest_path = dijkstra(
            &Pos(0, 0),
            |p| successors(&p, &goal, |po| cost_large_map_edge(po, &risk_levels)),
            |p| *p == goal,
        );

        assert_eq!(
            shortest_path,
            Some((
                vec![
                    Pos(0, 0),
                    Pos(1, 0),
                    Pos(2, 0),
                    Pos(3, 0),
                    Pos(4, 0),
                    Pos(5, 0),
                    Pos(6, 0),
                    Pos(7, 0),
                    Pos(8, 0),
                    Pos(9, 0),
                    Pos(10, 0),
                    Pos(11, 0),
                    Pos(12, 0),
                    Pos(12, 1),
                    Pos(12, 2),
                    Pos(13, 2),
                    Pos(14, 2),
                    Pos(15, 2),
                    Pos(15, 3),
                    Pos(16, 3),
                    Pos(16, 4),
                    Pos(16, 5),
                    Pos(16, 6),
                    Pos(16, 7),
                    Pos(16, 8),
                    Pos(16, 9),
                    Pos(17, 9),
                    Pos(18, 9),
                    Pos(18, 10),
                    Pos(18, 11),
                    Pos(18, 12),
                    Pos(19, 12),
                    Pos(19, 13),
                    Pos(19, 14),
                    Pos(20, 14),
                    Pos(21, 14),
                    Pos(21, 15),
                    Pos(22, 15),
                    Pos(22, 16),
                    Pos(23, 16),
                    Pos(24, 16),
                    Pos(25, 16),
                    Pos(25, 17),
                    Pos(25, 18),
                    Pos(25, 19),
                    Pos(26, 19),
                    Pos(27, 19),
                    Pos(28, 19),
                    Pos(28, 20),
                    Pos(28, 21),
                    Pos(28, 22),
                    Pos(29, 22),
                    Pos(29, 23),
                    Pos(29, 24),
                    Pos(30, 24),
                    Pos(30, 25),
                    Pos(30, 26),
                    Pos(30, 27),
                    Pos(31, 27),
                    Pos(32, 27),
                    Pos(33, 27),
                    Pos(33, 28),
                    Pos(33, 29),
                    Pos(34, 29),
                    Pos(34, 30),
                    Pos(34, 31),
                    Pos(34, 32),
                    Pos(35, 32),
                    Pos(36, 32),
                    Pos(36, 33),
                    Pos(37, 33),
                    Pos(37, 34),
                    Pos(38, 34),
                    Pos(39, 34),
                    Pos(39, 35),
                    Pos(39, 36),
                    Pos(39, 37),
                    Pos(40, 37),
                    Pos(41, 37),
                    Pos(42, 37),
                    Pos(43, 37),
                    Pos(43, 38),
                    Pos(43, 39),
                    Pos(43, 40),
                    Pos(43, 41),
                    Pos(44, 41),
                    Pos(45, 41),
                    Pos(46, 41),
                    Pos(46, 42),
                    Pos(47, 42),
                    Pos(47, 43),
                    Pos(47, 44),
                    Pos(47, 45),
                    Pos(48, 45),
                    Pos(49, 45),
                    Pos(49, 46),
                    Pos(49, 47),
                    Pos(49, 48),
                    Pos(49, 49)
                ],
                315
            ))
        );
    }
}
