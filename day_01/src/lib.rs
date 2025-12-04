#![feature(never_type)]
#![allow(unused_variables)]

/// Make sure outputs impl Display!
type Output<T> = Result<T, Box<dyn std::error::Error>>;

pub fn part_1(input: &'static str) -> Output<u32> {
    let mut running = 50;
    let mut zeros = 0;
    for line in input.lines() {
        let sign = match &line[0..1] {
            "L" => -1,
            "R" => 1,
            _ => 0, // won't happen
        };
        let num: i32 = line[1..].parse()?;
        running += sign * num;

        if running % 100 == 0 {
            zeros += 1;
        }
    }
    Ok(zeros)
}

pub fn part_2(input: &'static str) -> Output<u32> {
    let mut running = 50;
    let mut zeros = 0;
    for line in input.lines() {
        let sign = match &line[0..1] {
            "L" => -1,
            "R" => 1,
            _ => 0, // won't happen
        };
        let num: i32 = line[1..].parse()?;

        let old = running;
        running += sign * num;
        let new = running;

        // compute # times it crosses a multiple of 100 (exclude start but include end)
        zeros += (new / 100).abs_diff(old / 100);
        // edge case: crossing positive to negative boundary
        if old >= 0 && new < 0 || old < 0 && new >= 0 {
            zeros += 1;
        }
        // boundary case 1: starting on positive zero mod 100 and decreasing
        //                  or starting on negative zero mod 100 and increasing
        if old >= 0 && old % 100 == 0 && sign == -1 || old < 0 && old % 100 == 0 && sign == 1 {
            zeros -= 1;
        }
        // boundary case 2: ending on positive zero mod 100 and decreasing
        //                  or ending on negative zero mod 100 and increasing
        if new >= 0 && new % 100 == 0 && sign == -1 || new < 0 && new % 100 == 0 && sign == 1 {
            zeros += 1;
        }
    }
    Ok(zeros)
}
