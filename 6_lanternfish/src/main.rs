use std::io::{BufRead, BufReader};

type Lanternfish = i8;
type Sample = [usize; 9];

fn get_lanternfish(s: String) -> Vec<Lanternfish> {
    s.split(",").filter_map(|z| z.parse::<i8>().ok()).collect()
}

fn age_generation(sample: Sample, _: usize) -> Sample {
    [
        sample[1],             // 0s
        sample[2],             // 1s
        sample[3],             // 2s
        sample[4],             // 3s
        sample[5],             // 4s
        sample[6],             // 5s
        sample[7] + sample[0], // 6s
        sample[8],             // 7s
        sample[0],             // 8s
    ]
}

fn count_lanternfish(list: &Vec<Lanternfish>) -> Sample {
    [
        list.iter().filter(|l| *l == &0).count(),
        list.iter().filter(|l| *l == &1).count(),
        list.iter().filter(|l| *l == &2).count(),
        list.iter().filter(|l| *l == &3).count(),
        list.iter().filter(|l| *l == &4).count(),
        list.iter().filter(|l| *l == &5).count(),
        list.iter().filter(|l| *l == &6).count(),
        list.iter().filter(|l| *l == &7).count(),
        list.iter().filter(|l| *l == &8).count(),
    ]
}

fn total_sample(sample: Sample) -> usize {
    sample.iter().sum::<usize>()
}

fn main() {
    let lanternfish: Vec<Lanternfish> = BufReader::new(std::io::stdin())
        .lines()
        .filter_map(Result::ok)
        .flat_map(get_lanternfish)
        .collect();

    let after_18 = (0..18)
        .into_iter()
        .fold(count_lanternfish(&lanternfish), age_generation);
    let after_80 = (18..80).into_iter().fold(after_18, age_generation);
    let after_256 = (80..256).into_iter().fold(after_80, age_generation);

    println!("Total after 18: {}", total_sample(after_18));
    println!("Total after 80: {}", total_sample(after_80));
    println!("Total after 256: {}", total_sample(after_256));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_get_lanternfish() {
        assert_eq!(get_lanternfish("3,4,3,1,2".to_owned()), vec![3, 4, 3, 1, 2]);
    }

    #[test]
    fn check_count_lanternfish() {
        let s = count_lanternfish(&vec![3, 4, 3, 1, 2]);
        assert_eq!(s, [0, 1, 1, 2, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn check_total_sample() {
        assert_eq!(total_sample([0, 1, 1, 2, 1, 0, 0, 0, 0]), 5);
    }

    #[test]
    fn check_age_generation() {
        let initial = [
            [0, 1, 1, 2, 1, 0, 0, 0, 0],
            [1, 1, 2, 1, 0, 0, 0, 0, 0],
            [1, 2, 1, 0, 0, 0, 1, 0, 1],
            [2, 1, 0, 0, 0, 1, 1, 1, 1],
            [1, 0, 0, 0, 1, 1, 3, 1, 2],
        ];
        assert_eq!(age_generation(initial[0], 0), initial[1]);
        assert_eq!(age_generation(initial[1], 1), initial[2]);
        assert_eq!(age_generation(initial[2], 2), initial[3]);
        assert_eq!(age_generation(initial[3], 3), initial[4]);
    }
}
