use std::io::{self, Read};
use itertools::Itertools;
#[derive(Debug, Clone, Copy)]
struct Entry {
    xa: i128, 
    ya: i128,
    xb: i128,
    yb: i128,
    xtarget: i128,
    ytarget: i128
}

fn solve(e: Entry) -> Option<(i128, i128)> {
    let det = e.xa * e.yb - e.ya * e.xb;
    if det != 0 {
        let det1 = e.xtarget * e.yb  - e.ytarget * e.xb;
        let det2 = e.xa * e.ytarget - e.ya * e.xtarget;
        if det1 % det == 0 && det2 % det == 0 {
            let t1 = det1/det;
            let t2 = det2/det;
            if t1 >= 0 && t2 >= 0 {
                //println!("{:?} works", e);
                Some((t1,t2))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        println!("lindep");
        None
    }
}

fn compute<A>(parsed: A) -> i128
    where A: Iterator<Item = Entry> {
    parsed.filter_map(solve).map(|(a,b)| 3 * a + b).sum()
}

fn main() {
    let mut input = String::new();
    
    let _ = io::stdin().read_to_string(&mut input);
    let parsed=input.split("\n\n").map(|a|
        { let v = a.split(|x| x == ',' || x == '+' || x == '=' || x == '\n').collect::<Vec<_>>();
           Entry {
                xa: v[1].parse().unwrap(),
                ya: v[3].parse().unwrap(),
                xb: v[5].parse().unwrap(),
                yb: v[7].parse().unwrap(),
                xtarget: v[9].parse().unwrap(),
                ytarget: v[11].parse().unwrap()
           }});
    
    use std::time::Instant;
    let now = Instant::now();
    //println!("{:?}", parsed.map(solve).collect_vec());
    let r1 = compute(parsed.clone());
    let r2 = compute(parsed.map(|e| Entry { xtarget: 10000000000000 + e.xtarget, ytarget: 10000000000000 + e.ytarget, ..e}));
    let elapsed = now.elapsed();
    println!("{} {}", r1, r2);

    println!("Elapsed: {:.2?}", elapsed);
}