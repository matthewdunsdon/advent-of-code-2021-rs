use std::io::{BufRead, BufReader};

#[derive(Default)]
struct Counter {
    count: i16,
    bit_counts: [i16; 16],
}

fn most_common_bit_scan(counter: &mut Counter, value: &i16) -> Option<i16> {
    let mut bit_counts: [i16; 16] = [0; 16];
    let mut result = 0_i16;

    let count = counter.count + 1;
    // Add 1 to round up when dividing by 2
    let half_way_count = (count + 1) / 2;

    for n in 0..16 {
        bit_counts[n] = counter.bit_counts[n] + has_bit_at(value, n) as i16;

        if bit_counts[n] >= half_way_count {
            result += 1 << n;
        }
    }

    *counter = Counter { count, bit_counts };
    Some(result)
}

fn get_oxygen_generator_rating(mut current_readings: Vec<i16>, bit_index: usize) -> Vec<i16> {
    if current_readings.len() > 1 {
        let has_bit = has_bit_at(&get_most_common_bits(&current_readings).unwrap(), bit_index);
        current_readings = current_readings
            .into_iter()
            .filter(|r| has_bit_at(r, bit_index) == has_bit)
            .collect();
    }
    current_readings
}

fn get_scrubber_rating(mut current_readings: Vec<i16>, bit_index: usize) -> Vec<i16> {
    if current_readings.len() > 1 {
        let has_bit = has_bit_at(&get_most_common_bits(&current_readings).unwrap(), bit_index);
        current_readings = current_readings
            .into_iter()
            .filter(|r| has_bit_at(r, bit_index) != has_bit)
            .collect();
    }
    current_readings
}

fn get_most_common_bits(readings: &Vec<i16>) -> Option<i16> {
    let m = readings
        .iter()
        .scan(Counter::default(), most_common_bit_scan)
        .collect::<Vec<i16>>();

    m.last().map(|a| a.to_owned())
}

fn has_bit_at(value: &i16, position: usize) -> bool {
    ((value >> position) & 1) > 0
}

fn main() {
    let readings: Vec<i16> = BufReader::new(std::io::stdin())
        .lines()
        .filter_map(Result::ok)
        .filter_map(|d| i16::from_str_radix(&d[..], 2).ok())
        .collect();

    let most_common_bits = get_most_common_bits(&readings);
    let oxy_reading = match (0..12)
        .rev()
        .fold(readings.clone(), get_oxygen_generator_rating)[..]
    {
        [reading] => Some(reading),
        _ => None,
    };
    let scr_reading = match (0..12).rev().fold(readings.clone(), get_scrubber_rating)[..] {
        [reading] => Some(reading),
        _ => None,
    };

    let data =
        most_common_bits.and_then(|b| oxy_reading.and_then(|o| scr_reading.map(|s| (b, o, s))));

    match data {
        Some((gamma_rate, oxygen_generator_rating, scrubber_rating)) => {
            let epsilon_rate = gamma_rate ^ 0b111111111111_i16;
            let power_consumption = i32::from(gamma_rate) * i32::from(epsilon_rate);
            let life_support_rating =
                i32::from(oxygen_generator_rating) * i32::from(scrubber_rating);

            println!("gamma rate:              {0:8} ({:#b})", gamma_rate);
            println!("epsilon rate:            {0:8} ({:#b})", epsilon_rate);
            println!("power_consumption:       {0:8} ({:#b})", power_consumption);
            println!(
                "oxygen_generator_rating: {0:8} ({:#b})",
                oxygen_generator_rating
            );
            println!("scrubber_rating:         {0:8} ({:#b})", scrubber_rating);
            println!(
                "life_support_rating:     {0:8} ({:#b})",
                life_support_rating
            );
        }
        None => println!("No results"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_simple_case() -> Vec<i16> {
        vec![
            0b00100, // 01
            0b11110, // 02
            0b10110, // 03
            0b10111, // 04
            0b10101, // 05
            0b01111, // 06
            0b00111, // 07
            0b11100, // 08
            0b10000, // 09
            0b11001, // 10
            0b00010, // 11
            0b01010, // 12
        ]
    }

    #[test]
    fn check_most_common_bit_scan() {
        let simple_case = get_simple_case();
        let results = simple_case
            .iter()
            .scan(Counter::default(), most_common_bit_scan);

        let expectation: Vec<i16> = vec![
            0b00100, // 01 | 0b00100 | 0 0 1 0 0
            0b11110, // 02 | 0b11110 | 1 1 2 1 0
            0b10110, // 03 | 0b10110 | 2 1 3 2 0
            0b10110, // 04 | 0b10111 | 3 1 4 3 1
            0b10110, // 05 | 0b10101 | 4 1 5 3 2
            0b10111, // 06 | 0b01111 | 4 2 6 4 3
            0b10111, // 07 | 0b00111 | 4 2 7 5 4
            0b10111, // 08 | 0b11100 | 5 3 8 5 4
            0b10110, // 09 | 0b10000 | 6 3 8 5 4
            0b10111, // 10 | 0b11001 | 7 4 8 5 5
            0b10110, // 11 | 0b00010 | 7 4 8 6 5
            0b10110, // 12 | 0b01010 | 7 5 8 7 5
        ];
        for ((actual, expected), index) in results.zip(&expectation).zip(1..) {
            assert_eq!(
                actual, *expected,
                "for step:{} actual:{:#07b} expected:{:#07b}",
                index, actual, expected
            );
        }
    }

    #[test]
    fn check_get_oxygen_generator_rating() {
        let simple_case = get_simple_case();
        let results = (0..5)
            .rev()
            .fold(simple_case.clone(), get_oxygen_generator_rating);

        assert!(matches!(results[..], [reading] if reading == 23));
    }

    #[test]
    fn check_get_oxygen_generator_rating_by_step() {
        let mut results = get_oxygen_generator_rating(get_simple_case(), 4);
        assert_eq!(
            results,
            vec![
                0b11110, // 02
                0b10110, // 03
                0b10111, // 04
                0b10101, // 05
                0b11100, // 08
                0b10000, // 09
                0b11001, // 10
            ]
        );
        results = get_oxygen_generator_rating(results, 3);
        assert_eq!(
            results,
            vec![
                0b10110, // 03
                0b10111, // 04
                0b10101, // 05
                0b10000, // 09
            ]
        );
        results = get_oxygen_generator_rating(results, 2);
        assert_eq!(
            results,
            vec![
                0b10110, // 03
                0b10111, // 04
                0b10101, // 05
            ]
        );
        results = get_oxygen_generator_rating(results, 1);
        assert_eq!(
            results,
            vec![
                0b10110, // 03
                0b10111, // 04
            ]
        );
        results = get_oxygen_generator_rating(results, 0);
        assert_eq!(results, vec![0b10111]);
    }

    #[test]
    fn check_get_scrubber_rating() {
        let simple_case = get_simple_case();
        let results = (0..5).rev().fold(simple_case.clone(), get_scrubber_rating);

        assert!(matches!(results[..], [reading] if reading == 10));
    }

    #[test]
    fn check_get_scrubber_rating_by_step() {
        let mut results = get_scrubber_rating(get_simple_case(), 4);
        assert_eq!(
            results,
            vec![
                0b00100, // 01
                0b01111, // 06
                0b00111, // 07
                0b00010, // 11
                0b01010, // 12
            ]
        );
        results = get_scrubber_rating(results, 3);
        assert_eq!(
            results,
            vec![
                0b01111, // 06
                0b01010, // 12
            ]
        );
        results = get_scrubber_rating(results, 2);
        assert_eq!(results, vec![0b01010]);
        results = get_scrubber_rating(results, 1);
        assert_eq!(results, vec![0b01010]);
        results = get_scrubber_rating(results, 0);
        assert_eq!(results, vec![0b01010]);
    }
}
