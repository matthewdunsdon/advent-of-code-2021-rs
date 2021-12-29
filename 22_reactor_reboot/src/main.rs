use std::{
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Clone, Copy, Debug, PartialEq)]
enum RebootState {
    Off,
    On,
}

impl FromStr for RebootState {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(RebootState::On),
            "off" => Ok(RebootState::Off),
            _ => Err("Unrecognised reboot state"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct CuboidRebootStep {
    state: RebootState,
    start: (i64, i64, i64),
    end: (i64, i64, i64),
}

impl FromStr for CuboidRebootStep {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (state, coords) = s
            .split_once(" ")
            .ok_or("Unable to extract cuboid reboot state")?;

        let coords = coords
            .splitn(3, ',')
            .map(|c| {
                let (a, b) = c[2..].split_once("..").unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            })
            .collect::<Vec<(i64, i64)>>();

        let start = (coords[0].0, coords[1].0, coords[2].0);
        let end = (coords[0].1, coords[1].1, coords[2].1);

        Ok(CuboidRebootStep {
            state: state.parse()?,
            start,
            end,
        })
    }
}

impl CuboidRebootStep {
    fn cubes_on(&self) -> usize {
        match self.state {
            RebootState::Off => 0,
            RebootState::On => ((1 + self.end.0 - self.start.0)
                * (1 + self.end.1 - self.start.1)
                * (1 + self.end.2 - self.start.2))
                .try_into()
                .unwrap(),
        }
    }

    fn overlaps_with(&self, target: &CuboidRebootStep) -> bool {
        target.end.0 >= self.start.0
            && self.end.0 >= target.start.0
            && target.end.1 >= self.start.1
            && self.end.1 >= target.start.1
            && target.end.2 >= self.start.2
            && self.end.2 >= target.start.2
    }

    fn non_overlaping_cuboids(&self, target: &CuboidRebootStep) -> Vec<CuboidRebootStep> {
        if self.overlaps_with(target) {
            let mut parts = vec![];
            if self.start.0 < target.start.0 {
                parts.push(CuboidRebootStep {
                    state: self.state,
                    start: self.start,
                    end: (target.start.0 - 1, self.end.1, self.end.2),
                });
            }
            if target.end.0 < self.end.0 {
                parts.push(CuboidRebootStep {
                    state: self.state,
                    start: (target.end.0 + 1, self.start.1, self.start.2),
                    end: self.end,
                });
            }
            let overlap_in_x = (
                self.start.0.max(target.start.0),
                self.end.0.min(target.end.0),
            );
            if self.start.1 < target.start.1 {
                parts.push(CuboidRebootStep {
                    state: self.state,
                    start: (overlap_in_x.0, self.start.1, self.start.2),
                    end: (overlap_in_x.1, target.start.1 - 1, self.end.2),
                });
            }
            if target.end.1 < self.end.1 {
                parts.push(CuboidRebootStep {
                    state: self.state,
                    start: (overlap_in_x.0, target.end.1 + 1, self.start.2),
                    end: (overlap_in_x.1, self.end.1, self.end.2),
                });
            }
            let overlap_in_y = (
                self.start.1.max(target.start.1),
                self.end.1.min(target.end.1),
            );
            if self.start.2 < target.start.2 {
                parts.push(CuboidRebootStep {
                    state: self.state,
                    start: (overlap_in_x.0, overlap_in_y.0, self.start.2),
                    end: (overlap_in_x.1, overlap_in_y.1, target.start.2 - 1),
                });
            }
            if target.end.2 < self.end.2 {
                parts.push(CuboidRebootStep {
                    state: self.state,
                    start: (overlap_in_x.0, overlap_in_y.0, target.end.2 + 1),
                    end: (overlap_in_x.1, overlap_in_y.1, self.end.2),
                });
            }
            parts
        } else {
            vec![self.clone()]
        }
    }
}

fn update_cuboids_list(
    cuboids: Vec<CuboidRebootStep>,
    next_cuboid: CuboidRebootStep,
) -> Vec<CuboidRebootStep> {
    let mut next_cuboids = cuboids
        .into_iter()
        .flat_map(|f| f.non_overlaping_cuboids(&next_cuboid))
        .collect::<Vec<_>>();
    if next_cuboid.state == RebootState::On {
        next_cuboids.push(next_cuboid);
    }
    next_cuboids
}

fn main() {
    let initial_cuboids = BufReader::new(std::io::stdin())
        .lines()
        .filter_map(Result::ok)
        .filter_map(|l| l.parse::<CuboidRebootStep>().ok())
        .collect::<Vec<_>>();

    let simple_cuboids_on = initial_cuboids
        .clone()
        .into_iter()
        .filter(|c| {
            c.start.0.min(c.start.1.min(c.start.2)) >= -50
                && c.end.0.min(c.end.1.min(c.end.2)) <= 50
        })
        .fold(Vec::new(), update_cuboids_list);
    let simple_cubes_on: usize = simple_cuboids_on.iter().map(|s| s.cubes_on()).sum();
    println!("Total cubes on for simple case: {}", simple_cubes_on);

    let cuboid_on: Vec<CuboidRebootStep> = initial_cuboids
        .into_iter()
        .fold(Vec::new(), update_cuboids_list);

    let cubes_on: usize = cuboid_on.iter().map(|s| s.cubes_on()).sum();
    println!("Total cubes on: {}", cubes_on);
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! cuboid_reboot_step_tests {
        ($($name:ident($input:expr).parses_to($expected:expr),)*) => {
        $(
            #[test]
            fn $name() {
                assert_eq!($input.parse::<CuboidRebootStep>().unwrap(), $expected);
            }
        )*
        };
        ($($name:ident($input:expr).cubes_on($expected:expr),)*) => {
        $(
            #[test]
            fn $name() {
                assert_eq!($input.parse::<CuboidRebootStep>().unwrap().cubes_on(), $expected);
            }
        )*
        };
        ($($name:ident($input:expr).overlaps_with($other:expr).is($expected:expr),)*) => {
        $(
            #[test]
            fn $name() {
                assert_eq!($input.parse::<CuboidRebootStep>().unwrap()
                    .overlaps_with(&$other.parse::<CuboidRebootStep>().unwrap()), $expected);
            }
        )*
        };
        ($($name:ident($input:expr).non_overlaping_cuboids($other:expr).is($expected:expr),)*) => {
        $(
            #[test]
            fn $name() {
                let expected_cuboids = $expected.into_iter().map(|s| s.parse().unwrap()).collect::<Vec<_>>();

                assert_eq!($input.parse::<CuboidRebootStep>().unwrap()
                    .non_overlaping_cuboids(&$other.parse::<CuboidRebootStep>().unwrap()), expected_cuboids);
            }
        )*
        };
        ($($name:ident($inputs:expr).track_total_cubes_on($expected:expr),)*) => {
        $(
            #[test]
            fn $name() {
                let input_cuboids = $inputs.into_iter().map(|s| s.parse().unwrap())
                    .scan(Vec::new(), |cuboids: &mut Vec<CuboidRebootStep>, next_cuboid| {
                        *cuboids = cuboids
                            .into_iter()
                            .flat_map(|f| f.non_overlaping_cuboids(&next_cuboid))
                            .collect::<Vec<_>>();
                        if next_cuboid.state == RebootState::On {
                            cuboids.push(next_cuboid);
                        }
                        let count : usize = cuboids.iter().map(|c| c.cubes_on()).sum();
                        Some(count)
                    })
                        .collect::<Vec<_>>();

                assert_eq!(input_cuboids, $expected);
            }
        )*
        };
    }

    cuboid_reboot_step_tests! {
        parsing_on("on x=10..12,y=10..12,z=10..12").parses_to(
            CuboidRebootStep {
                state: RebootState::On,
                start: (10, 10, 10),
                end: (12, 12, 12)
            }),

            parsing_off("off x=18..30,y=-20..-8,z=-3..13").parses_to(
            CuboidRebootStep {
                state: RebootState::Off,
                start: (18, -20, -3),
                end: (30, -8, 13)
            }),
    }

    cuboid_reboot_step_tests! {
        cubes_on_for_simple_on_cube("on x=10..12,y=10..12,z=10..12").cubes_on(27),

        cubes_on_for_simple_off_cube("off x=18..30,y=-20..-8,z=-3..13").cubes_on(0),

        cubes_on_for_single_cube("on x=10..10,y=10..10,z=10..10").cubes_on(1),

        cubes_on_for_large_cuboid("on x=-57795..-6158,y=29564..72030,z=20435..90618").cubes_on(153_907_261_834_064),
    }

    cuboid_reboot_step_tests! {
        cuboids_overlap_when_cuboid_completely_within("on x=10..12,y=10..12,z=10..12").overlaps_with("on x=11..11,y=11..11,z=11..11").is(true),

        cuboids_do_not_overlap_when_target_x_is_lower_then_self("off x=18..30,y=-20..18,z=-3..13").overlaps_with("on x=10..10,y=10..10,z=10..10").is(false),

        cuboids_do_not_overlap_when_target_x_is_high_then_self("on x=10..10,y=10..10,z=10..10").overlaps_with("off x=18..30,y=-20..18,z=-3..13").is(false),

        cuboids_do_not_overlap_when_target_y_is_lower_then_self("on x=10..10,y=10..10,z=10..10").overlaps_with("off x=8..30,y=-20..8,z=-3..13").is(false),

        cuboids_do_not_overlap_when_target_y_is_high_then_self("off x=8..30,y=-20..8,z=-3..13").overlaps_with("on x=10..10,y=10..10,z=10..10").is(false),

        cuboids_do_not_overlap_when_target_z_is_lower_then_self("on x=10..10,y=10..10,z=10..10").overlaps_with("off x=8..30,y=-20..18,z=-3..3").is(false),

        cuboids_do_not_overlap_when_target_z_is_high_then_self("off x=8..30,y=-20..18,z=-3..3").overlaps_with("on x=10..10,y=10..10,z=10..10").is(false),
    }

    cuboid_reboot_step_tests! {
        non_overlaping_cuboids_return_self_when_no_overlap("on x=10..10,y=10..10,z=10..10")
            .non_overlaping_cuboids("on x=18..30,y=-20..18,z=-3..13")
            .is(vec!["on x=10..10,y=10..10,z=10..10"]),

        overlaping_cuboids_return_segments_higher_x_both_y_lower_z("on x=-5..47,y=-31..22,z=-19..33")
            .non_overlaping_cuboids("on x=-44..5,y=-27..21,z=-14..35")
            .is(vec![
                "on x=6..47,y=-31..22,z=-19..33", // higher x area
                "on x=-5..5,y=-31..-28,z=-19..33", // lower y area (inside x overlap)
                "on x=-5..5,y=22..22,z=-19..33", // higher y area (inside x overlap)
                "on x=-5..5,y=-27..21,z=-19..-15", // lower z area (inside x,y overlap)
            ]),

        overlaping_cuboids_return_segments_lower_x_no_y_higher_z("on x=-44..5,y=-27..21,z=-14..35")
            .non_overlaping_cuboids("on x=-5..47,y=-31..22,z=-19..33")
            .is(vec![
                "on x=-44..-6,y=-27..21,z=-14..35", // lower x area
                "on x=-5..5,y=-27..21,z=34..35", // higher z area (inside x,y overlap)
            ]),
    }

    cuboid_reboot_step_tests! {
        track_total_cubes_on_simple_case(vec![
                "on x=10..12,y=10..12,z=10..12",
                "on x=11..13,y=11..13,z=11..13",
                "off x=9..11,y=9..11,z=9..11",
                "on x=10..10,y=10..10,z=10..10",
            ])
            .track_total_cubes_on(vec![27, 46, 38, 39]),

        track_total_cubes_on_larger_example(vec![
                "on x=-20..26,y=-36..17,z=-47..7",
                "on x=-20..33,y=-21..23,z=-26..28",
                "on x=-22..28,y=-29..23,z=-38..16",
                "on x=-46..7,y=-6..46,z=-50..-1",
                "on x=-49..1,y=-3..46,z=-24..28",
                "on x=2..47,y=-22..22,z=-23..27",
                "on x=-27..23,y=-28..26,z=-21..29",
                "on x=-39..5,y=-6..47,z=-3..44",
                "on x=-30..21,y=-8..43,z=-13..34",
                "on x=-22..26,y=-27..20,z=-29..19",
                "off x=-48..-32,y=26..41,z=-47..-37",
                "on x=-12..35,y=6..50,z=-50..-2",
                "off x=-48..-32,y=-32..-16,z=-15..-5",
                "on x=-18..26,y=-33..15,z=-7..46",
                "off x=-40..-22,y=-38..-28,z=23..41",
                "on x=-16..35,y=-41..10,z=-47..6",
                "off x=-32..-23,y=11..30,z=-14..3",
                "on x=-49..-5,y=-3..45,z=-29..18",
                "off x=18..30,y=-20..-8,z=-3..13",
                "on x=-41..9,y=-7..43,z=-33..15",
            ])
            .track_total_cubes_on(vec![
                    139590,
                    210918,
                    225476,
                    328328,
                    387734,
                    420416,
                    436132,
                    478727,
                    494759,
                    494804,
                    492164,
                    534936,
                    534936,
                    567192,
                    567150,
                    592167,
                    588567,
                    592902,
                    590029,
                    590784
                ]),
    }
}
