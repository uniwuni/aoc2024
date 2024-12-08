use std::io::{self, BufRead};
use itertools::Itertools;
use multimap::MultiMap;
use std::collections::BTreeSet;

fn is_valid(max_x: isize, max_y: isize, x: isize, y: isize) -> bool {
    x >= 0 && y >= 0 && x < max_x && y < max_y
}

fn compute_antinodes(data: &MultiMap<char, (isize, isize)>, max_x: isize, max_y: isize) -> BTreeSet<(isize, isize)> {
    let mut set: BTreeSet<(isize, isize)> = BTreeSet::new();
    for (_,v) in data.iter_all()  {
        for ((x1,y1), (x2,y2)) in v.iter().tuple_combinations::<(_,_)>() {
            let (dx,dy): (isize, isize) = (x2 - x1, y2 - y1);
            if is_valid(max_x, max_y, x2 + dx, y2 + dy) {
                set.insert((x2 + dx, y2 + dy));
            }
            if is_valid(max_x, max_y, x1 - dx, y1 - dy) {
                set.insert((x1- dx, y1 - dy));
            }
        }   
    }
    set
}

fn compute_antinodes2(data: &MultiMap<char, (isize, isize)>, max_x: isize, max_y: isize) -> BTreeSet<(isize, isize)> {
    let mut set: BTreeSet<(isize, isize)> = BTreeSet::new();
    for (_,v) in data.iter_all()  {
        for (&(x1,y1), &(x2,y2)) in v.iter().tuple_combinations::<(_,_)>() {
            let (dx,dy): (isize, isize) = (x2 - x1, y2 - y1);
            let mut newx = x2;
            let mut newy = y2;
            while is_valid(max_x, max_y, newx, newy) {
                set.insert((newx, newy));
                newx += dx;
                newy += dy;
            }
            newx = x1;
            newy = y1;
            while is_valid(max_x, max_y, newx, newy) {
                set.insert((newx, newy));
                newx -= dx;
                newy -= dy;
            }
        }   
    }
    set
}




fn compute(data: &MultiMap<char, (isize, isize)>, max_x: isize, max_y: isize) -> (usize, usize) {
    (compute_antinodes(data, max_x, max_y).len(), compute_antinodes2(data, max_x, max_y).len())
}
fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let lines2 = lines.peekable();
    let mut maxx = 0;
    let mut maxy = 0;
    let data: MultiMap<char, (isize,isize)> =
        lines2.enumerate().map(|(y,a)| {
            let a = a.unwrap();
            maxx = a.len() as isize;
            let chrs = a.char_indices();
            maxy = y as isize;
            chrs.filter_map(|(x,c)| {
                if c.is_alphanumeric() {
                    Some((c,(x as isize,y as isize)))
                } else {
                    None
                }

            }).collect::<Vec<_>>()
        }).flatten().collect();

    use std::time::Instant;
    let now = Instant::now();

    let (r1, r2) = compute(&data, maxx, maxy + 1);
    let elapsed = now.elapsed();
    println!("{} {}", r1, r2);

    println!("Elapsed: {:.2?}", elapsed);
}
