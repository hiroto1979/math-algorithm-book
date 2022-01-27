use proconio::input;
use std::cmp::max;

fn main(){
    input!{
        n : usize,
        mut a : [usize; n],
    }

    // 配列の初期化
    let mut dp = vec![0; n+1];

    if 2 <= n {
        if a[1-1] <= a[2-1] {
            dp[1] = a[2-1];
        } else {
            dp[1] = a[1-1];
        }
    }
    if 3 <= n {
        if a[2-1] <= a[1-1] + a[3-1] {
            dp[2] = a[1-1] + a[3-1];
        } else {
            dp[2] = a[2-1];
        }
    }
    for i in 4..n+1 {
        dp[i] = max(max(dp[i-2] + a[i-1], dp[n-3] + a[i-1]), dp[i-1]);
    }

    println!("{}", dp[n]);
}
