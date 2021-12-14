use std::{
    collections::HashMap,
    hash::Hash,
    io::{BufRead, BufReader},
    ops::{Add, Div},
    str::FromStr,
};

fn count_pairs(polymer_template: &String) -> HashMap<Pair, u64> {
    let mut mapping = HashMap::with_capacity(polymer_template.len());

    for (start, end) in polymer_template
        .chars()
        .zip(polymer_template.chars().skip(1))
    {
        *mapping.entry(Pair { start, end }).or_insert(0) += 1;
    }
    mapping
}

fn take_step(
    pair_counts: &HashMap<Pair, u64>,
    pair_insertion_lookup: &HashMap<Pair, char>,
) -> HashMap<Pair, u64> {
    let mut mapping = HashMap::with_capacity(pair_counts.len());

    for (pair, count) in pair_counts {
        match pair_insertion_lookup.get(pair) {
            Some(insert_char) => {
                *mapping
                    .entry(Pair {
                        start: pair.start,
                        end: insert_char.clone(),
                    })
                    .or_insert(0) += count;
                *mapping
                    .entry(Pair {
                        start: insert_char.clone(),
                        end: pair.end,
                    })
                    .or_insert(0) += count;
            }
            None => {
                panic!("No mapping defined for pair: {:?}", pair);
            }
        }
    }
    mapping
}

fn count_characters(pair_counts: &HashMap<Pair, u64>) -> HashMap<char, u64> {
    let mut mapping = HashMap::with_capacity(pair_counts.len());

    for (pair, count) in pair_counts.into_iter() {
        *mapping.entry(pair.start).or_insert(0) += count;
        *mapping.entry(pair.end).or_insert(0) += count;
    }
    for val in mapping.values_mut() {
        *val = val.add(1).div(2);
    }
    mapping
}

fn score_count(char_count: &HashMap<char, u64>) -> Option<(u64, u64)> {
    char_count.values().max().and_then(|max| {
        char_count
            .values()
            .min()
            .map(|min| (max.to_owned(), min.to_owned()))
    })
}

fn main() -> Result<(), String> {
    let lines: Vec<String> = BufReader::new(std::io::stdin())
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .map_err(|e| e.to_string())?;

    let mut iter = lines.iter();
    let polymer_template = iter.next().ok_or("No polymer template")?;
    let pair_insertion_rules = iter
        .skip(1)
        .map(|s| s.parse())
        .collect::<Result<Vec<PairInsertionRule>, _>>()?;

    let pair_insertion_lookup: HashMap<Pair, char> =
        HashMap::from_iter(pair_insertion_rules.into_iter().map(|i| (i.pair, i.insert)));

    let after_ten_steps = (0..10)
        .into_iter()
        .fold(count_pairs(polymer_template), |x, _| {
            take_step(&x, &pair_insertion_lookup)
        });

    let (max, min) = score_count(&count_characters(&after_ten_steps)).ok_or("No count")?;
    println!(
        "After ten steps. max:{}, min:{}, score:{}",
        max,
        min,
        max - min
    );

    let after_fourty_steps = (10..40).into_iter().fold(after_ten_steps, |x, _| {
        take_step(&x, &pair_insertion_lookup)
    });
    let (max, min) = score_count(&count_characters(&after_fourty_steps)).ok_or("No count")?;
    println!(
        "After ten steps. max:{}, min:{}, score:{}",
        max,
        min,
        max - min
    );

    Ok(())
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Pair {
    start: char,
    end: char,
}

impl FromStr for Pair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().collect::<Vec<char>>()[..] {
            [start, end] => Ok(Pair { start, end }),
            _ => Err(format!("Bad pair insertion rules: {}", s)),
        }
    }
}

#[derive(Debug)]
struct PairInsertionRule {
    pair: Pair,
    insert: char,
}

impl FromStr for PairInsertionRule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [elements, element] = s.split(" -> ").collect::<Vec<&str>>()[..] {
            let pair: Pair = elements.parse()?;
            if let [insert] = element.chars().collect::<Vec<char>>()[..] {
                return Ok(PairInsertionRule { pair, insert });
            }
        }
        Err(format!("Bad pair insertion rules: {}", s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_pair_insertion_rules() -> Vec<PairInsertionRule> {
        vec![
            "CH -> B".parse().unwrap(),
            "HH -> N".parse().unwrap(),
            "CB -> H".parse().unwrap(),
            "NH -> C".parse().unwrap(),
            "HB -> C".parse().unwrap(),
            "HC -> B".parse().unwrap(),
            "HN -> C".parse().unwrap(),
            "NN -> C".parse().unwrap(),
            "BH -> H".parse().unwrap(),
            "NC -> B".parse().unwrap(),
            "NB -> B".parse().unwrap(),
            "BN -> B".parse().unwrap(),
            "BB -> N".parse().unwrap(),
            "BC -> B".parse().unwrap(),
            "CC -> N".parse().unwrap(),
            "CN -> C".parse().unwrap(),
        ]
    }

    #[test]
    fn check_count_pairs() {
        let polymer_template = "NNCB".to_string();

        let pair_counters = count_pairs(&polymer_template);
        let mut entries = Vec::from_iter(pair_counters);
        entries.sort();

        assert_eq!(
            entries,
            vec![
                ("CB".parse().unwrap(), 1),
                ("NC".parse().unwrap(), 1),
                ("NN".parse().unwrap(), 1)
            ]
        );
    }

    #[test]
    fn check_take_step() {
        let polymer_template = "NNCB".to_string();
        let rules = sample_pair_insertion_rules();
        let pair_insertion_lookup: HashMap<Pair, char> =
            HashMap::from_iter(rules.into_iter().map(|i| (i.pair, i.insert)));
        let initial = count_pairs(&polymer_template);

        let step = take_step(&initial, &pair_insertion_lookup);
        let mut entries = Vec::from_iter(step.iter());
        entries.sort();

        assert_eq!(
            entries,
            vec![
                (&"BC".parse().unwrap(), &1),
                (&"CH".parse().unwrap(), &1),
                (&"CN".parse().unwrap(), &1),
                (&"HB".parse().unwrap(), &1),
                (&"NB".parse().unwrap(), &1),
                (&"NC".parse().unwrap(), &1),
            ]
        );
        let step = take_step(&step, &pair_insertion_lookup);
        let mut entries = Vec::from_iter(step.iter());
        entries.sort();

        assert_eq!(
            entries,
            vec![
                (&"BB".parse().unwrap(), &2),
                (&"BC".parse().unwrap(), &2),
                (&"BH".parse().unwrap(), &1),
                (&"CB".parse().unwrap(), &2),
                (&"CC".parse().unwrap(), &1),
                (&"CN".parse().unwrap(), &1),
                (&"HC".parse().unwrap(), &1),
                (&"NB".parse().unwrap(), &2),
            ]
        );
    }

    #[test]
    fn check_count_characters() {
        let mut entries = Vec::from_iter(count_characters(&count_pairs(&"NNCB".to_string())));
        entries.sort();

        assert_eq!(entries, vec![('B', 1), ('C', 1), ('N', 2)]);

        let mut entries = Vec::from_iter(count_characters(&count_pairs(
            &"NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".to_string(),
        )));
        entries.sort();

        assert_eq!(entries, vec![('B', 23), ('C', 10), ('H', 5), ('N', 11)]);
    }
}
