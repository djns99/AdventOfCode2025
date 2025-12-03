use std::io;
use std::iter;
use itertools::Itertools;

fn part1(lines: &Vec<String>) {
    let result = lines.iter().fold(0, |acc, line| {
        let result = line.chars().map(|x| x.to_digit(10).unwrap()).fold((0, 0), |(first_digit, second_digit), digit| {
            if first_digit < second_digit {
                (second_digit, digit)
            } else if digit >= first_digit {
                (first_digit, digit)
            } else if digit > second_digit {
                (first_digit, digit)
            } else {
                (first_digit, second_digit)
            }
        });
        acc + result.0 * 10 + result.1
    });
    println!("Answer is {:?}", result);
}

fn shift_in(arr: &[u32; 12], next: u32) -> [u32; 12] {
    let full_arr =  arr.iter().chain(iter::once(&next));
    let first_out_of_order_value = full_arr.clone().tuple_windows().position(|(w1, w2)| w1 < w2).unwrap_or(12);
    full_arr.enumerate()
        .filter_map(|(i, &x)| { if i != first_out_of_order_value { Some(x) } else { None } })
        .collect_array()
        .expect("Couldn't collect array")
}

fn part2(lines: &Vec<String>) {
    let result = lines.iter()
        .fold(0, |acc, line| {
            let result = line.chars()
                .map(|x| x.to_digit(10).expect("Failed to parse digit"))
                .fold([0; 12], |shift_register, digit| {
                    shift_in(&shift_register, digit)
                }).iter()
                .fold(0u64, |acc, digit| acc * 10 + (*digit as u64));
            acc + result
        });
    println!("Answer is {:?}", result);
}


fn main() {
    let lines = io::stdin().lines().map(|x| x.unwrap()).collect();
    part1(&lines);
    part2(&lines);
}