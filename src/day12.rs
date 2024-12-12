use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

fn renumber(parsed: Vec<Vec<char>>) -> Vec<Vec<u32>> {
    let mut res = vec![];
    let mut maxnum = 0;
    for y in 0..parsed.len() {
        res.push(vec![]);
        for _x in 0..parsed[0].len() {
            res[y].push(maxnum);
            maxnum += 1;
        }
    }
    let mut res_old = vec![];
    while res != res_old {
        res_old = res.clone().into_iter().collect();
        for y in 0..parsed.len() {
            for x in 0..parsed[0].len() {
                let n = res[y][x];
                for (dx, dy) in [
                    (x, y.wrapping_sub(1)),
                    (x.wrapping_sub(1), y),
                    (x + 1, y),
                    (x, y + 1),
                ] {
                    if res.get(dy).and_then(|a| a.get(dx)).is_some() {
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

fn get_area_perimeter(res: Vec<Vec<u32>>) -> HashMap<u32, (usize, usize)> {
    let mut map = HashMap::new();
    for y in 0..res.len() {
        for x in 0..res[0].len() {
            let n = res[y][x];
            map.entry(n)
                .and_modify(|a: &mut (usize, usize)| (*a).0 += 1)
                .or_insert((1, 0));
            for (dx, dy) in [
                (x, y.wrapping_sub(1)),
                (x.wrapping_sub(1), y),
                (x + 1, y),
                (x, y + 1),
            ] {
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

fn scan(res: Vec<Vec<u32>>, entries: Vec<u32>) -> HashMap<u32, usize> {
    let mut h = HashMap::new();
    for e in entries {
        let mut ysides = 0;
        let mut oldxs: HashSet<isize> = HashSet::new();
        for y in 0..res.len() {
            let xs: HashSet<isize> = (0..=res[0].len())
                .filter_map(|x| {
                    if x == 0 && res[y][x] == e {
                        Some(x as isize)
                    } else if x == res[0].len() && res[y][x - 1] == e {
                        Some(-(x as isize))
                    } else if x > 0 && x < res[0].len() && res[y][x] == e && res[y][x - 1] != e {
                        Some(x as isize)
                    } else if x > 0 && x < res[0].len() && res[y][x - 1] == e && res[y][x] != e {
                        Some(-(x as isize))
                    } else {
                        None
                    }
                })
                .collect();
            ysides += xs.difference(&oldxs).count();
            oldxs = xs;
        }
        let mut xsides = 0;
        let mut oldys: HashSet<isize> = HashSet::new();
        for x in 0..res.len() {
            let ys: HashSet<isize> = (0..=res[0].len())
                .filter_map(|y| {
                    if y == 0 && res[y][x] == e {
                        Some(y as isize)
                    } else if y == res.len() && res[y - 1][x] == e {
                        Some(-(y as isize))
                    } else if y > 0 && y < res.len() && res[y][x] == e && res[y - 1][x] != e {
                        Some(y as isize)
                    } else if y > 0 && y < res.len() && res[y - 1][x] == e && res[y][x] != e {
                        Some(-(y as isize))
                    } else {
                        None
                    }
                })
                .collect();
            xsides += ys.difference(&oldys).count();
            //println!("{} {} {:?}", e, x, ysides);
            oldys = ys;
        }
        h.insert(e, ysides + xsides);
    }
    h
}

fn compute(parsed: Vec<Vec<char>>) -> (usize, usize) {
    let a = renumber(parsed);
    let v = get_area_perimeter(a.clone());
    let sides = scan(a, v.keys().copied().collect());

    let w: usize = v.iter().map(|(_, (v, w))| v * w).sum();
    let w2: usize = v.iter().map(|(k, (v, w))| v * sides[k]).sum();
    (w, w2)
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
