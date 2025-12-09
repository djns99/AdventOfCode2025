use std::collections::{HashMap, HashSet};
use std::io;
use std::io::BufRead;
use std::ops::Mul;
use itertools::Itertools;
use std::collections::hash_map::Entry;

fn sqr<T: Copy + Mul<T, Output = T>>(v: T) -> T {
    v * v
}

fn part1(nodes: &Vec<(i64, i64, i64)>) {
    // let num_pairs = sqr(nodes.len());
    // subtrace guass' formula for sum(1, total_nodes - m - 1) from total num_pairs
    // let ctoi = |m, n| {
    //     let N = nodes.len() - m - 1;
    //     num_pairs - (N * (N + 1) / 2) + n
    // };

    let itoc = |c| (c % nodes.len(), c / nodes.len());

    let mut distance_matrix = vec![i64::MAX; sqr(nodes.len())];
    nodes.iter().enumerate().for_each(|(m, &(x, y, z))| {
        nodes.iter().enumerate().skip(m+1).for_each(|(n, (x_, y_, z_))| {
            distance_matrix[m * nodes.len() + n] = sqr(x - x_) + sqr(y - y_) + sqr(z - z_);
        })
    });

    let result = distance_matrix.iter().enumerate().sorted_unstable_by_key(|(_, d)| *d);

    let mut connections: HashMap<usize, Vec<usize>> = HashMap::new();
    result.take(1000).for_each(|(pos, _)| {
        let (src, dest) = itoc(pos);
        // println!("{:?} {:?}", pos, itoc(pos));
        connections.entry(src).and_modify(|v| v.push(dest)).or_insert_with(|| Vec::from([dest]));
        connections.entry(dest).and_modify(|v| v.push(src)).or_insert_with(|| Vec::from([src]));
    });

    let mut groups = vec![];
    let mut stack = vec![];
    // println!("{:?}", connections);
    while !connections.is_empty() {
        let top = *connections.keys().next().unwrap();
        // println!("{:?}", top);
        let mut size = 0;
        stack.push(top);
        while !stack.is_empty() {
            let pos = stack.pop().unwrap();
            let item = connections.remove(&pos);
            if item.is_none() { continue; }
            size += 1;
            for &v in item.unwrap().iter() {
                if connections.contains_key(&v) {
                    stack.push(v);
                }
            }
        }
        groups.push(size);
    }
    groups.sort();
    println!("{:?}", groups);
    println!("{:?}", groups.into_iter().rev().take(3).product::<usize>());
}

fn update_group(group_ids: &mut HashMap<usize, usize>, group_members: &mut HashMap<usize, HashSet<usize>>, old_group: usize, new_group: usize) {
    let group = group_members.remove(&old_group);
    if group.is_none() { return; }
    let group = group.unwrap();
    for mem in &group {
        *group_ids.get_mut(mem).unwrap() = new_group;
    }
    group_members.entry(new_group)
        .and_modify(|x| x.extend(&group))
        .or_insert(group);
}

fn part2(nodes: Vec<(i64, i64, i64)>) {
    let itoc = |c| (c % nodes.len(), c / nodes.len());

    let mut distance_matrix = vec![i64::MAX; sqr(nodes.len())];
    nodes.iter().enumerate().for_each(|(m, &(x, y, z))| {
        nodes.iter().enumerate().skip(m+1).for_each(|(n, (x_, y_, z_))| {
            distance_matrix[m * nodes.len() + n] = sqr(x - x_) + sqr(y - y_) + sqr(z - z_);
        })
    });

    let result = distance_matrix.iter().enumerate().sorted_unstable_by_key(|(_, d)| *d);

    let mut group_ids: HashMap<usize, usize> = HashMap::new();
    let mut group_members = HashMap::new();
    group_members.insert(0, HashSet::new());
    let last_insert = result.take_while_inclusive(|(pos, _)| {
        let (src, dest) = itoc(*pos);
        let curr_min = std::cmp::min(src, dest);
        let old_src_group = *group_ids.get(&src).unwrap_or(&src);
        let old_dest_group = *group_ids.get(&dest).unwrap_or(&dest);
        if old_src_group == old_dest_group {
            println!("Node {} and {} already belong to the same group {}", src, dest, old_src_group);
            return true;
        }
        let curr_min = std::cmp::min(curr_min, old_src_group);
        let curr_min = std::cmp::min(curr_min, old_dest_group);
        if curr_min < old_src_group {
            update_group(&mut group_ids, &mut group_members, old_src_group, curr_min);
        }
        if curr_min < old_dest_group {
            update_group(&mut group_ids, &mut group_members, old_dest_group, curr_min);
        }
        group_ids.insert(src, curr_min);
        group_ids.insert(dest, curr_min);
        let group = group_members.entry(curr_min).or_insert_with(HashSet::new);
        group.insert(src);
        group.insert(dest);

        println!("Add {} and {} to group {} with {} members", src, dest, curr_min, group.len());

        !(curr_min == 0 && group.len() == nodes.len())
    }).last().unwrap();

    let coord = itoc(last_insert.0);
    println!("Part 2 {:?} {:?} {:?} {:?} Answer: {}", last_insert, itoc(last_insert.0), nodes[coord.0], nodes[coord.1], nodes[coord.0].0 * nodes[coord.1].0);
}

fn main() {
     let result = io::stdin().lock().lines()
         .map(|line|
             line.unwrap()
                 .split(',')
                 .map(|x| x.parse().unwrap())
                 .collect_tuple()
                 .unwrap()
         ).collect();
    part1(&result);
    part2(result);
}