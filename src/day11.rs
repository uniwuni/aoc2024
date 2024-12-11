use std::io::{self, BufRead, Read};
use std::str::FromStr;
use std::collections::HashMap;
use itertools::Itertools;

fn step(n : u64) -> (u64, Option<u64>) {
    if n == 0 {
        (1, None)
    }
    else if n < 10 {
       (n * 2024, None)
   } else if n < 100 {
       (n/10, Some(n%10))
   } else if n < 1000 {
    (n * 2024, None)
   } else if n < 10000 {
    (n/100, Some(n%100))
   } else if n < 100000 {
    (n * 2024, None)
   } else if n < 1000000 {
    (n/1000, Some(n%1000))
   } else if n < 10000000 {
    (n * 2024, None)
   } else if n < 100000000 {
    (n/10000, Some(n%10000))
   } else if n < 1000000000 {
    (n * 2024, None)
   } else if n < 10000000000 {
    (n/100000, Some(n%100000))
   }  else  if n<100000000000 {
    (n * 2024, None)
} else if n < 1000000000000 {
    (n/1000000, Some(n%1000000))
} else if n < 10000000000000 {
 (n * 2024, None)
} else if n < 100000000000000 {
    (n/10000000, Some(n%10000000))
} else if n < 1000000000000000 {
 (n * 2024, None)
} else if n < 10000000000000000 {
    (n/100000000, Some(n%100000000))
} else if n < 100000000000000000 {
 (n * 2024, None)
} else if n < 1000000000000000000 {
    (n/1000000000, Some(n%1000000000))
} else if n < 10000000000000000000 {
 (n * 2024, None)
} else {
    (n/10000000000, Some(n%10000000000))
}
}

fn compute(parsed: Vec<u64>, max: usize) -> usize {
    let mut parsed: HashMap<u64, usize> = parsed.into_iter().counts();
    for i in 0..max {
        let mut newmap = HashMap::new();
        for (p,c) in parsed {
            let (a,b) = step(p);
            newmap.entry(a).and_modify(|e| { *e += c; }).or_insert(c);
            if let Some(b) = b {
                newmap.entry(b).and_modify(|e| { *e += c; }).or_insert(c);
            }
        }
        parsed = newmap;
        //println!("{} {} {:?}", i, parsed.iter().map(|(a,b)| (*b as usize)).sum::<usize>(), parsed);
    }
    parsed.into_iter().map(|(a,b)| b as usize).sum()
}
fn main() {
    let mut input = String::new();

    let _ = io::stdin().read_to_string(&mut input);
    let parsed: Vec<u64> = input.split_whitespace().map(|a| a.parse().unwrap()).collect();
    use std::time::Instant;
    let now = Instant::now();                                            

    let r1 = compute(parsed.clone(), 25);
    let r2 = compute(parsed, 75);
    let elapsed = now.elapsed();
    println!("{} {}", r1, r2);

    println!("Elapsed: {:.2?}", elapsed);
}
