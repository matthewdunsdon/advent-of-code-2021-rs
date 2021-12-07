use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(std::io::stdin());
    let args: Vec<String> = std::env::args().collect();

    let count = match args.get(1) {
        Some(v) if v == "windowed" => {
            count_depth_increments(reader, WindowedResultDepthMeasure::new())
        }
        _ => count_depth_increments(reader, SingleResultDepthMeasure::new()),
    };
    println!("Count {}", count);
}

fn count_depth_increments(read: impl std::io::BufRead, from: impl DepthMeasure) -> i32 {
    let readings = BufReader::new(read)
        .lines()
        .filter_map(Result::ok)
        .filter_map(|s| s.parse::<i32>().ok());

    let mut counter = 0;
    let mut previous = from;
    for current in readings {
        let measure = previous.generate_next_measure(current);
        if let Some(reading) = measure.reading() {
            if matches!(previous.reading(), Some(last_reading) if last_reading < reading) {
                counter += 1;
            }
        }
        previous = measure
    }
    return counter;
}

trait DepthMeasure {
    fn generate_next_measure(&self, reading: i32) -> Self;
    fn reading(&self) -> Option<i32>;
}

pub struct SingleResultDepthMeasure {
    value: Option<i32>,
}

impl SingleResultDepthMeasure {
    fn new() -> Self {
        SingleResultDepthMeasure { value: None }
    }
}

impl DepthMeasure for SingleResultDepthMeasure {
    fn generate_next_measure(&self, reading: i32) -> Self {
        SingleResultDepthMeasure {
            value: Some(reading),
        }
    }

    fn reading(&self) -> Option<i32> {
        self.value
    }
}

pub struct WindowedResultDepthMeasure {
    value: (Option<i32>, Option<i32>, Option<i32>),
}

impl WindowedResultDepthMeasure {
    fn new() -> Self {
        WindowedResultDepthMeasure {
            value: (None, None, None),
        }
    }
}

impl DepthMeasure for WindowedResultDepthMeasure {
    fn generate_next_measure(&self, reading: i32) -> Self {
        let (_, second, third) = self.value;
        WindowedResultDepthMeasure {
            value: (second, third, Some(reading)),
        }
    }

    fn reading(&self) -> Option<i32> {
        let (first, second, third) = self.value;
        first.and_then(|a| second.and_then(|b| third.map(|c| a + b + c)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    static SIMPLE_CASE: &[u8] = b"199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

    #[test]
    fn can_count_changes() {
        let cursor = Cursor::new(SIMPLE_CASE);
        let count = count_depth_increments(cursor, SingleResultDepthMeasure::new());
        assert_eq!(count, 7);
    }
    #[test]
    fn can_count_changes_with_window() {
        let cursor = Cursor::new(SIMPLE_CASE);
        let count = count_depth_increments(cursor, WindowedResultDepthMeasure::new());
        assert_eq!(count, 5);
    }
}
