use std::{collections::HashMap, io::{self, Read}};

use itertools::Itertools;

fn renumber(parsed: Vec<Vec<char>>) -> Vec<Vec<u32>> {
    let mut res = vec![];
    let mut maxnum = 0;
    for y in 0..parsed.len() {
        res.push(vec![]);
        for x in 0..parsed[0].len() {
            res[y].push(maxnum);
            maxnum += 1;
            
        }
    }
    //println!("{:?}", res);
    let mut res_old = vec![];
    while res != res_old {
        res_old = res.clone().into_iter().collect();
        for y in 0..parsed.len() {
        for x in 0..parsed[0].len() { 
            let n = res[y][x];
            for (dx,dy) in [(x,y.wrapping_sub(1)),(x.wrapping_sub(1),y), (x+1, y),  (x, y+1), ] {
                if let Some(a) = res.get_mut(dy).and_then(|a| a.get_mut(dx)) {
                    if parsed[dy][dx] == parsed[y][x] {
                        //println!("copy {} {} to {} {}",x, y,dx, dy);
                        res[dy][dx] = n.min(res[dy][dx]);
                        res[y][x] = n.min(res[dy][dx]);
                        
                    }
                }
            }
        }
    }
}
    //println!("{:?}", res);
    res
}

fn get_area_perimeter(res : Vec<Vec<u32>>) -> HashMap<u32, (usize, usize)> {
    let mut map = HashMap::new();
    for y in 0..res.len() {
        for x in 0..res[0].len() { 
            let n = res[y][x];
            map.entry(n).and_modify(|a: &mut (usize, usize)| (*a).0 += 1).or_insert((1,0));
            for (dx,dy) in [(x,y.wrapping_sub(1)),(x.wrapping_sub(1),y), (x+1, y),  (x, y+1), ] {
                if let Some(a) = res.get(dy).and_then(|a| a.get(dx)) {
                    if *a != n {
                        map.get_mut(&n).unwrap().1 += 1;
                    }
                } else {
                    map.get_mut(&n).unwrap().1 += 1;
                }
            }
        }
    }
    map
}

fn corners(res : Vec<Vec<u32>>) -> HashMap<u32, Vec<(usize,usize)>> {
    let mut map = HashMap::new();
    for y in 0..res.len() {
        for x in 0..res[0].len() { 
            let n = res[y][x];
            let mut neighbors = vec![];
            for (dx,dy) in [(x,y.wrapping_sub(1)),(x.wrapping_sub(1),y), (x, y+1), (x+1, y) ] {
                if let Some(a) = res.get(dy).and_then(|a| a.get(dx)) {
                    if *a != n {
                        neighbors.push(true);
                    } else {
                        neighbors.push(false);
                    }
                } else {
                    neighbors.push(true);
                }
            }
            if (neighbors[0] && neighbors[1]) || (neighbors[1] && neighbors[2]) || (neighbors[2] && neighbors[3]) || (neighbors[3] && neighbors[0]) {
                map.entry(n).and_modify(|a: &mut Vec<(usize, usize)>| { (*a).push((y,x)); }).or_insert(vec![(y,x)]);
            }
        }
    }
    map
}


fn compute(parsed: Vec<Vec<char>>) -> (usize, usize) {
    let a = renumber(parsed);
    let v = get_area_perimeter(a.clone());
    println!("{:?}", v.len());
    let w: usize = v.iter().map(|(_,(v,w))| v * w).sum();
    (w,0)
}
fn main() {
    let mut input = String::new();

    let _ = io::stdin().read_to_string(&mut input);
    use std::time::Instant;
    let parsed: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let now = Instant::now();
    let (r1, r2) = compute(parsed);
    let elapsed = now.elapsed();
    println!("{} {}", r1, r2);

    println!("Elapsed: {:.2?}", elapsed);
}