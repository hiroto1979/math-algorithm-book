use proconio::input;
use std::cmp::min;

// 正の整数 A と B の最大公約数を返す関数
// GCD は Greatest Common Divisor（最大公約数）の略
fn gcd(a: usize, b: usize) -> usize {
    let mut ans = 0;
    let n = min(a, b);
    for i in 1..n {
        if a % i == 0 && b % i == 0 {
            ans = i;
        }
    }
    return ans;
}

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    println!("{}", gcd(a, b));
}
