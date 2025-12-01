use std::io;

fn part1(input: &Vec<String>) {
    let answer = input.iter().fold((0, 50), |curr, line| {
        let direction = if line.chars().next() == Some('L') { -1 } else { 1 };
        let shift = line[1..].parse::<i32>().unwrap() * direction;
        let new_pos = (curr.1 + shift).rem_euclid(100);
        match new_pos {
            0 => (curr.0 + 1, new_pos),
            _ => (curr.0, new_pos)
        }
    });
    println!("Answer is {:?}", answer);
}

fn part2(input: &Vec<String>) {
    let answer = input.iter().fold((0, 50), |curr, line| {
        let direction = if line.chars().next() == Some('L') { -1 } else { 1 };
        let shift = line[1..].parse::<i32>().unwrap() * direction;
        let unwrapped = curr.1 + shift;
        let wrapped = unwrapped.rem_euclid(100);
        let diff = ((unwrapped - wrapped) / 100).abs();
        let diff = if unwrapped <= 0 { diff + 1 } else { diff };
        return (curr.0 + diff.abs(), wrapped);
    });
    println!("Answer is {:?}", answer);
}

fn main() {
    let answer = io::stdin().lines().map(|x| x.unwrap()).collect();
    part1(&answer);
    part2(&answer);
}