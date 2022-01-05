use std::{
    ops::{Add, Mul, Not},
    str::FromStr,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Steps(usize);

impl Add for Steps {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Add<usize> for Steps {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Mul<usize> for Steps {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl From<Steps> for usize {
    fn from(s: Steps) -> Self {
        s.0
    }
}

fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl FromStr for Amphipod {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Amphipod::Amber),
            "B" => Ok(Amphipod::Bronze),
            "C" => Ok(Amphipod::Copper),
            "D" => Ok(Amphipod::Desert),
            _ => Err("Is not a supported amphipod"),
        }
    }
}

impl Amphipod {
    fn step_cost(&self, steps: Steps) -> usize {
        match self {
            Amphipod::Amber => steps,
            Amphipod::Bronze => steps * 10,
            Amphipod::Copper => steps * 100,
            Amphipod::Desert => steps * 1000,
        }
        .into()
    }
    fn room_index(&self) -> usize {
        AMPHIPODS_BY_ROOM
            .iter()
            .position(|amphipod| self.eq(amphipod))
            .expect("All Amphipods has an associated room")
    }
}

const AMPHIPODS_BY_ROOM: [Amphipod; 4] = [
    Amphipod::Amber,
    Amphipod::Bronze,
    Amphipod::Copper,
    Amphipod::Desert,
];

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Hallway([Option<Amphipod>; 11]);

impl Hallway {
    fn amphipods(&self) -> impl Iterator<Item = (usize, Amphipod)> + '_ {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(position, cell)| cell.map(|amphipod| (position, amphipod)))
    }

    fn leave(&mut self, position: usize) -> Option<Amphipod> {
        self.0[position].take()
    }

    fn occupy(&mut self, position: usize, amphipod: Amphipod) {
        self.0[position] = Some(amphipod);
    }

    fn walk(&self, start: usize) -> Walker {
        Walker {
            start,
            left: (&self.0[..start], Some(0)),
            right: (&self.0[start + 1..11], Some(0)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Walk {
    position: usize,
    steps_taken: Steps,
}

/// Iterator to "walk" hallway to find available positions
struct Walker<'hallway> {
    start: usize,
    left: (&'hallway [Option<Amphipod>], Option<usize>),
    right: (&'hallway [Option<Amphipod>], Option<usize>),
}

impl<'hallway> Iterator for Walker<'hallway> {
    type Item = Walk;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut left_steps) = self.left.1 {
            *left_steps += 1;
            if let Some(val) = self.left.0.last() {
                self.left.0 = &self.left.0[..self.left.0.len() - 1];
                if val.is_none() {
                    return Some(Walk {
                        position: self.start - *left_steps,
                        steps_taken: Steps(*left_steps),
                    });
                }
            }
            self.left.1.take();
        };
        if let Some(ref mut right_steps) = self.right.1 {
            *right_steps += 1;
            if let Some(val) = self.right.0.first() {
                self.right.0 = &self.right.0[1..];
                if val.is_none() {
                    return Some(Walk {
                        position: self.start + *right_steps,
                        steps_taken: Steps(*right_steps),
                    });
                }
            }
            self.right.1.take();
        };
        None
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Room([Option<Amphipod>; 4]);

impl Room {
    fn all(&self, amphipod: &Amphipod) -> bool {
        self.0.into_iter().flatten().all(|a| a.eq(amphipod))
    }

    fn leave(&mut self) -> Option<(Amphipod, Steps)> {
        for (index, entry) in self.0.iter_mut().enumerate() {
            if entry.is_some() {
                return entry.take().map(|amphipod| (amphipod, Steps(index + 1)));
            }
        }
        None
    }

    fn occupy(&mut self, amphipod: Amphipod) -> Steps {
        for (index, entry) in self.0.iter_mut().enumerate().rev() {
            if entry.is_none() {
                *entry = Some(amphipod);
                return Steps(index + 1);
            }
        }
        panic!("Room full");
    }
}

const ROOM_HALLWAY_POSITIONS: [usize; 4] = [2, 4, 6, 8];

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Burrow {
    hallway: Hallway,
    rooms: [Room; 4],
}

impl FromStr for Burrow {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut burrow = Burrow::new();
        let mut lines = s.lines().rev();
        lines.next().unwrap();
        for _ in 0..4 {
            let line = lines.next().ok_or("Missing line")?;
            for (pos, amphipod) in line
                .chars()
                .filter_map(|c| c.to_string().parse::<Amphipod>().ok())
                .enumerate()
            {
                burrow.rooms[pos].occupy(amphipod);
            }
        }
        Ok(burrow)
    }
}

impl Burrow {
    pub fn new() -> Self {
        Self::default()
    }

    fn rooms_needing_evictions(&self) -> impl Iterator<Item = usize> + '_ {
        self.rooms
            .iter()
            .enumerate()
            .zip(AMPHIPODS_BY_ROOM)
            .filter_map(|(room, expected_amphipod)| {
                room.1.all(&expected_amphipod).not().then(|| room.0)
            })
    }

    fn hallway_ready_to_enter_room(&self) -> impl Iterator<Item = (usize, Walk)> + '_ {
        self.hallway
            .amphipods()
            .filter(|(_, amphipod)| self.rooms[amphipod.room_index()].all(amphipod))
            .filter_map(|(start, amphipod)| {
                let room_index = amphipod.room_index();
                let room_position = ROOM_HALLWAY_POSITIONS[room_index];
                self.hallway
                    .walk(start)
                    .find(|walk| walk.position == room_position)
                    .map(|walk| (start, walk))
            })
    }

    fn successor_from_room_to_hallway(&self, room_index: usize, walk: Walk) -> (Self, usize) {
        let mut successor = *self;

        let (amphipod, leave_room_steps) = successor.rooms[room_index]
            .leave()
            .expect("Room not empty as it is needs evictions");

        let cost = amphipod.step_cost(leave_room_steps + walk.steps_taken);
        successor.hallway.occupy(walk.position, amphipod);

        (successor, cost)
    }

    fn successor_from_hallway_to_room(&self, start: usize, walk: Walk) -> (Self, usize) {
        let mut successor = *self;

        let amphipod = successor
            .hallway
            .leave(start)
            .expect("Position will contain amphipod");

        let enter_room_steps = successor.rooms[amphipod.room_index()].occupy(amphipod);

        let cost = amphipod.step_cost(walk.steps_taken + enter_room_steps);

        (successor, cost)
    }

    pub fn successors(&self) -> impl Iterator<Item = (Self, usize)> + '_ {
        let room_successors = self
            .rooms_needing_evictions()
            .flat_map(|room_index| {
                self.hallway
                    .walk(ROOM_HALLWAY_POSITIONS[room_index])
                    .filter(|walk| !ROOM_HALLWAY_POSITIONS.contains(&walk.position))
                    .map(move |walk| (room_index, walk))
            })
            .map(|(room_index, walk)| self.successor_from_room_to_hallway(room_index, walk));

        let hallway_successors = self
            .hallway_ready_to_enter_room()
            .map(|(room_index, walk)| self.successor_from_hallway_to_room(room_index, walk));

        room_successors.chain(hallway_successors)
    }

    pub fn estimated_cost(&self) -> usize {
        let room_scores = self.rooms_needing_evictions().map(|room_index| {
            let room_pos = ROOM_HALLWAY_POSITIONS[room_index];
            let room_amphipod = AMPHIPODS_BY_ROOM[room_index];
            let mut room = self.rooms[room_index];
            let mut cost = 0;

            while !room.all(&room_amphipod) {
                let (amphipod, steps) = room.leave().expect("Room can't be empty");
                let steps = if amphipod == room_amphipod {
                    steps + 3
                } else {
                    let pos = ROOM_HALLWAY_POSITIONS[amphipod.room_index()];
                    steps + abs_diff(room_pos, pos) + 1
                };
                cost += amphipod.step_cost(steps);
            }
            cost
        });
        let hallway_scores = self.hallway.amphipods().map(|(pos, amphipod)| {
            let room_pos = ROOM_HALLWAY_POSITIONS[amphipod.room_index()];
            amphipod.step_cost(Steps(abs_diff(room_pos, pos) + 1))
        });
        room_scores.chain(hallway_scores).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn successor_from_room_to_hallway_without_cost(
        burrow: Burrow,
        room_index: usize,
        position: usize,
        hallway_steps_taken: Steps,
    ) -> Burrow {
        burrow
            .successor_from_room_to_hallway(
                room_index,
                Walk {
                    position,
                    steps_taken: hallway_steps_taken,
                },
            )
            .0
    }

    fn successor_from_hallway_to_room_without_cost(
        burrow: Burrow,
        start: usize,
        amphipod: Amphipod,
        hallway_steps_taken: Steps,
    ) -> Burrow {
        burrow
            .successor_from_hallway_to_room(
                start,
                Walk {
                    position: ROOM_HALLWAY_POSITIONS[amphipod.room_index()],
                    steps_taken: hallway_steps_taken,
                },
            )
            .0
    }

    /// Generates:
    ///
    /// #############
    /// #AA...D...DA#
    /// ###A#B#.#.###
    ///   #D#B#C#.#
    ///   #C#B#C#.#
    ///   #D#B#C#.#
    ///   #########
    fn sample_burrow() -> Burrow {
        let mut burrow: Burrow = "#############
#...........#
###A#B#D#A###
  #D#B#C#D#
  #C#B#C#A#
  #D#B#C#A#
  #########"
            .parse()
            .unwrap();

        burrow = successor_from_room_to_hallway_without_cost(burrow, 3, 10, Steps(2));
        burrow = successor_from_room_to_hallway_without_cost(burrow, 3, 9, Steps(1));
        burrow = successor_from_room_to_hallway_without_cost(burrow, 3, 0, Steps(8));
        burrow = successor_from_room_to_hallway_without_cost(burrow, 3, 1, Steps(7));

        burrow = successor_from_room_to_hallway_without_cost(burrow, 2, 5, Steps(1));

        assert_eq!(burrow, burrow);
        burrow
    }

    #[test]
    fn check_successors_from_room_no_blockages() {
        let burrow: Burrow = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #A#B#C#D#
  #A#B#C#D#
  #########"
            .parse()
            .unwrap();

        let mut expected = Burrow::default();

        expected.rooms[0].occupy(Amphipod::Amber);
        expected.rooms[0].occupy(Amphipod::Amber);
        expected.rooms[0].occupy(Amphipod::Amber);
        expected.rooms[0].occupy(Amphipod::Bronze);

        expected.rooms[1].occupy(Amphipod::Bronze);
        expected.rooms[1].occupy(Amphipod::Bronze);
        expected.rooms[1].occupy(Amphipod::Desert);
        expected.rooms[1].occupy(Amphipod::Copper);

        expected.rooms[2].occupy(Amphipod::Copper);
        expected.rooms[2].occupy(Amphipod::Copper);
        expected.rooms[2].occupy(Amphipod::Copper);
        expected.rooms[2].occupy(Amphipod::Bronze);

        expected.rooms[3].occupy(Amphipod::Desert);
        expected.rooms[3].occupy(Amphipod::Desert);
        expected.rooms[3].occupy(Amphipod::Amber);
        expected.rooms[3].occupy(Amphipod::Desert);

        assert_eq!(burrow, expected);
    }

    #[test]
    fn can_get_amphipods_in_hallway() {
        let mut hallway = Hallway::default();
        hallway.occupy(0, Amphipod::Bronze);
        hallway.occupy(6, Amphipod::Amber);
        hallway.occupy(1, Amphipod::Copper);

        let amphipods: Vec<_> = hallway.amphipods().collect();

        assert_eq!(
            amphipods,
            vec![
                (0, Amphipod::Bronze),
                (1, Amphipod::Copper),
                (6, Amphipod::Amber),
            ]
        )
    }

    #[test]
    fn can_walk_occupied_hallway() {
        let mut hallway = Hallway::default();
        hallway.occupy(0, Amphipod::Bronze);
        hallway.occupy(6, Amphipod::Amber);

        let walk: Vec<_> = hallway.walk(3).collect();
        assert_eq!(
            walk,
            vec![
                Walk {
                    position: 2,
                    steps_taken: Steps(1)
                },
                Walk {
                    position: 1,
                    steps_taken: Steps(2)
                },
                Walk {
                    position: 4,
                    steps_taken: Steps(1)
                },
                Walk {
                    position: 5,
                    steps_taken: Steps(2)
                },
            ]
        );
    }

    #[test]
    fn can_walk_partially_occupied_hallway() {
        let mut hallway = Hallway::default();
        hallway.occupy(6, Amphipod::Amber);

        let walk: Vec<_> = hallway.walk(3).collect();
        assert_eq!(
            walk,
            vec![
                Walk {
                    position: 2,
                    steps_taken: Steps(1)
                },
                Walk {
                    position: 1,
                    steps_taken: Steps(2)
                },
                Walk {
                    position: 0,
                    steps_taken: Steps(3)
                },
                Walk {
                    position: 4,
                    steps_taken: Steps(1)
                },
                Walk {
                    position: 5,
                    steps_taken: Steps(2)
                },
            ]
        );
    }

    #[test]
    fn can_walk_empty_hallway() {
        let hallway = Hallway::default();

        let walk: Vec<_> = hallway.walk(3).collect();
        assert_eq!(
            walk,
            vec![
                Walk {
                    position: 2,
                    steps_taken: Steps(1)
                },
                Walk {
                    position: 1,
                    steps_taken: Steps(2)
                },
                Walk {
                    position: 0,
                    steps_taken: Steps(3)
                },
                Walk {
                    position: 4,
                    steps_taken: Steps(1)
                },
                Walk {
                    position: 5,
                    steps_taken: Steps(2)
                },
                Walk {
                    position: 6,
                    steps_taken: Steps(3)
                },
                Walk {
                    position: 7,
                    steps_taken: Steps(4)
                },
                Walk {
                    position: 8,
                    steps_taken: Steps(5)
                },
                Walk {
                    position: 9,
                    steps_taken: Steps(6)
                },
                Walk {
                    position: 10,
                    steps_taken: Steps(7)
                },
            ]
        );
    }

    #[test]
    fn can_occupy_and_leave_room() {
        let mut room = Room::default();
        assert_eq!(room.occupy(Amphipod::Desert), Steps(4));

        assert_eq!(room.occupy(Amphipod::Desert), Steps(3));
        assert!(room.all(&Amphipod::Desert));
        assert_eq!(room.leave(), Some((Amphipod::Desert, Steps(3))));

        assert_eq!(room.occupy(Amphipod::Bronze), Steps(3));
        assert_eq!(room.occupy(Amphipod::Copper), Steps(2));
        assert_eq!(room.occupy(Amphipod::Amber), Steps(1));
        assert!(!room.all(&Amphipod::Desert));

        assert_eq!(room.leave(), Some((Amphipod::Amber, Steps(1))));
        assert_eq!(room.leave(), Some((Amphipod::Copper, Steps(2))));
        assert_eq!(room.leave(), Some((Amphipod::Bronze, Steps(3))));

        assert!(room.all(&Amphipod::Desert));
        assert_eq!(room.leave(), Some((Amphipod::Desert, Steps(4))));

        assert!(room.all(&Amphipod::Desert));
        assert_eq!(room.leave(), None);
    }

    #[test]
    fn can_find_rooms_needing_evictions() {
        let burrow = sample_burrow();
        let successors: Vec<usize> = burrow.rooms_needing_evictions().collect();

        // As only room 0 contains amphipods of the wrong kind, it is the only room to generate successor
        assert_eq!(successors, vec![0]);
    }

    #[test]
    fn can_generate_successors() {
        let burrow = sample_burrow();

        let successors: Vec<_> = burrow.successors().collect();

        assert_eq!(
            successors,
            vec![
                (
                    successor_from_room_to_hallway_without_cost(sample_burrow(), 0, 3, Steps(1)),
                    2
                ),
                (
                    successor_from_hallway_to_room_without_cost(
                        sample_burrow(),
                        5,
                        Amphipod::Desert,
                        Steps(3)
                    ),
                    7000
                ),
                (
                    successor_from_hallway_to_room_without_cost(
                        sample_burrow(),
                        9,
                        Amphipod::Desert,
                        Steps(1)
                    ),
                    5000
                ),
            ],
        );
    }

    #[test]
    fn can_estimated_cost_of_solved_burrow() {
        let resolved_burrow: Burrow = "#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #A#B#C#D#
  #A#B#C#D#
  #########"
            .parse()
            .unwrap();

        assert_eq!(resolved_burrow.estimated_cost(), 0);
    }
    #[test]
    fn can_estimated_cost_of_unsolved_burrow() {
        let burrow = sample_burrow();

        // #############
        // #AA...D...DA#
        // ###A#B#.#.###
        //   #D#B#C#.#
        //   #C#B#C#.#
        //   #D#B#C#.#
        //   #########

        let estimated_cost = [
            Amphipod::Amber.step_cost(Steps(4)),   // Evict and enter room
            Amphipod::Amber.step_cost(Steps(3)),   // Hallway 0 to enter room
            Amphipod::Amber.step_cost(Steps(2)),   // Hallway 1 to enter room
            Amphipod::Amber.step_cost(Steps(9)),   // Hallway 10 to enter room
            Amphipod::Copper.step_cost(Steps(8)),  // Evict, move and enter room
            Amphipod::Desert.step_cost(Steps(11)), // Evict, move and enter room
            Amphipod::Desert.step_cost(Steps(9)),  // Evict, move and enter room
            Amphipod::Desert.step_cost(Steps(4)),  // Hallway 5 to enter room
            Amphipod::Desert.step_cost(Steps(2)),  // Hallway 9 to enter room
        ]
        .into_iter()
        .sum();

        assert_eq!(burrow.estimated_cost(), estimated_cost);
    }
}
