use proconio::input;

fn func(n: usize) -> usize {
    if n == 1 {
        return 1;
    }
    return func(n-1) * n;
}


fn main() {
    input! {
        n: usize,
    }
    
    // 出力
    println!("{}", func(n));
}
