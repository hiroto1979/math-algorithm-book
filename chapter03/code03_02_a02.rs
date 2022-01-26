use proconio::input;

// ユークリッドの互除法による実装

// 正の整数 A と B の最大公約数を返す関数
// GCD は Greatest Common Divisor（最大公約数）の略
fn gcd(mut a: usize, mut b: usize) -> usize {
    while 1 <= a && 1 <= b {
        if a < b {
            b = b % a;
        } else {
            a = a % b;
        }
    }
    if 1 <= a {
        return a;
    } else {
        return b;
    }
}

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    println!("{}", gcd(a, b));
}
