use std::collections::{HashMap, HashSet, VecDeque};
use std::io;

fn solve_links(links: &HashMap<String, Vec<String>>, start: String, end: &str) -> usize {
    let mut queue = VecDeque::new();
    // let mut seen = HashSet::new();
    queue.push_back(&start);
    let mut paths = 1;
    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        if curr == end {continue;}
        let nexts = &links.get(curr).expect(&format!("Key {} to exist", curr));
        paths += nexts.len() - 1;
        nexts.iter().for_each(|next| {
            // if !seen.contains(next) {
            //     seen.insert(next.to_string());
            // }
            queue.push_back(next);
        });
    }
    paths
}

fn solve_recurse<'a>(links: &'a HashMap<String, Vec<String>>, cache: &mut HashMap<&'a String, usize>, curr: &'a String, end: &str) -> usize {
    if curr == end {
        cache.insert(curr, 1);
        return 1;
    }
    if curr == "out" {
        return 0;
    }
    if cache.contains_key(curr) {
        return cache[curr];
    }

    let res = links.get(curr).expect(&format!("Key {} to exist", curr)).iter().fold(0, |acc, link| {
        acc + solve_recurse(links, cache, link, end)
    });
    cache.insert(curr, res);
    res
}


fn solve_links_fast(links: &HashMap<String, Vec<String>>, start: &str, end: &str) -> usize {
    let mut cache = HashMap::new();
    solve_recurse(links, &mut cache, &start.to_string(), end)
}


fn part1(links: &HashMap<String, Vec<String>>) {
    println!("Num paths {}", solve_links(links, "you".into(), "out"));
}

fn part2(links: HashMap<String, Vec<String>>) {
    let fft_dac = solve_links_fast(&links, "fft", "dac");
    let (first, second) = if fft_dac == 0 {
        ("dac", "fft")
    } else {
        ("fft", "dac")
    };

    let answer = solve_links_fast(&links, "svr", first)
    * solve_links_fast(&links, first, second)
    * solve_links_fast(&links, second, "out");


    println!("Num paths {}", answer);
}
fn main() {
    let links: HashMap<String, Vec<String>> = io::stdin().lines().map(|x| {
        let line = x.unwrap();
        let mut it = line.split(" ");
        let item = it.next().unwrap()[0..=2].to_string();
        let links = it.map(str::to_string).collect();
        (item, links)
    }).collect();
    // part1(&links);
    part2(links);
}