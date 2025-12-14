use std::cmp::Ordering;
use std::collections::{BTreeSet};
use std::io;
use std::io::BufRead;
use std::iter::once;
use std::ops::Bound::Included;
use itertools::Itertools;
use crate::Direction::Undefined;

fn part1(lines: &Vec<(i64, i64)>) {
    // let mut left_bests = Vec::<(i64, i64)>::new();
    // let mut right_bests = Vec::<(i64, i64)>::new();
    // let insert = |layer, item: Option<usize>, best: &mut Vec<(i64, i64)>, cmp| {
    //     if item.is_some() {
    //         let item = item.unwrap() as i64;
    //         best.push((layer, item));
    //     }
    // };

    // lines.iter().zip(0i64..).for_each(|(line, layer)| {
    //     let left = line.find('#');
    //     let right = line.rfind('#');
    //
    //     insert(layer, left, &mut left_bests, std::cmp::Ordering::Less);
    //     insert(layer, right, &mut right_bests, std::cmp::Ordering::Greater);
    // });

    // println!("Left {:?}", left_bests);
    // println!("Right {:?}", right_bests);

    let result = lines.iter().fold(0i64, |acc, posl| {
        lines.iter()
            .filter(|posr| posr.1 >= posl.1)
            .fold(acc, |acc, posr| {
                std::cmp::max(acc, ((posl.0 - posr.0).abs() + 1) * (posr.1 - posl.1 + 1))
            })
    });
    println!("Part 1 {:?}", result);
}


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
    Undefined
}
fn invert(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::South,
        Direction::East => Direction::West,
        Direction::South => Direction::North,
        Direction::West => Direction::East,
        _ => Direction::Undefined,
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Line {
    dir: Direction,
    startx: i64,
    starty: i64,
    endx: i64,
    endy: i64,
}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Line) -> Option<Ordering> {
        // Broken if we compared Vertical to Horizontal
        let cmp_dir = match (self.dir, other.dir) {
            (Direction::Undefined, Direction::Undefined) => return None,
            (Direction::Undefined, x) | (x, Direction::Undefined) => x,
            (x, _) => x,
        };
        if cmp_dir == Direction::North || cmp_dir == Direction::South {
            Some((self.startx, self.starty, self.endy).cmp(&(other.startx, other.starty, other.endy)))
        } else {
            Some((self.starty, self.startx, self.endx).cmp(&(other.starty, other.startx, other.endx)))
        }
    }
}

impl Ord for Line {
    fn cmp(&self, other: &Line) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
// struct Rectangle {
//     start: i64,
//     end: i64,
//     y: i64,
// }


fn part2(lines: &mut Vec<(i64, i64)>) {
    let mut wrap = 0;
    let parse_line = lines.windows(2)
        .chain(once(&[*lines.last().unwrap(), *lines.first().unwrap()][0..2]))
        .scan(Direction::Undefined, |last_dir, item| {
            let curr = item[0];
            let next = item[1];
            let mut turn = |old_for_left_turn, new_dir| {
                println!("Turning {:?} to {:?} (checking {:?})", last_dir, new_dir, old_for_left_turn);
                if *last_dir == old_for_left_turn {
                    wrap += 1;
                } else {
                    wrap -= 1;
                }
                *last_dir = new_dir;
            };
            if curr.0 < next.0 {
                turn(Direction::South, Direction::East);
            } else if curr.0 > next.0 {
                turn(Direction::North, Direction::West);
            } else if curr.1 < next.1 {
                turn(Direction::West, Direction::South);
            } else if curr.1 > next.1 {
                turn(Direction::East, Direction::North);
            } else {
                panic!();
            }

            Some(Line{
                dir: last_dir.clone(),
                startx: std::cmp::min(curr.0, next.0),
                starty: std::cmp::min(curr.1, next.1),
                endx: std::cmp::max(curr.0, next.0),
                endy: std::cmp::max(curr.1, next.1),
            })
        }).collect_vec();

    let (vertical, horizontal): (BTreeSet<Line>, BTreeSet<Line>) = parse_line.iter().partition(|x| x.dir == Direction::North || x.dir == Direction::South);

    let clockwise = wrap < 0;
    let (open_vert, open_hor, close_vert, close_hor) = if clockwise {
        (Direction::East, Direction::North, Direction::West, Direction::South)
    } else {
        (Direction::West, Direction::South, Direction::East, Direction::North)
    };

    let is_valid_rect = |lidx: usize, posl: &(i64, i64), posr: &(i64, i64)| {
        let mut next_dir = parse_line[lidx].dir;
        let mut last_dir = parse_line[(lidx + parse_line.len() - 1) % parse_line.len()].dir;
        if !clockwise {
            let tmp = invert(last_dir);
            last_dir = invert(next_dir);
            next_dir = tmp;
        }

        let left = posl.0 > posr.0;
        let right = posl.0 < posr.1;
        let up = posl.1 > posr.1;
        let down = posl.1 < posr.1;


        match (last_dir, next_dir) {
            (Direction::North, Direction::West) => {
                if left && down {
                    return false;
                }
            },
            (Direction::North, Direction::East) => {
                if !(right && down) {
                    return false;
                }
            },
            (Direction::South, Direction::East) => {
                if right && up {
                    return false;
                }
            }
            (Direction::South, Direction::West) => {
                if !(left && up) {
                    return false;
                }
            }
            (Direction::West, Direction::North) => {
                if !(right && up) {
                    return false;
                }
            },
            (Direction::West, Direction::South) => {
                if right && down {
                    return false;
                }
            }
            (Direction::East, Direction::North) => {
                if left && up {
                    return false;
                }
            },
            (Direction::East, Direction::South) => {
                if !(left && down) {
                    return false;
                }
            }
            _ => panic!("{:?} {:?} {:?} {:?} {:?}", lidx, (lidx + parse_line.len() - 1) % parse_line.len(), parse_line.len(), next_dir, last_dir),
        }

        let dummy_start = Line{
            dir: Undefined,
            startx: std::cmp::min(posl.0, posr.0),
            starty: std::cmp::min(posl.1, posr.1),
            endx: std::cmp::min(posl.0, posr.0),
            endy: std::cmp::min(posl.1, posr.1),
        };
        let dummy_end = Line{
            dir: Undefined,
            startx: std::cmp::max(posl.0, posr.0),
            starty: std::cmp::max(posl.1, posr.1),
            endx: std::cmp::max(posl.0, posr.0),
            endy: std::cmp::max(posl.1, posr.1),
        };

        // TODO doesn't work if there is a gap of 0 between two lines
        let cross_vert = vertical.range((Included(dummy_start), Included(dummy_end))).any(|line| {
            (line.startx > dummy_start.startx && line.endx < dummy_end.endx) // Line is inside the danger zone
            &&
            (line.endy > dummy_start.starty && line.starty < dummy_end.endy) // Line overlaps with the danger zone
        });
        if cross_vert {
            return false;
        }

        let cross_horiz = horizontal.range((Included(dummy_start), Included(dummy_end))).any(|line| {
            (line.starty > dummy_start.starty && line.endy < dummy_end.endy) // Line is inside the danger zone
                &&
                (line.endx > dummy_start.startx && line.startx < dummy_end.endx) // Line overlaps with the danger zone
        });

        !cross_horiz
    };

    let result = lines.iter().enumerate().fold(0i64, |acc, (lidx, posl)| {
        lines.iter()
            .filter(|posr| is_valid_rect(lidx, posl, posr))
            .fold(acc, |acc, posr| {
                std::cmp::max(acc, ((posl.0 - posr.0).abs() + 1) * ((posr.1 - posl.1).abs() + 1))
            })
    });

    println!("Result is {:?}", result);




}


fn main() {
    let mut lines = io::stdin().lock().lines().map(|x| x.unwrap().split(',').map(|x| x.parse().unwrap()).collect_tuple().unwrap()).collect();
    part1(&lines);
    part2(&mut lines);
}