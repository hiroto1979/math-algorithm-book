use proconio::input;
use std::cmp::max;

fn main(){
    input!{
        n : usize,
        w : usize,
        v : [[usize; 2]; n],
    }

    // 配列の初期化
    let mut dp = vec![vec![0; w + 1]; n + 1];
    dp[0][0] = 0;

    // 動的計画法
    for i in 0..n {
        for j in 0..w+1 {
            // j < w[i] のとき、方法 B をとる選び方ができない
            dp[i + 1][j] = max(dp[i + 1][j], dp[i][j]);
            // j> = w[i] のとき、方法 A・方法 B どちらも選べる
            if j + v[i][0] <= w {
                dp[i + 1][j + v[i][0]] = max(dp[i + 1][j + v[i][0]], dp[i][j] + v[i][1]);
            }
        }
    }

    let mut ans = 0;
    for i in 0..w + 1{
        ans = max(ans, dp[n][i]);
    }
    println!("{}", ans);
}
