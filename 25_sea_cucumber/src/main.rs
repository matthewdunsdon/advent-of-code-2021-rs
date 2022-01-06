use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum StepResult {
    Moved,
    NoMovement,
}

impl StepResult {
    fn then(self, rhs: Self) -> Self {
        match self {
            StepResult::Moved => self,
            StepResult::NoMovement => rhs,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum CucumberHerd {
    MoveEast,
    MoveSouth,
}

impl Display for CucumberHerd {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            CucumberHerd::MoveEast => ">",
            CucumberHerd::MoveSouth => "v",
        };
        write!(f, "{}", val)
    }
}

impl CucumberHerd {
    fn step(&self, position: &(usize, usize), bounds: (usize, usize)) -> (usize, usize) {
        match self {
            CucumberHerd::MoveEast => ((position.0 + 1) % bounds.0, position.1),
            CucumberHerd::MoveSouth => (position.0, (position.1 + 1) % bounds.1),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Region {
    cucumbers: HashMap<(usize, usize), CucumberHerd>,
    height: usize,
    width: usize,
}

impl FromIterator<String> for Region {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut height = 0;
        let mut width = 0;
        let cucumbers = iter
            .into_iter()
            .enumerate()
            .flat_map(|(y, line)| {
                height = height.max(y + 1);
                width = width.max(line.len());
                line.chars()
                    .enumerate()
                    .filter_map(move |(x, c)| match c {
                        '>' => Some(((x, y), CucumberHerd::MoveEast)),
                        'v' => Some(((x, y), CucumberHerd::MoveSouth)),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        Self {
            cucumbers,
            height,
            width,
        }
    }
}

impl Display for Region {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            writeln!(f)?;
            for x in 0..self.width {
                let key = &(x, y);
                if self.cucumbers.contains_key(key) {
                    write!(f, "{}", self.cucumbers.get(key).unwrap())?;
                } else {
                    write!(f, ".")?;
                };
            }
        }

        Ok(())
    }
}

impl Region {
    fn till_no_movement(&mut self) -> usize {
        let mut steps = 1;

        while self.take_step() == StepResult::Moved {
            steps += 1;
        }

        steps
    }

    fn sub_step(&mut self, cucumber_type: CucumberHerd) -> StepResult {
        let mut moved = StepResult::NoMovement;

        let other_herd = self
            .cucumbers
            .iter()
            .filter(|(_, cucumber)| cucumber_type.ne(cucumber))
            .fold(
                HashMap::with_capacity(self.cucumbers.len()),
                |mut cucumbers, (position, herd)| {
                    cucumbers.insert(*position, herd.to_owned());
                    cucumbers
                },
            );

        let next_cucumbers = self
            .cucumbers
            .iter()
            .filter(|(_, cucumber)| cucumber_type.eq(cucumber))
            .fold(other_herd, |mut cucumbers, (position, herd)| {
                let next_position = herd.step(position, (self.width, self.height));
                let new_position = if !self.cucumbers.contains_key(&next_position) {
                    moved = StepResult::Moved;
                    next_position
                } else {
                    position.to_owned()
                };

                cucumbers.insert(new_position, herd.to_owned());
                cucumbers
            });

        self.cucumbers = next_cucumbers;
        moved
    }

    fn take_step(&mut self) -> StepResult {
        let movement_east = self.sub_step(CucumberHerd::MoveEast);
        let movement_south = self.sub_step(CucumberHerd::MoveSouth);

        movement_east.then(movement_south)
    }
}

fn main() {
    let mut region: Region = BufReader::new(std::io::stdin())
        .lines()
        .filter_map(Result::ok)
        .collect();

    let steps = region.till_no_movement();
    println!("Region: {}", region);
    println!("In steps: {}", steps);
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use super::*;

    static TEST_INPUT: &str = r"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn can_display() {
        let region: Region = TEST_INPUT.lines().map(|s| s.to_string()).collect();

        let mut display = String::new();
        write!(&mut display, "{}", region).unwrap();

        assert_eq!(
            display,
            r"
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"
        );
    }

    #[test]
    fn can_count_till_no_movement() {
        let mut region: Region = TEST_INPUT.lines().map(|s| s.to_string()).collect();

        assert_eq!(region.till_no_movement(), 58);
    }

    #[test]
    fn can_display_after_no_movement() {
        let mut region: Region = TEST_INPUT.lines().map(|s| s.to_string()).collect();

        region.till_no_movement();

        let mut display = String::new();
        write!(&mut display, "{}", region).unwrap();

        assert_eq!(
            display,
            r"
..>>v>vv..
..v.>>vv..
..>>v>>vv.
..>>>>>vv.
v......>vv
v>v....>>v
vvv.....>>
>vv......>
.>v.vv.v.."
        );
    }
}
