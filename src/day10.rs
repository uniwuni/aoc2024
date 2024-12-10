use std::collections::BTreeSet;
use std::io::{self, Read};
use std::iter::once;

fn compute(
    parsed: Vec<Vec<(u32, BTreeSet<(usize, usize)>, Vec<(usize, usize)>)>>,
) -> (usize, usize) {
    let mut parsed = parsed;
    for y in 0..parsed.len() {
        for x in 0..parsed[0].len() {
            if parsed[y][x].0 == 9 {
                parsed[y][x].1 = once((y, x)).collect();
                parsed[y][x].2 = once((y, x)).collect();
            }
        }
    }
    for i in (0..9).rev() {
        for y in 0..parsed.len() {
            for x in 0..parsed[0].len() {
                if parsed[y][x].0 == i {
                    let mut neighbor: Vec<(usize, usize)> = vec![];
                    for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        //println!("{} {} {} {}", y, x, (y as isize + dy) as usize, (x as isize + dx) as usize);
                        if let Some(vec) = parsed.get((y as isize + dy) as usize) {
                            if let Some(point) = vec.get(((x as isize) + dx) as usize) {
                                //      println!("{:?}", point);
                                if point.0 == i + 1 {
                                    neighbor.extend(&point.2);
                                }
                            }
                        }
                    }
                    parsed[y][x].1 = neighbor.clone().into_iter().collect();
                    parsed[y][x].2 = neighbor;
                }
            }
        }
    }
    let mut res1 = 0;
    let mut res2 = 0;
    for y in 0..parsed.len() {
        for x in 0..parsed[0].len() {
            if parsed[y][x].0 == 0 {
                res1 += parsed[y][x].1.len();
                res2 += parsed[y][x].2.len();
            }
        }
    }
    (res1, res2)
}
fn main() {
    let mut input = String::new();

    let _ = io::stdin().read_to_string(&mut input);
    use std::time::Instant;
    let parsed: Vec<Vec<(u32, BTreeSet<(_, _)>, Vec<_>)>> = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|a| (a.to_digit(10).unwrap(), BTreeSet::new(), vec![]))
                .collect()
        })
        .collect();
    let now = Instant::now();
    let (r1, r2) = compute(parsed);
    let elapsed = now.elapsed();
    println!("{} {}", r1, r2);

    println!("Elapsed: {:.2?}", elapsed);
}
