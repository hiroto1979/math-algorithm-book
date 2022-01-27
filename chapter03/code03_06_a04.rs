use proconio::input;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    return gcd(b, a%b);
}


fn main() {
    input! {
        a: usize,
        b: usize,
    }
    
    // 出力
    println!("{}", gcd(a, b));
}
