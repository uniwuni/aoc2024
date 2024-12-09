use std::{io::{self, Read}, iter::Cycle};

fn compute(parsed: Vec<((usize,u32), &bool)>) -> (u32, u32) {
    let mut parsed = parsed;
    let mut i = 0;
    let mut flag = true;
    while flag {
        println!("{}: {:?}", i, parsed.iter().map(|((a,b), &c)| (a,b, if c {'*'} else {'_'})).collect::<Vec<_>>());
        
        if *parsed[i].1 {
            i += 1;
            if i >= parsed.len() {
                break; 
            }
            continue;
        }
        
        let mut j = 1;
        while parsed[i].0.1 > 0 {
            let mut last = *parsed.last().unwrap();
            if parsed[i].0.1 >= last.0.1 {
                parsed.insert(i + j, last);
                parsed[i].0.1 -= last.0.1;
                parsed.pop();
                if i + j != parsed.len() - 1 {
                    parsed.pop();
                } else {
                    flag = false;
                    break;
                }
                println!("if {:?}", parsed.iter().map(|((a,b), &c)| (a,b, if c {'*'} else {'_'})).collect::<Vec<_>>());
                j += 1;
        
            } else {
                parsed.insert(i + j, ((last.0.0, parsed[i].0.1), last.1));
                let _l = &parsed.len() - 1;
                parsed[_l].0.1 -= parsed[i].0.1;
                parsed[i].0.1 = 0;
                println!("else {:?}", parsed.iter().map(|((a,b), &c)| (a,b, if c {'*'} else {'_'})).collect::<Vec<_>>());
        
            }
        }
        
        parsed.remove(i);
        println!("iter over {:?}", parsed.iter().map(|((a,b), &c)| (a,b, if c {'*'} else {'_'})).collect::<Vec<_>>());
    }
    /*for i in 0..parsed.len() / 2 {
        let mut space_len = parsed[2 * i + 1].1;
        let mut j = 1;
        let mut last = *parsed.last().unwrap();
        println!("{:?}", parsed);
        while space_len != 0 {
            if last.1 <= space_len {
            parsed.insert(2 * i + j + 1, last);
            space_len -= last.1;
            parsed.pop();
            parsed.pop();
            println!("{:?}", parsed);
            j += 1;
            last = *parsed.last().unwrap();
        } else {
            parsed.insert(2 * i + 1 + j,(last.0, last.1 - space_len));
            let l = parsed.len();
            parsed[l - 1]= (last.0, space_len);
            println!("{:?}", parsed);
            space_len = 0;
        }
        }
        
            parsed.remove(2 * i + 1);
    
        
    }*/
    //let res1: u64 = parsed.iter().filter(|((a,b),c)| **c).enumerate().map(|(i,((a,b),c)|.sum()
}
fn main() {
    let mut input = String::new();
    
    let _ = io::stdin().read_to_string(&mut input);
    use std::time::Instant;
    let now = Instant::now();

    let (r1, r2) = compute(input.char_indices().filter_map(|(i, c)| c.to_digit(10).and_then(|c| Some((i,c)))).zip([true,false].iter().cycle()).collect());
    let elapsed = now.elapsed();
    println!("{} {}", r1, r2);

    println!("Elapsed: {:.2?}", elapsed);
}