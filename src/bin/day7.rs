use std::collections::HashSet;
use std::collections::HashMap;
use std::io;

fn part1(lines: &Vec<String>) {
    let start_index = lines[0].find('S').unwrap();
    let mut replace_positions = Vec::new();
    let mut state = (HashSet::from([start_index]), 0);
    let result = lines[1..].iter().fold(&mut state, |state, line| {
        replace_positions.clear();
        for &pos in &state.0 {
            if line.as_bytes()[pos] == b'^' {
                replace_positions.push(pos);
            }
        }

        for &pos in &replace_positions {
            state.0.remove(&pos);
            if pos > 0 {
                    state.0.insert(pos-1);
            }
            if pos < line.len() - 1 {
                state.0.insert(pos + 1);
            }
        }
        state.1 += replace_positions.len();

        state
    });

    println!("Num splits: {}", result.1);
}

fn part2(lines: &Vec<String>) {
    let start_index = lines[0].find('S').unwrap();
    let mut replace_positions = Vec::new();
    let mut state = HashMap::from([(start_index, 1usize)]);
    let result: usize = lines[1..].iter().fold(&mut state, |state, line| {
        replace_positions.clear();
        for (&pos, &mut count) in state.into_iter() {
            if line.as_bytes()[pos] == b'^' {
                replace_positions.push((pos, count));
            }
        }

        for pos in &replace_positions {
            state.remove(&pos.0);
            state.entry(pos.0-1).and_modify(|e| *e += pos.1).or_insert(pos.1);
            state.entry(pos.0+1).and_modify(|e| *e += pos.1).or_insert(pos.1);
        }

        state
    }).values().sum();

    println!("Num splits: {}", result);
}

fn main() {
    let lines = io::stdin().lines().map(Result::unwrap).collect();
    part1(&lines);
    part2(&lines);
}