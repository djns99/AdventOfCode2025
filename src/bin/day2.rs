use std::io;

fn check_reversible(value: u64) -> bool {
    let allowed_chunks = [2, 3, 5, 7, 11]; // Part 1
    let digits = (value * 10).ilog10();
    'chunk: for chunks in allowed_chunks {
        if digits % chunks != 0 || chunks > digits {
            continue;
        }
        let chunk_size = digits / chunks;
        let chunk_div = 10u64.pow(chunk_size);
        let mut curr = value;
        for _ in 1..chunks {
            let next = curr / chunk_div;
            if curr % chunk_div != next % chunk_div {
                continue 'chunk;
            }
            curr = next;
        }
        return true;
    }

    false
}

fn part1(ranges_to_check: Vec<(u64, u64)>) {
    let answer = ranges_to_check.iter().fold(0, |acc, &(start, end)| {
        let values = start..=end;
        values.fold(acc, |acc, v| {
            if check_reversible(v) {
                acc + v
            } else {
                acc
            }
        })
    });

    println!("Result is {:?}", answer);
}

fn main() {
    let ranges = io::stdin().lines().next().unwrap().unwrap().split(',').map(|x| {
        let mut it = x.split('-');
        let start = it.next().unwrap().parse().expect("could not parse start of range");
        let end = it.next().unwrap().parse().expect("could not parse end of range");
        (start, end)
    }).collect();
    part1(ranges);
}