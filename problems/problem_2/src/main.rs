use std::mem::replace;

/// Solves the second Project Euler problem
fn main() {
    println!("Solution 2: {}", solution(4_000_000));
}

fn solution(max: i32) -> i32 {
    let mut f0 = 0;
    let mut f1 = 1;
    let mut sum = 0;

    loop {
        let f2 = f0 + &f1;
        f0 = replace(&mut f1, f2);

        if f0 > max {
            break;
        } else {
            if f0 % 2 == 0 {
                sum += f0
            }
        }
    }

    sum
}
