use std::{collections::HashMap, io::{self, Read}, iter::once};
use itertools::{repeat_n, Itertools};
#[derive(Debug, Clone, Copy, PartialEq,Eq)]
struct Entry {
    xp: i64, 
    yp: i64,
    xv: i64,
    yv: i64
}

fn go(e: &Entry, dt: i64, width: i64, height: i64) -> Entry {
    let dt = dt.rem_euclid(width * height);
    Entry {xp: (e.xp + (e.xv * dt)).rem_euclid(width), yp: (e.yp + (e.yv * dt)).rem_euclid(height), xv: e.xv, yv: e.yv}
}

fn compute<A>(parsed: A) -> (usize, usize)
    where A: Iterator<Item = Entry> + Clone {
    let width = 101;//101;
    let height =103;// 103;
    let dt = 100;
    let mut v = parsed.clone().collect_vec();
    let v2 = parsed.clone().collect_vec();
    let pos = parsed.map(|e| go(&e, dt, width, height));

    let lu = pos.clone().filter(|e| e.xp < width/2 && e.yp < height/2).count();
    let ru = pos.clone().filter(|e| e.xp > width/2 && e.yp < height/2).count();
    let ld = pos.clone().filter(|e| e.xp < width/2 && e.yp > height/2).count();
    let rd = pos.clone().filter(|e| e.xp > width/2 && e.yp > height/2).count();
    for i in 0..11000 {
        let (stri,t) = pretty_print(&v, width, height);
        if t {
            println!("{}\n\n\n\n\n", i);
            println!("{}", stri);
        }
        for e in &mut v {
            *e = go(e,1, width, height);
        }
    }
    //println!("{:?}", pos.collect_vec());
    (lu * ru * ld * rd, 0)
}

fn pretty_print(v: &[Entry], width: i64, height: i64) -> (String,bool) {
    let mut a = repeat_n(repeat_n(' ', width as usize).chain(once('\n')).collect_vec(), height as usize).flatten().collect_vec();
    let mut ys = HashMap::new();
    for e in v {
        a[(e.yp * (width + 1) + e.xp) as usize] = '█';
        ys.entry(e.yp).and_modify(|v| { *v+= 1;}).or_insert(1);
    }
    (a.into_iter().collect(), ys.values().max().unwrap() >= &30)
}

fn main() {
    let mut input = String::new();
    
    let _ = io::stdin().read_to_string(&mut input);
    let parsed=input.lines().map(|a|
        { let v = a.split(|x| x == ',' || x == '=' || x == ' ').collect::<Vec<_>>();
           Entry {
                xp: v[1].parse().unwrap(),
                yp: v[2].parse().unwrap(),
                xv: v[4].parse().unwrap(),
                yv: v[5].parse().unwrap()
           }});
    
    use std::time::Instant;
    let now = Instant::now();
    //println!("{:?}", parsed.map(solve).collect_vec());
    let (r1, r2) = compute(parsed.clone());
    let elapsed = now.elapsed();
    println!("{} {}", r1, r2);

    println!("Elapsed: {:.2?}", elapsed);
}