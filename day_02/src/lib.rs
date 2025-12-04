#![feature(never_type)]
#![allow(unused_variables)]

/// Make sure outputs impl Display!
type Output<T> = Result<T, Box<dyn std::error::Error>>;

pub fn part_1(input: &'static str) -> Output<u64> {
    let sum = input
        .trim()
        .split(',')
        .filter_map(|s| s.split_once('-'))
        .filter_map(|(start, end)| {
            // optimization 1: exclude ranges which include zero even-length numbers
            if start.len() % 2 == 1 && start.len() == end.len() {
                return None;
            }

            let mut n_start: u64 = start.parse().unwrap();
            let mut n_end: u64 = end.parse().unwrap();
            // optimization 2: normalize ranges to only include even lengths
            // case 1: start is odd length, end is even length -> start must be increased
            if start.len() % 2 == 1 && end.len() % 2 == 0 {
                n_start = 10u64.pow(start.len() as u32);
            }
            // case 2: start is even length, end is odd length -> end must be decreased
            else if start.len() % 2 == 0 && end.len() % 2 == 1 {
                n_end = 10u64.pow(end.len() as u32 - 1) - 1;
            }
            Some((n_start, n_end))
        })
        // 1. if first halves of start and end are the same, check if first half is
        //    in the range created from the bottom halves of start/end
        // 2. if first are different but are of equal length, we repeat the check for all first halves
        //    in the range from start_first_half to end_first_half
        .map(|(start, end)| {
            let l_start = start.checked_ilog10().unwrap_or(0) + 1;
            let l_end = end.checked_ilog10().unwrap_or(0) + 1;

            let (mut start_half, mut end_half) = (start, end);
            for _ in 0..(l_start / 2) {
                start_half /= 10;
            }
            for _ in 0..(l_end / 2) {
                end_half /= 10;
            }

            let mut total = 0;
            for half in start_half..=end_half {
                let candidate = half * 10u64.pow(l_start / 2) + half;
                if (start..=end).contains(&candidate) {
                    total += candidate;
                }
            }
            total
        })
        .sum();
    Ok(sum)
}

pub fn part_2(input: &'static str) -> Output<u64> {
    todo!()
}
