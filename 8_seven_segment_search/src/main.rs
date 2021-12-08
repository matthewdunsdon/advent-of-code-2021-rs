use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};

fn solve(entry: Entry) -> Result<[i8; 4], String> {
    let mut digit_1 = None;
    let mut digit_4 = None;
    let mut digit_7 = None;
    let mut digit_8 = None;
    let mut len_5_digits = Vec::new();
    let mut len_6_digits = Vec::new();
    for signal_pattern in entry.signal_patterns {
        match signal_pattern.0.len() {
            2 if digit_1.is_none() => digit_1 = Some(signal_pattern),
            3 if digit_7.is_none() => digit_7 = Some(signal_pattern),
            4 if digit_4.is_none() => digit_4 = Some(signal_pattern),
            7 if digit_8.is_none() => digit_8 = Some(signal_pattern),
            5 => len_5_digits.push(signal_pattern),
            6 => len_6_digits.push(signal_pattern),
            _ => return Err("Invalid digit occured".to_string()),
        }
    }
    let digit_1 = digit_1.ok_or("No digit 1")?;
    let digit_4 = digit_4.ok_or("No digit 4")?;
    let digit_7 = digit_7.ok_or("No digit 7")?;
    let digit_8 = digit_8.ok_or("No digit 8")?;
    let [b_segment, d_segment] = match digit_4.0.difference(&digit_1.0).collect::<Vec<&Segment>>()[..]
    {
        [x, y] => {
            if len_6_digits.iter().all(|digit| digit.0.contains(x)) {
                [x, y]
            } else {
                [y, x]
            }
        }
        _ => panic!(),
    };
    let (digit_5, digit_2_or_3): (Vec<SignalPattern>, Vec<SignalPattern>) = len_5_digits
        .into_iter()
        .partition(|digit| digit.0.contains(b_segment));
    let (digit_6_or_9, digit_0): (Vec<SignalPattern>, Vec<SignalPattern>) = len_6_digits
        .into_iter()
        .partition(|digit| digit.0.contains(d_segment));
    let digit_5 = digit_5.first().ok_or("No digit 5")?;
    let digit_0 = digit_0.first().ok_or("No digit 0")?;
    let [c_segment, e_segment] = match digit_0.0.difference(&digit_5.0).collect::<Vec<&Segment>>()[..]
    {
        [x, y] => {
            if digit_2_or_3.iter().all(|digit| digit.0.contains(x)) {
                [x, y]
            } else {
                [y, x]
            }
        }
        _ => panic!(),
    };
    let (digit_2, digit_3): (Vec<SignalPattern>, Vec<SignalPattern>) = digit_2_or_3
        .into_iter()
        .partition(|digit| digit.0.contains(e_segment));
    let (digit_9, digit_6): (Vec<SignalPattern>, Vec<SignalPattern>) = digit_6_or_9
        .into_iter()
        .partition(|digit| digit.0.contains(c_segment));
    let digit_2 = digit_2.first().ok_or("No digit 2")?;
    let digit_3 = digit_3.first().ok_or("No digit 3")?;
    let digit_6 = digit_6.first().ok_or("No digit 6")?;
    let digit_9 = digit_9.first().ok_or("No digit 9")?;

    let res = entry.output_values.map(|a| {
        if a.0.eq(&digit_0.0) {
            0
        } else if a.0.eq(&digit_1.0) {
            1
        } else if a.0.eq(&digit_2.0) {
            2
        } else if a.0.eq(&digit_3.0) {
            3
        } else if a.0.eq(&digit_4.0) {
            4
        } else if a.0.eq(&digit_5.0) {
            5
        } else if a.0.eq(&digit_6.0) {
            6
        } else if a.0.eq(&digit_7.0) {
            7
        } else if a.0.eq(&digit_8.0) {
            8
        } else if a.0.eq(&digit_9.0) {
            9
        } else {
            panic!()
        }
    });
    Ok(res)
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Segment {
    fn from_char(s: char) -> Result<Self, String> {
        match s {
            'a' => Ok(Segment::A),
            'b' => Ok(Segment::B),
            'c' => Ok(Segment::C),
            'd' => Ok(Segment::D),
            'e' => Ok(Segment::E),
            'f' => Ok(Segment::F),
            'g' => Ok(Segment::G),
            _ => Err(format!("Unrecognised segment {}", s)),
        }
    }
}

#[derive(Debug)]
struct SignalPattern(HashSet<Segment>);

impl std::str::FromStr for SignalPattern {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .map(Segment::from_char)
            .collect::<Result<Vec<Segment>, _>>()
            .map(|segments| {
                let pattern: HashSet<Segment> = HashSet::from_iter(segments.into_iter());
                SignalPattern(pattern)
            })
    }
}

#[derive(Debug)]
struct Entry {
    signal_patterns: [SignalPattern; 10],
    output_values: [SignalPattern; 4],
}

impl std::str::FromStr for Entry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        match parts.len() {
            15 => {
                let signal_patterns: [SignalPattern; 10] = [
                    parts[0].parse()?,
                    parts[1].parse()?,
                    parts[2].parse()?,
                    parts[3].parse()?,
                    parts[4].parse()?,
                    parts[5].parse()?,
                    parts[6].parse()?,
                    parts[7].parse()?,
                    parts[8].parse()?,
                    parts[9].parse()?,
                ];
                let output_values: [SignalPattern; 4] = [
                    parts[11].parse()?,
                    parts[12].parse()?,
                    parts[13].parse()?,
                    parts[14].parse()?,
                ];
                Ok(Entry {
                    signal_patterns,
                    output_values,
                })
            }
            _ => Err("Invalid formated line supplied".to_owned()),
        }
    }
}

fn main() -> Result<(), String> {
    let entries = BufReader::new(std::io::stdin())
        .lines()
        .filter_map(Result::ok)
        .map(|s| s.parse())
        .collect::<Result<Vec<Entry>, _>>()?;

    let mut count_simple_values = 0;
    let mut sum = 0_i32;
    for entry in entries {
        let solve_value = solve(entry)?;
        count_simple_values += solve_value
            .iter()
            .filter(|x| matches!(x, 1 | 4 | 7 | 8))
            .count();
        let [th, h, te, u] = solve_value;
        let val = i32::from(th) * 1000 + i32::from(h) * 100 + i32::from(te) * 10 + i32::from(u);
        sum += val;
    }
    println!("Count of simple value {:?}", count_simple_values);
    println!("Summation {:?}", sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_solve(input: &str) -> [i8; 4] {
        solve(input.parse::<Entry>().unwrap()).unwrap()
    }

    #[test]
    fn check_solve() {
        let inputs: [(&str, [i8;4]);11] = [
            ("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf", [5,3,5,3]),
            ("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe", [8,3,9,4]),
            ("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc", [9,7,8,1]),
            ("fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg", [1,1,9,7]),
            ("fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb", [9,3,6,1]),
            ("aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea", [4,8,7,3]),
            ("fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb", [8,4,1,8]),
            ("dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe", [4,5,4,8]),
            ("bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef", [1,6,2,5]),
            ("egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb", [8,7,1,7]),
            ("gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce", [4,3,1,5]),
        ];
        for (input, expectation) in inputs {
            assert_eq!(get_solve(input), expectation);
        }
    }
}
