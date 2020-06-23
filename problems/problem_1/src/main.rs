/// Solves the first Project Euler problem
fn main() {
    println!("Solution 1: {}", solution(1000));
}

fn solution(max: u64) -> u64 {
    (0..max).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}
