use petgraph::{
    graphmap::{GraphMap, UnGraphMap},
    EdgeType,
};
use std::io::{BufRead, BufReader};

fn is_small_cave(name: &str) -> bool {
    name.to_lowercase() == name
}

fn get_paths(lines: &[(String, String)]) -> Vec<(String, bool)> {
    let mut caves = UnGraphMap::<&str, ()>::default();
    let start = caves.add_node("start");
    let end = caves.add_node("end");

    for line in lines {
        caves.add_edge(&line.0, &line.1, ());
    }

    traverse_graph(&caves, start, start, end, false, &[])
}

fn traverse_graph<T: EdgeType>(
    caves: &GraphMap<&str, (), T>,
    node: &str,
    start: &str,
    end: &str,
    double_visits_taken: bool,
    already_visited: &[&str],
) -> Vec<(String, bool)> {
    if node == end {
        return vec![(end.to_owned(), double_visits_taken)];
    }
    let visited = if is_small_cave(node) {
        let mut update_vis = already_visited.to_owned();
        update_vis.push(node);
        update_vis
    } else {
        already_visited.to_vec()
    };

    caves
        .neighbors(node)
        .filter(|&n| n != start)
        .filter(|n| !(double_visits_taken && visited.contains(n)))
        .flat_map(|n| {
            let double_visits_taken = double_visits_taken || visited.contains(&n);

            traverse_graph(caves, n, start, end, double_visits_taken, &visited)
                .into_iter()
                .map(|(rn, rt)| (format!("{},{}", node, rn), rt))
        })
        .collect()
}

fn create_parts(s: String) -> Result<(String, String), String> {
    match s.split('-').collect::<Vec<&str>>()[..] {
        [a, b] => Ok((a.to_owned(), b.to_owned())),
        _ => Err(format!("Could not find two elements: {}", s)),
    }
}

fn main() -> Result<(), String> {
    let lines: Vec<(String, String)> = BufReader::new(std::io::stdin())
        .lines()
        .map(|r| r.map_err(|e| e.to_string()).and_then(create_parts))
        .collect::<Result<Vec<(String, String)>, _>>()?;

    let paths = get_paths(&lines);
    println!(
        "Paths without a small cave double visit: {:?}",
        paths.iter().filter(|(_, a)| !a).count()
    );
    println!(
        "Paths with single small cave double visit allowed: {:?}",
        paths.len()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_parse_line() {
        let lines = [
            ("start".to_string(), "A".to_string()),
            ("start".to_string(), "b".to_string()),
            ("A".to_string(), "c".to_string()),
            ("A".to_string(), "b".to_string()),
            ("b".to_string(), "d".to_string()),
            ("A".to_string(), "end".to_string()),
            ("b".to_string(), "end".to_string()),
        ];

        let paths = get_paths(&lines);
        let mut part_1 = paths
            .iter()
            .filter_map(|(s, b)| match b {
                false => Some(s),
                _ => None,
            })
            .collect::<Vec<&String>>();
        let mut part_2 = paths.iter().map(|(s, _)| s).collect::<Vec<&String>>();

        part_1.sort();
        part_2.sort();

        assert_eq!(
            part_1,
            [
                "start,A,b,A,c,A,end",
                "start,A,b,A,end",
                "start,A,b,end",
                "start,A,c,A,b,A,end",
                "start,A,c,A,b,end",
                "start,A,c,A,end",
                "start,A,end",
                "start,b,A,c,A,end",
                "start,b,A,end",
                "start,b,end",
            ],
        );

        assert_eq!(
            part_2,
            [
                "start,A,b,A,b,A,c,A,end",
                "start,A,b,A,b,A,end",
                "start,A,b,A,b,end",
                "start,A,b,A,c,A,b,A,end",
                "start,A,b,A,c,A,b,end",
                "start,A,b,A,c,A,c,A,end",
                "start,A,b,A,c,A,end",
                "start,A,b,A,end",
                "start,A,b,d,b,A,c,A,end",
                "start,A,b,d,b,A,end",
                "start,A,b,d,b,end",
                "start,A,b,end",
                "start,A,c,A,b,A,b,A,end",
                "start,A,c,A,b,A,b,end",
                "start,A,c,A,b,A,c,A,end",
                "start,A,c,A,b,A,end",
                "start,A,c,A,b,d,b,A,end",
                "start,A,c,A,b,d,b,end",
                "start,A,c,A,b,end",
                "start,A,c,A,c,A,b,A,end",
                "start,A,c,A,c,A,b,end",
                "start,A,c,A,c,A,end",
                "start,A,c,A,end",
                "start,A,end",
                "start,b,A,b,A,c,A,end",
                "start,b,A,b,A,end",
                "start,b,A,b,end",
                "start,b,A,c,A,b,A,end",
                "start,b,A,c,A,b,end",
                "start,b,A,c,A,c,A,end",
                "start,b,A,c,A,end",
                "start,b,A,end",
                "start,b,d,b,A,c,A,end",
                "start,b,d,b,A,end",
                "start,b,d,b,end",
                "start,b,end",
            ],
        );
    }
}
