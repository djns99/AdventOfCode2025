use std::io;
use std::io::BufRead;
use itertools::Itertools;

fn part1(ranges: &Vec<(i64, i64)>, valid: Vec<i64>) {
    let num_matches = valid.iter().filter_map(|x| {
        for (start, end) in ranges.iter() {
            if start <= x && x <= end {
                return Some(end - start);
            } else if start > x {
                break;
            }
        }
        None
    }).count();
    println!("{}", num_matches);
}

fn part2(ranges: Vec<(i64, i64)>) {
    // let result = ranges.iter()
    //     .chunk_by(|a, b| {
    //         b.0 <= a.1
    //     }).map(|chunk| {
    //     let first = chunk[0].0;
    //     let last = chunk.into_iter().fold(0, |acc, end| std::cmp::max(acc, end.1));
    //     last - first + 1
    // }).sum::<u64>();
    let result = ranges.iter()
        .fold((0, -1, 0), |acc, x| {
            if x.0 <= acc.1 {
                println!("{:?} {:?} continue range", acc, x);
                (acc.0, std::cmp::max(acc.1, x.1), acc.2)
            } else {
                println!("{:?} {:?} ends range", acc, x);
                (x.0, x.1, acc.2 + (acc.1 - acc.0 + 1))
            }
        });
    let answer = result.2 + (result.1 - result.0 + 1);
    println!("{:?}", answer);
}

fn main() {
    let input_lines = io::stdin().lines().map(Result::unwrap).collect_vec();
    let mut input_iter = input_lines.into_iter();
    let mut ranges = input_iter
        .take_while_ref(|x| !x.is_empty())
        .map(|x| {
            x.split('-')
             .map(|half| half.parse().expect(format!("could not parse {}, {} was not an integer", x, half).as_str()))
             .collect_tuple().unwrap()
        }).collect_vec();
    let valid = input_iter.skip(1)
        .map(|ingredient| ingredient.parse().unwrap())
        .collect_vec();

    ranges.sort();
    part1(&ranges, valid);
    part2(ranges);
}