use std::io::{BufRead, BufReader};

#[derive(PartialEq, Debug)]
enum Movement {
    Up(i32),
    Down(i32),
    Forward(i32),
}

fn extract_reading(line: String) -> Option<Movement> {
    let segments = line.split_whitespace().collect::<Vec<&str>>();
    let action = segments.get(0);
    let distance = segments.get(1).and_then(|d| d.parse::<i32>().ok());

    distance.and_then(|d| match action {
        Some(&"forward") => Some(Movement::Forward(d)),
        Some(&"up") => Some(Movement::Up(d)),
        Some(&"down") => Some(Movement::Down(d)),
        _ => None,
    })
}

fn dive(current: (i32, i32, ()), movement: Movement) -> (i32, i32, ()) {
    let (depth, horizontal, _) = current;
    match movement {
        Movement::Up(d) => (depth - d, horizontal, ()),
        Movement::Down(d) => (depth + d, horizontal, ()),
        Movement::Forward(d) => (depth, horizontal + d, ()),
    }
}

fn aimed_dive(current: (i32, i32, i32), movement: Movement) -> (i32, i32, i32) {
    let (depth, horizontal, aim) = current;
    match movement {
        Movement::Up(d) => (depth, horizontal, aim - d),
        Movement::Down(d) => (depth, horizontal, aim + d),
        Movement::Forward(d) => (depth + (d * aim), horizontal + d, aim),
    }
}

fn log_readings<P>(reading: (i32, i32, P)) {
    println!(
        "Depth {} Horizontal position {} and when multiplied {}",
        reading.0,
        reading.1,
        reading.0 * reading.1
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let readings = BufReader::new(std::io::stdin())
        .lines()
        .filter_map(Result::ok)
        .filter_map(extract_reading);

    match args.get(1) {
        Some(v) if v == "aimed" => log_readings(readings.fold((0, 0, 0), aimed_dive)),
        _ => log_readings(readings.fold((0, 0, ()), dive)),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_extract_readings() {
        assert_eq!(
            extract_reading("forward 5".to_owned()),
            Some(Movement::Forward(5))
        );
        assert_eq!(
            extract_reading("down 5".to_owned()),
            Some(Movement::Down(5))
        );
        assert_eq!(
            extract_reading("forward 8".to_owned()),
            Some(Movement::Forward(8))
        );
        assert_eq!(extract_reading("up 3".to_owned()), Some(Movement::Up(3)));
        assert_eq!(extract_reading("forward a".to_owned()), None);
        assert_eq!(extract_reading("upwards 5".to_owned()), None);
        assert_eq!(extract_reading("reset".to_owned()), None);
    }
    #[test]
    fn check_dive() {
        let movements = vec![
            Movement::Forward(5),
            Movement::Down(5),
            Movement::Forward(8),
            Movement::Up(3),
            Movement::Down(8),
            Movement::Forward(2),
        ];

        let results = movements.into_iter().fold(Vec::new(), |mut r, movement| {
            let (depth, horizontal) = r.last().unwrap_or(&(0, 0));
            let result = dive((*depth, *horizontal, ()), movement);
            r.push((result.0, result.1));
            r
        });

        let expectation = vec![(0, 5), (5, 5), (5, 13), (2, 13), (10, 13), (10, 15)];

        assert_eq!(results, expectation);
    }
    #[test]
    fn check_aimed_dive() {
        let movements = vec![
            Movement::Forward(5),
            Movement::Down(5),
            Movement::Forward(8),
            Movement::Up(3),
            Movement::Down(8),
            Movement::Forward(2),
        ];

        let results = movements.into_iter().fold(Vec::new(), |mut r, movement| {
            let last = r.last().unwrap_or(&(0, 0, 0));
            let result = aimed_dive(*last, movement);
            r.push(result);
            r
        });

        let expectation = vec![
            (0, 5, 0),
            (0, 5, 5),
            (40, 13, 5),
            (40, 13, 2),
            (40, 13, 10),
            (60, 15, 10),
        ];

        assert_eq!(results, expectation);
    }
}
