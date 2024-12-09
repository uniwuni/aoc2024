use std::io::{self, Read};

fn defragment1(parsed: Vec<((usize, u32), &bool)>) -> Vec<((usize, u32), &bool)> {
    let mut parsed = parsed;
    let mut i = 0;
    let mut flag = true;
    while flag {
        if *parsed[i].1 {
            i += 1;
            if i >= parsed.len() {
                break;
            }
            continue;
        }
        let mut j = 1;
        while parsed[i].0 .1 > 0 {
            let last = *parsed.last().unwrap();
            if parsed[i].0 .1 >= last.0 .1 {
                parsed.insert(i + j, last);
                parsed[i].0 .1 -= last.0 .1;
                parsed.pop();
                if i + j != parsed.len() - 1 {
                    parsed.pop();
                } else {
                    flag = false;
                    break;
                }
                
                j += 1;
            } else {
                parsed.insert(i + j, ((last.0 .0, parsed[i].0 .1), last.1));
                let _l = &parsed.len() - 1;
                parsed[_l].0 .1 -= parsed[i].0 .1;
                parsed[i].0 .1 = 0;
            }
        }

        parsed.remove(i);
    }
    parsed
}

fn pprint(p: &[((usize, u32), &bool)]) {
    for i in p {
        if(! i.1) {
            print!(".");
        } else {
            print!("{}", i.0.0);
        }
    }
    println!();
    for i in p {
        print!("{}", i.0.1);
    }
    println!();
}

fn defragment2(parsed: Vec<((usize, u32), &bool)>) -> Vec<((usize, u32), &bool)> {
    let mut parsed = parsed;
    let mut i = parsed.len() - 1;
    let mut flag = true;
    while flag {
        if i == 0 || i == ((0-1) as usize) {
            break;
        }
        if !*parsed[i].1 {
            i -= 1;
            continue;
        }
        let ((id, size), _) = parsed[i];
        match parsed.iter().position(|((id2,size2), full)| !**full && *size2 >= size) {
            None => {i -= 1}
            Some(p) => {
                if p >= i {
                    i -= 1;
                    continue;
                }
                let ((_,size2), _) = parsed[p];
                if size2 == size {
                    (parsed[p], parsed[i]) = (parsed[i], parsed[p]);
                    i -= 1;
                } else {
                    parsed[p].0.1 -= size;
                    parsed[i] = ((id, size), &  false);
                    parsed.insert(p, ((id, size), &true));
                }
            }
        }
        
    }
    parsed
}

fn checksum(parsed: &[((usize, u32), &bool)]) -> usize {
    let mut j = 0;
    let mut res1 = 0;
    for ((a, b), c) in parsed {
        if **c {
        res1 += j * a * *b as usize; 
        res1 += (b * (b - 1)) as usize * a / 2;
        }
        j += *b as usize;
    }
    
    res1
}

fn compute(parsed: Vec<((usize, u32), &bool)>) -> (usize, usize) {
    (checksum(&defragment1(parsed.clone())), checksum(&defragment2(parsed)))
}
fn main() {
    let mut input = String::new();

    let _ = io::stdin().read_to_string(&mut input);
    use std::time::Instant;
    let now = Instant::now();

    let (r1, r2) = compute(
        input
            .char_indices()
            .filter_map(|(i, c)| c.to_digit(10).and_then(|c| Some((i/2, c))))
            .zip([true, false].iter().cycle())
            .collect(),
    );
    let elapsed = now.elapsed();
    println!("{} {}", r1, r2);

    println!("Elapsed: {:.2?}", elapsed);
}
