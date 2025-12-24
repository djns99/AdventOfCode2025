use std::cmp::Ordering;
use std::collections::{BTreeSet, HashSet};
use std::hash::{Hash, Hasher};
use std::io::stdin;
use std::iter;
use std::rc::Rc;
use itertools::{iproduct, Itertools};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Shape {
    items: Vec<Vec<bool>>,
    unique_orientations: Vec<u32>,
    dot_count: u32,
    idx: usize
}

fn lookup_impl(shape_items: &Vec<Vec<bool>>, orientation: u32, x: usize, y: usize) -> bool {
    match orientation {
        0 => shape_items[y][x],
        1 => shape_items[y][2-x],
        2 => shape_items[2-y][x],
        3 => shape_items[2-y][2-x],
        4 => shape_items[x][y],
        5 => shape_items[x][2-y],
        6 => shape_items[2-x][y],
        7 => shape_items[2-x][2-y],
        _ => false,
    }
}

fn lookup(shape: OrientedShape, x: usize, y: usize) -> bool {
    lookup_impl(&shape.1.items, shape.0, x, y)
}

impl Shape {
    fn pprint(&self, orientation: u32) -> String {
        (0..3).map(|y| (0..3).map(|x| if lookup_impl(&self.items, orientation, x, y) { '#' } else { '.' }).collect::<String>()).join("\n")
    }
}

impl Hash for Shape {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.idx);
    }
}

type OrientedShape<'a> = (u32, &'a Shape, usize);

fn place(grid: &mut Vec<Vec<bool>>, pos: (usize, usize), shape: OrientedShape) -> bool {
    if ! iproduct!(0..3, 0..3).any(|(dx, dy)| {
        lookup(shape, dx, dy) && grid[pos.0 + dy][pos.1 + dx]
    }) {
        iproduct!(0..3, 0..3).for_each(|(dx, dy)| {
            if lookup(shape, dx, dy) {
                grid[pos.0 + dy][pos.1 + dx] = true;
            }
        });
        true
    } else {
        false
    }
}

fn unplace(grid: &mut Vec<Vec<bool>>, pos: (usize, usize), shape: OrientedShape)  {
    iproduct!(0..3, 0..3).for_each(|(dx, dy)| {
        if lookup(shape, dx, dy) {
            grid[pos.0 + dy][pos.1 + dx] = false;
        }
    });
}

fn pprint(grid: &Vec<Vec<bool>>) {
    for row in grid {
        for cell in row {
            print!("{}", if *cell { '#' } else { '.' });
        }
        println!();
    }
}
// type AllowedGrid<'a> = Vec<Vec<Vec<OrientedShape<'a>>>>;
type AllowList<'a> = HashSet<OrientedShape<'a>>;
type AllowedGrid<'a> = Vec<Vec<Rc<AllowList<'a>>>>;

#[derive(Debug, Clone)]
struct AllowedQueueEntry<'a>(Rc<AllowList<'a>>, usize, usize);

impl PartialEq for AllowedQueueEntry<'_> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0) && self.1 == other.1 && self.2 == other.2
    }
}
impl Eq for AllowedQueueEntry<'_> {}
impl<'a> PartialOrd for AllowedQueueEntry<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.0.len(), self.1, self.2).cmp(&(self.0.len(), other.1, other.2)))
    }
}

impl<'a> Ord for AllowedQueueEntry<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

type AllowState<'a, 'b, 'c> = &'b mut (&'c mut AllowedGrid<'a>, BTreeSet<AllowedQueueEntry<'a>>);

// fn sort_allowed(allowed_grid: &mut AllowedGrid) {
//     allowed_grid.sort_by(|a, b| (a.2.len(), a.0, a.1).cmp(&(b.2.len(), b.0, b.1)))
// }

type ShapeCombinations<'a> = Vec<[AllowList<'a>; 25]>;

fn recurse<'a, 'b, 'c>(grid: &mut Vec<Vec<bool>>, allowed: AllowState<'a, 'b, 'c>, shapes: &Vec<Shape>, combos: &Vec<[AllowList<'a>; 25]>, problems: &mut [u32; 6], calls: &mut u64) -> bool {
    // println!();
    // pprint(grid);
    // println!();
    let first_iter = *calls < 4;
    *calls += 1;
    // println!("recurse({}, {}) with problems {:?}", pos.0, pos.1, problems);
    let remaining_dot_count = problems.iter().enumerate().fold(0, |acc, (idx, x)| {
        acc + x * shapes[idx].dot_count
    });
    return remaining_dot_count as usize <= (grid[0].len() * grid.len());
    if remaining_dot_count == 0 {
        return true;
    }
    let w = grid[0].len();
    let h = grid.len();

    if allowed.1.is_empty() {
        // println!("No spaces left");
        return false;
    }

    let mut next = allowed.1.pop_first().unwrap();
    while next.0.is_empty() {
        next = allowed.1.pop_first().unwrap();
    }
    let y = next.1;
    let x = next.2;

    let local_allow_list = (*allowed.0[y][x]).clone();
    // println!("next: {:?} with allowed {:?}", next, local_allow_list);
    for shape in local_allow_list {
        if problems[shape.1.idx] == 0 {
            continue;
        }
        if first_iter {
            println!("Up to {} calls, processing shape {:?}", calls, shape);
        }

        problems[shape.1.idx] -= 1;
        if ! place(grid, (y, x), shape) {
            pprint(grid);
            panic!("Could not place shape at {:?}, {:?}:\n{} ", y, x, shape.1.pprint(shape.0));
        }

        // println!("Placed shape:");
        // println!("{}", shape.1.pprint(shape.0));
        // println!("New Grid:");
        // pprint(grid);

        let mut old_neighbours = vec![None; combos[shape.2].len()];
        for (idx, combo) in combos[shape.2].iter().enumerate() {
            let dy = idx % 5;
            let dx = idx / 5;
            if x + dx < 2 || x + dx >= (w + 2) || y + dy < 2 || y + dy >= (h + 2) {
                // println!("Skipping {} + {}, {} + {}", x, dx, y, dy);
                continue;
            }
            let nx = x + dx - 2;
            let ny = y + dy - 2;

            let exists = allowed.1.remove(&AllowedQueueEntry(Rc::clone(&allowed.0[ny][nx]), ny, nx));
            if ! exists {
                continue;
            }

            old_neighbours[idx] = Some(Rc::clone(&allowed.0[ny][nx]));
            let new_intersection: AllowList = allowed.0[ny][nx].intersection(combo).map(|x| x.clone()).collect();
            // println!("Pos {} {} can place {} shapes:", nx, ny, new_intersection.len());
            // new_intersection.iter().for_each(|s| println!("{}", s.1.pprint(s.0)));
            // println!("Used to have {}", old_neighbours.last().unwrap().len());
            // println!("Filtered to {}:", combo.len());
            // combo.iter().for_each(|s| println!("{}", s.1.pprint(s.0)));

            allowed.0[ny][nx] = Rc::new(new_intersection);
            if allowed.0[ny][nx].len() > 0 {
                allowed.1.insert(AllowedQueueEntry(Rc::clone(&allowed.0[ny][nx]), ny, nx));
            }
        }

        if recurse(grid, allowed, shapes, combos, problems, calls) {
            return true;
        }

        for idx in 0..combos[shape.1.idx].len() {
            let dy = idx % 5;
            let dx = idx / 5;
            if x + dx < 2 || x + dx >= (w + 2) || y + dy < 2 || y + dy >= (h + 2) {
                continue;
            }
            let nx = x + dx - 2;
            let ny = y + dy - 2;

            if old_neighbours[idx].is_none() {
                continue;
            }

            allowed.1.remove(&AllowedQueueEntry(Rc::clone(&allowed.0[ny][nx]), ny, nx));
            allowed.0[ny][nx] = old_neighbours[idx].clone().unwrap();
            allowed.1.insert(AllowedQueueEntry(Rc::clone(&allowed.0[ny][nx]), ny, nx));
        }

        unplace(grid, (y, x), shape);
        problems[shape.1.idx] += 1;
    }

    // No problem solved, just disable placing a token here
    // println!("No place recurse: {:?}", allowed.1);
    if recurse(grid, allowed, shapes, combos, problems, calls) {
        return true;
    }
    allowed.1.insert(next);

    false
}

fn all_orientations(shapes: &Vec<Shape>) -> impl Iterator<Item=OrientedShape> + Clone {
    shapes.iter().flat_map(|shape| shape.unique_orientations.iter().zip(iter::repeat(shape)).map(|(&x, y)| (x, y))).enumerate().map(|(i, s)| (s.0, s.1, i))
}

fn combine_shapes(shapes: &Vec<Shape>) -> ShapeCombinations {
    all_orientations(shapes)
    .map(|first_shape: OrientedShape|
        iproduct!(-2..=2, -2..=2)
        .map(|(dx, dy)| {
            all_orientations(shapes)
            .filter_map(|overlap_shape: OrientedShape| {
                let overlap_x = std::cmp::max(dx, 0)..std::cmp::min(dx+3, 3);
                let overlap_y = std::cmp::max(dy, 0)..std::cmp::min(dy+3, 3);
                if iproduct!(overlap_x, overlap_y).all(|(x, y)| {
                    let dx = (x - dx) as usize;
                    let dy = (y - dy) as usize;
                    let x = x as usize;
                    let y = y as usize;
                    !lookup(first_shape, x, y) || !lookup(overlap_shape, dx, dy)
                }) {
                    // println!("Shape at offset {:?} can overlap at orienation {} and {}\n{}\n\n{} ", (dx, dy), overlap_shape.0, first_shape.0, overlap_shape.1.pprint(overlap_shape.0), first_shape.1.pprint(first_shape.0));
                    Some(overlap_shape)
                } else {
                    None
                }
            }).collect()
        }).collect_array().unwrap()
    ).collect_vec()
}

fn part1(shapes: &Vec<Shape>, problems: &Vec<((usize, usize), [u32; 6])>) {
    let mut count = 0;

    let valid_combinations = combine_shapes(shapes);


    for ((w, h), presents) in &mut problems.clone()[0..] {
        let combinations: AllowList = all_orientations(shapes).collect();
        let mut allowed_parts = (0..*h)
            .map(|y|
                (0..*w)
                    .map(|x|
                        if y >= *h - 2 || x >= *w - 2 {
                            Rc::new(AllowList::new())
                        } else {
                            Rc::new(combinations.clone())
                        })
                    .collect_vec()
            ).collect_vec();
        let parts_tree =
            iproduct!(0..*h, 0..*w)
                .filter_map(|(y, x)| {
                    let v = allowed_parts[y][x].clone();
                    if v.is_empty() || y >= *h - 2 || x >= *w - 2 { None } else { Some(AllowedQueueEntry(v, y, x)) }
                }).collect();
        let mut allow_state = (&mut allowed_parts, parts_tree);
        if recurse(&mut vec![vec![false; *w]; *h], &mut allow_state, shapes, &valid_combinations, presents, &mut 0) {
            count += 1;
        }
    }
    println!("Part 1: {}", count);
}

fn main() {

    // for i in 0..=31 {
    //     println!("{}:\t{:?}\t\t{:?}", i, unswizzle(i, 4, 8), unswizzle(i, 8, 4));
    // }
    // return;

    let chunks = stdin().lines().map(Result::unwrap)
        .chunk_by(|x| {
            x != ""
        });
    let mut chunks = chunks.into_iter()
        .filter_map(|(x, item)| {
            if x {
                Some(item)
            } else {
                None
            }
        });
    let shapes = chunks.by_ref().take(6)
    .map(|item| {
        item.skip(1).map(|x| x.chars().map(|c| c == '#').collect_vec()).collect_vec()
    }).enumerate()
    .map(|(idx, items)| {
        let unique_orientations = (0..8).filter(|&new_orientation| {
            ! (0..new_orientation).any(|old_orientation| {
                iproduct!(0..3, 0..3).all(|(dx, dy)| {
                    lookup_impl(&items, old_orientation, dx, dy) == lookup_impl(&items, new_orientation, dx, dy)
                })
            })
        }).collect_vec();
        let dot_count = items.iter().flatten().fold(0, |acc, x| if *x { acc + 1 } else { acc });
        Shape {
            items,
            unique_orientations,
            dot_count,
            idx
        }
    }).collect_vec();

    let problems = chunks.next().unwrap().map(|item| {
        let mut parts = item.split(" ");
        let dim = parts.next().unwrap();
        let dim: (usize, usize) = dim[0..dim.len()-1].split("x").map(|dim| {dim.parse().expect(&format!("Expected {} to be an int", dim))}).take(2).collect_tuple().unwrap();
        let shape_counts = parts.map(|dim| {dim.parse().unwrap()}).collect_array().unwrap();
        (dim, shape_counts)
    }).collect_vec();

    println!("Part 1: {:?}\n{:?}", shapes, problems);
    part1(&shapes, &problems);
}