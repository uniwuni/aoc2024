use std::io::{self, BufRead};
use itertools::Itertools;

fn next_power_of_10(n : u64) -> u64 {
     if n < 10 {
        10
    } else if n < 100 {
        100
    } else if n < 1000 {
        1000
    } else if n < 10000 {
        10000
    } else if n < 100000 {
        100000
    } else if n < 1000000 {
        1000000
    } else if n < 10000000 {
        10000000
    } else if n < 100000000 {
        100000000
    } else if n < 1000000000 {
        1000000000
    } else if n < 10000000000 {
        10000000000
    } else if n < 100000000000 {
        100000000000
    } else if n < 1000000000000 {
        1000000000000
    } else if n < 10000000000000 {
        10000000000000
    } else if n < 100000000000000 {
        100000000000000
    } else if n < 1000000000000000 {
        1000000000000000
    } else if n < 10000000000000000 {
        10000000000000000
    } else if n < 100000000000000000 {
        100000000000000000
    } else if n < 1000000000000000000 {
        1000000000000000000
    } else if n < 10000000000000000000 {
        10000000000000000000
    } else {
        std::u64::MAX
    }
}

fn can_be_made(target: u64, vals: &[u64]) -> bool {
    let op_count = vals.len() - 1;
    let arrangements = vec![[false,true].into_iter(); op_count].into_iter().multi_cartesian_product();
    for arrangement in arrangements {
        let mut res = vals[0];
        for n in 0..op_count {
            if res > target {
                break;
            }
            if arrangement[n] {
                res *= vals[n + 1];
            } else {
                res += vals[n + 1];
            }
        }
        if res == target {
            return true
        }
    }
    false
}

fn can_be_made2(target: u64, vals: &[u64]) -> bool {
    let op_count = vals.len() - 1;
    let arrangements = vec![0..3; op_count].into_iter().multi_cartesian_product();
    for arrangement in arrangements {
        let mut res = vals[0];
        for n in 0..op_count {
            if res > target {
                break;
            }
            if arrangement[n] == 0 {
                res *= vals[n + 1];
            } else if arrangement[n] == 1 {
                res += vals[n + 1];
            } else {
                res *= next_power_of_10(vals[n+1]);
                res += vals[n+1];
            }
        }
        if res == target {
            return true
        }
    }
    false
}

fn compute(data: &[(u64, Vec<u64>)]) -> (u64, u64) {
    let (possible, impossible) : (Vec<_>,_) = data.iter().partition(|(a,b)| can_be_made(*a,b));
    let res1 = possible.iter().map(|a| a.0).sum();
    let res2: u64 = impossible.iter().filter(|(a,b)| can_be_made2(*a,b)).map(|a| a.0).sum();
    (res1, res2 + res1)
}
fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let data: Vec<(u64, Vec<u64>)> =
        lines.map(|a| {
            let a = a.unwrap();
            let (i, e) = a.split_once(':').unwrap();
            (i.parse::<u64>().unwrap(), e.split_whitespace().map(|a| a.parse::<u64>().unwrap()).collect())
        }).collect();

    use std::time::Instant;
    let now = Instant::now();

    let (r1, r2) = compute(&data);
    let elapsed = now.elapsed();
    println!("{} {}", r1, r2);

    println!("Elapsed: {:.2?}", elapsed);
}
