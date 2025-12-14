use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::fmt::Debug;
use std::io;
use std::io::BufRead;
use std::ops::{Add, Mul};
use itertools::{enumerate, Itertools};

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    target: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>
}

fn part1(input: &Vec<State>) {
    let answer = input.iter().fold(0, |x, state| {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        queue.push_back(Some(vec![false; state.target.len()]));
        queue.push_back(None);
        let mut layer = 0;
        loop {
            let front = queue.pop_front().unwrap();
            if front.is_none() {
                layer += 1;
                if queue.is_empty() { panic!("Never reached the solution"); }
                queue.push_back(None);
                continue;
            }
            let mut front = front.unwrap();
            for toggles in &state.buttons {
                toggles.iter().for_each(|b| {
                    front[*b] = !front[*b];
                });

                if front == state.target {
                    return x + layer + 1;
                }

                if seen.insert(front.clone()) {
                    queue.push_back(Some(front.clone()));
                }

                toggles.iter().for_each(|b| {
                    front[*b] = !front[*b];
                });
            }
        }
    });
    println!("Part 1: {}", answer);
}

fn part2_slow_loop(input: &Vec<State>, x: i64, state: &State) -> i64 {
    let mut queue = BinaryHeap::new();

    let biggest_button = state.buttons.iter().fold(0, |acc, button| std::cmp::max(acc, button.len() as i64));
    let g = (state.joltage.iter().sum::<usize>() as i64 + biggest_button - 1) / biggest_button;

    queue.push(Reverse((g, g, vec![0; state.joltage.len()])));
    let mut seen = HashSet::new();
    loop {
        let Reverse((cost, ttm, front)) = queue.pop().unwrap();
        let depth = cost - ttm;
        // println!("{:?} {:?} {:?} {:?}", front, cost, ttm, depth);
        'l:
        for toggles in &state.buttons {
            let cost = front.iter().zip(state.joltage.iter()).map(|(a, b)| *b as i64 - *a as i64).sum::<i64>();
            let cost_per_slam = toggles.len() as i64;
            let slam_that_button = cost / cost_per_slam;
            let mut slams = 1;
            while slams <= slam_that_button {
                let mut front = front.clone();
                for b in toggles {
                    front[*b] += slams as usize;
                    if front[*b] > state.joltage[*b] {
                        continue 'l;
                    }
                }

                let cost = (cost - slams * cost_per_slam + biggest_button - 1) / biggest_button;
                if cost == 0 {
                    println!("Solved at depth {}", depth + slams);
                    return x + depth + slams;
                }

                if seen.insert(front.clone()) {
                    queue.push(Reverse((cost + depth + slams, cost, front)));
                }

                slams *= 2;
            }
        }
    }
}

fn part2_slow(input: &Vec<State>) {
    let answer = input.iter().fold(0, |x, state| {
        println!("Next problem: {:?}, {:?}", x, state);
        part2_slow_loop(input, x, state)
    });
    println!("Part 2: {}", answer);
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

#[derive(Clone, Copy, Eq)]
struct Fraction {
    num: i64,
    denom: i64,
}

impl Fraction {

    fn one() -> Fraction {
        Fraction { num: 1, denom: 1 }
    }
    fn zero() -> Fraction {
        Fraction { num: 0, denom: 1 }
    }

    fn integer(value: i64) -> Fraction {
        Fraction { num: value, denom: 1 }
    }

    fn is_negative(&self) -> bool {
        self.num * self.denom < 0
    }
    fn simplify(mut self) -> Fraction {
        if self.denom == 0 { panic!("Divide by zero"); }
        if self.num == 0 { return Fraction::zero(); }
        if self.denom < 0 {
            self.num = -self.num;
            self.denom = -self.denom;
        }
        let div = gcd(self.num, self.denom);
        self.num /= div;
        self.denom /= div;
        self
    }
    fn reciprocal(self) -> Fraction {
        Fraction {
            num: self.denom,
            denom: self.num,
        }
    }
}

impl Debug for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.denom != 1 {
            write!(f, "{}/{}", self.num, self.denom)
        } else {
            write!(f, "{}", self.num)
        }
    }
}

impl std::ops::Mul for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: Self) -> Self::Output {
        Fraction {
            num: self.num * rhs.num,
            denom: self.denom * rhs.denom,
        }.simplify()
    }
}

impl std::ops::Div for Fraction {
    type Output = Fraction;

    fn div(self, rhs: Self) -> Self::Output {
        self.mul(rhs.reciprocal())
    }
}


impl std::ops::Add for Fraction {
    type Output = Fraction;
    fn add(self, rhs: Self) -> Self::Output {
        Fraction {
            num: self.num * rhs.denom + self.denom * rhs.num,
            denom: self.denom * rhs.denom,
        }.simplify()
    }
}

impl std::ops::Sub for Fraction {
    type Output = Fraction;
    fn sub(self, rhs: Self) -> Self::Output {
        self.add(Fraction {
            num: -rhs.num, denom: rhs.denom
        })
    }
}

impl std::cmp::PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.num * other.denom == self.denom * other.num
    }
}

impl std::cmp::PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.num * other.denom).partial_cmp(&(self.denom * other.num))
    }
}
impl std::cmp::Ord for Fraction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}


fn mul_row(row: &mut Vec<Fraction>, amount: Fraction) {
    row.iter_mut().for_each(|fraction| {
        *fraction = *fraction * amount;
    });
}

fn sub_rows(row: &mut Vec<Fraction>, other: &Vec<Fraction>) {
    row.iter_mut().zip(other.iter()).for_each(|(fraction, other)| {
        *fraction = *fraction - *other;
    })
}

fn solve_col(matrix: &mut Vec<Vec<Fraction>>, col: usize, row: usize) -> bool {
    assert!(matrix.len() >= row);
    let ones = matrix[row..].iter().position(|row| row[col] != Fraction::zero());
    if ones.is_none() { return false; } // This value is dependent
    let ones = row + ones.unwrap();
    matrix.swap(row, ones);
    let recip = matrix[row][col].reciprocal();
    mul_row(&mut matrix[row], recip);
    let ref_row = matrix[row].clone();

    matrix.iter_mut().enumerate().for_each(|(idx, fix_row)| {
        if idx == row { return; }
        let fix = fix_row[col];
        if fix == Fraction::zero() { return; }
        mul_row(fix_row, fix.reciprocal());
        sub_rows(fix_row, &ref_row);
        mul_row(fix_row, fix);
    });


    true
}

enum BruteError {
    NotInteger,
    TooBig,
}

fn brute_buttons(solution: &Vec<Vec<Fraction>>, buttons: &mut Vec<Option<i64>>, has_buttons: &Vec<Vec<(usize, i64)>>) -> Option<i64> {
    if !buttons.iter().contains(&None) {
        return Some(buttons.iter().map(|x| x.unwrap()).sum::<i64>());
    }

    // println!("Free buttons: {:?}", buttons);

    let positive_row_idx = has_buttons.iter().enumerate().filter_map(|(rowidx, x)| {
        if x.iter().all(|b| buttons[b.0].is_some() || b.1 > 0) // All items are positive or already fixed
            &&
            x.iter().any(|b| buttons[b.0].is_none()) // At least one item is not fixed
        {
            Some((x.iter().filter(|b| buttons[b.0].is_none()).count(), rowidx))
        } else {
            None
        }
    }).min();

    if positive_row_idx.is_none() {
        // println!("No positive rows {:?} {:?}", buttons, has_buttons);
        return None;
    }
    let (num_free, positive_row_idx) = positive_row_idx.unwrap();

    let positive_row = &has_buttons[positive_row_idx];
    let first_item_in_row = positive_row.iter().find(|b| buttons[b.0].is_none());
    if first_item_in_row.is_none() { panic!("Expected a row with a free variable"); return None; }
    let first_item_in_row = first_item_in_row.unwrap();

    let target_sum = positive_row.iter().fold(solution[positive_row_idx].last().unwrap().num, |acc, b| {
        match buttons[b.0] {
            Some(x) => acc - b.1 * x,
            None => acc,
        }
    });

    if target_sum < 0 {
        // println!("Target already exceeded by previous decisions");
        return None;
    }
    // println!("Target sum for row {:?}: {}", positive_row_idx, target_sum);

    let max_presses = target_sum / first_item_in_row.1;


    if num_free == 1 {
        // println!("Direct compute {} with {} presses ({} / {})", first_item_in_row.0, max_presses, target_sum, first_item_in_row.1);
        if target_sum % first_item_in_row.1 != 0 {
            // println!("Bad alignment");
            return None;
        }
        buttons[first_item_in_row.0] = Some(max_presses);
        let res = brute_buttons(solution, buttons, has_buttons);
        buttons[first_item_in_row.0] = None;
        return res;
    }

    // println!("Line has other free variables. Iterating {:?} 0..{:?}", first_item_in_row.0, max_presses);
    let mut fewest = None;
    for b_val in 0..=max_presses {
        buttons[first_item_in_row.0] = Some(b_val);
        let res = brute_buttons(solution, buttons, has_buttons);
        fewest = match(fewest, res) {
            (None, x) | (x, None) => x,
            (Some(x), Some(y)) => Some(std::cmp::min(x, y)),
        }
    }
    buttons[first_item_in_row.0] = None;

    // println!("Fewest from iterating {:?} 0..{:?}: {:?}", first_item_in_row.0, max_presses, fewest);
    fewest


    // has_buttons.retain(|row| {
    //     row.iter().any(|b| buttons[b.0].is_none())
    // } );
}

fn search_solution(solution: &Vec<Vec<Fraction>>) -> Option<i64> {
    let mut buttons = vec![None; solution[0].len() - 1];
    let mut has_buttons: Vec<Vec<_>> = solution.iter().map(|row| {
        row[0..row.len()-1].iter().enumerate().filter_map(|(idx, fraction)| if fraction.num != 0 { Some((idx, fraction.num)) } else { None } ).collect_vec()
    }).collect();

    if has_buttons.iter().all(|x| x.len() <= 1) {
        println!("Trivial: {:?}", solution);
        return Some(solution.iter().map(|row| row.last().unwrap().num).sum::<i64>());
    }

    has_buttons.iter().enumerate().for_each(|(row, button)| {
        if button.len() == 1 {
            buttons[button[0].0] = Some(solution[row].last().unwrap().num);
        }
    });
    // has_buttons.retain(|row| {
    //     row.len() > 1 && row.iter().any(|b| buttons[b.0].is_none())
    // } );

    brute_buttons(&solution, &mut buttons, &has_buttons)
}

fn part2_fast(input: &Vec<State>) {
    let answer = input.iter().fold(0i64, |x, state| {
        let mut matrix = vec![vec![Fraction::zero(); state.buttons.len()+1]; state.joltage.len()];
        state.buttons.iter().enumerate().for_each(|(i, lights)| {
            lights.iter().for_each(|j| {
                matrix[*j][i] = Fraction::one();
            });
        });
        matrix.iter_mut().enumerate().for_each(|(i, light_row)| {
            *light_row.last_mut().unwrap() = Fraction::integer(state.joltage[i] as i64);
        });
        // let matrix = matrix.iter().zip(0usize..).sorted().collect_vec();
        matrix.sort();
        matrix.reverse();
        // println!("Buttons: {:?}", state.buttons);
        // println!("Matrix: {:?}", matrix);

        let mut rows = 0;
        for i in 0..state.buttons.len() {
            if solve_col(&mut matrix, i, rows) {
                rows += 1;
            }
        }

        matrix.iter_mut().for_each(|row| {
            let denom = row.iter().map(|x| x.denom.abs()).reduce(lcm).unwrap();
            if denom != 1 {
                mul_row(row, Fraction::integer(denom));
            }
        });

        println!("Solution:");
        matrix.iter().for_each(|row| {
            println!("{:?}", row);
        });
        let sol = search_solution(&matrix);
        let res = x + sol.unwrap();
        println!("Result: {}", res);
        res
    });
    println!("Part 2: {}", answer);
}

fn main() {
    let input = io::stdin().lock().lines().map(|x| {
        let x = x.unwrap();
        let mut parts = x.split(' ');
        let target_str = parts.next().unwrap();
        let target = target_str.chars().skip(1).take_while(|x| *x != ']').map(|x| x == '#').collect();

        let mut buttons = Vec::new();
        loop {
            let next = parts.next();
            if next.is_none() { panic!("No joltage"); }
            let next = next.unwrap();
            let items = next[1..next.len() - 1].split(',').map(|x| x.parse::<usize>().unwrap()).collect_vec();
            if next.chars().nth(0) == Some('{') {
                buttons.sort_by_key(|a: &Vec<usize>| a.len());
                return State {
                    target,
                    buttons,
                    joltage: items
                }
            }
            buttons.push(items);
        }
    }).collect_vec();

    part1(&input);
    // part2(&input);
    part2_fast(&input);
}