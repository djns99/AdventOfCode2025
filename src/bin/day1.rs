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
        // Entire revolutions we can ignore
        let mut shift_revolutions = shift.abs() / 100;

        // Remaining shift to apply
        let extra_shift = shift % 100;

        // The new position
        let mut new_pos = curr.1 + extra_shift;

        // If we have extra shift that will get us an extra zero, add it
        if extra_shift != 0 && curr.1 != 0 && (new_pos <= 0 || new_pos >= 100) {
            shift_revolutions += 1;
        }
        new_pos = new_pos.rem_euclid(100);
        println!("State {:?} + shift {:?} ({:?} * 100 + {:?}) = {:?} with {:?} + {:?} total zeros", curr, shift, shift.abs() / 100, extra_shift, new_pos, curr.0, shift_revolutions);

        return (curr.0 + shift_revolutions, new_pos);
    });
    println!("Answer is {:?}", answer);
}

fn main() {
    let answer = io::stdin().lines().map(|x| x.unwrap()).collect();
    part1(&answer);
    part2(&answer);
}