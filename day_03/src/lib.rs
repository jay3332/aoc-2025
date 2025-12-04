#![feature(never_type)]
#![allow(unused_variables)]

/// Make sure outputs impl Display!
type Output<T> = Result<T, Box<dyn std::error::Error>>;

pub fn part_1(input: &'static str) -> Output<u32> {
    let sum = input
        .lines()
        .map(|line| {
            let mut max_index = 0;
            let line = line.as_bytes();
            for i in 0..(line.len() - 1) {
                if line[i] > line[max_index] {
                    max_index = i;
                }
            }

            let a = line[max_index] as char;
            let b = *line.iter().skip(max_index + 1).max().unwrap() as char;

            a.to_digit(10).unwrap() * 10 + b.to_digit(10).unwrap()
        })
        .sum();
    Ok(sum)
}

pub fn part_2(input: &'static str) -> Output<!> {
    todo!()
}
