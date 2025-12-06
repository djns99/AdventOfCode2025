use std::fmt::format;
use std::{io, iter};
use std::ops::Add;
use itertools::{enumerate, Itertools};

fn solve(numbers: Vec<Vec<i64>>, operation: Vec<char>) {
    let mut result = numbers[0].clone();
    let answer = numbers.iter().skip(1).fold(&mut result, |acc, next| {
        for i in 0..next.len() {
            // println!("{} {} {}", acc[i], operation[i], next[i]);
            match operation[i] {
                '*' => acc[i] *= next[i],
                '+' => acc[i] += next[i],
                _ => panic!()
            };
        }
        acc
    });
    let answer: i64 = answer.iter().sum();

    println!("{:?}", answer);

}
fn part1(lines: &[String]) {
    let parts = lines.into_iter().chunk_by(|line| line.chars().any(|c| c.is_numeric()));
    let mut parts_it = parts.into_iter();
    let numbers = parts_it.next().unwrap().1 // Products vec
        .map(|line|
            line.trim()
                .split([' ', '\t'])
                .filter_map(|x| if x.is_empty() {
                    None
                } else {
                    Some(x.parse().expect(&format!("expected {} to be int", x))) // Get integer
                }).collect_vec()
        ).collect_vec();
    let operation = parts_it.next().unwrap().1 // Operations vec
        .map(|line|
            line.trim()
                .split([' ', '\t'])
                .filter_map(|x| if x.is_empty() { None } else { Some(x.chars().next().unwrap()) })
                .collect_vec()
        ).next().unwrap();

    println!("{:?}", numbers);
    println!("{:?}", operation);

    solve(numbers, operation);
}

fn part2(lines: Vec<String>) {
    let numbers = lines.iter().take(lines.len() - 1);
    let mut transpose = vec![String::new(); lines[0].len()];
    
    let operations = lines.last().unwrap().chars().filter(|x| *x != ' '); 
    /*.scan('\0', |a, c| {
        if c != ' ' {
            *a = c;
        }
        Some(*a)
    }).collect_vec()*/;
    
    let numbers = numbers.fold(&mut transpose, |acc, line| {
        let line_bytes = line.as_bytes(); // Cursed rust stuff
        println!("{} has len {}", line, line.len());
        for i in 0..line.len() {
            acc[i].push(line_bytes[i] as char);
        };
        acc
    }).into_iter().map(|result| {
        result.trim().parse::<i64>().unwrap_or(-1)
    });
    
    let mut operations_iter = operations.peekable();
    let mut advance = || {let c= operations_iter.next().unwrap(); match c  {
        '*' => (c, 1),
        _ => (c, 0),
    }
    };
    let apply = |state:(char,i64), next| {
        (state.0, match state.0 {
            '+' => state.1 + next,
            '*' => state.1 * next,
            _ => panic!(),
      })
    };
    let answer = numbers.fold((0, advance()), |acc, next| {
        if next == -1 {
            return (acc.0 + acc.1.1, advance());
        }
        (acc.0, apply(acc.1, next))
    });
    let answer = answer.0 + answer.1.1;
    
    
    println!("{:?}", answer);
}

fn main() {
    let input = io::stdin().lines().map(Result::unwrap).collect_vec();
    // part1(input.as_slice());
    part2(input);
}