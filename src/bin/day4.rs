use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use itertools::Itertools;

fn count_neighbours(x: usize, y: usize, grid: &Vec<Vec<char>>) -> usize {
    let w = grid[0].len() as isize;
    let h = grid.len() as isize;
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dy == 0 && dx == 0 { continue; }
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || nx >= w || ny < 0 || ny >= h || grid[ny as usize][nx as usize] == '.' {
                count += 1;
            }
        }
    }
    count
}

fn part1(grid: &Vec<Vec<char>>) {
    let w = grid[0].len();
    let h = grid.len();
    let mut count = 0;
    for y in 0..h {
        for x in 0..w {
            if grid[y][x] == '.' { continue; }
            let neighbours = count_neighbours(x, y, &grid);
            if neighbours >= 5 {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

fn part2(mut grid: Vec<Vec<char>>) {
    let w = grid[0].len();
    let h = grid.len();
    let mut done_map = Vec::new();
    let mut count_map = HashMap::new();
    for y in 0..h {
        for x in 0..w {
            if grid[y][x] == '.' { continue; }
            let c = count_neighbours(x, y, &grid);
            if c >= 5 { done_map.push((x,y)); }
            else { count_map.insert((x,y), c); }
        }
    }

    let mut count = 0;
    while !done_map.is_empty() {
        count += 1;
        let (x,y) = done_map.pop().unwrap();
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dy == 0 && dx == 0 { continue; }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && nx < (w as isize) && ny >= 0 && ny < (h as isize) {
                    let entry = count_map.entry((nx as usize, ny as usize)).and_modify(|c| *c += 1);
                    match entry {
                        std::collections::hash_map::Entry::Vacant(_) => { continue; },
                        std::collections::hash_map::Entry::Occupied(e) => {
                            if *e.get() >= 5 {
                                let (k, _) = e.remove_entry();
                                done_map.push(k);
                            }
                        }
                    }
                }
            }
        }

    }

    println!("{}", count);
}

fn main() {
    let grid = io::stdin().lines().map(|x| x.unwrap().chars().collect()).collect();
    part1(&grid);
    part2(grid);
}