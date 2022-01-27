use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [isize; n],
    }

    let mut dp: Vec<isize> = vec![0; n];

    // dp[1]の決定
    dp[1] = (h[1] - h[0]).abs();

    // dp[2] から dp[n-1] までの決定
    let mut d1;
    let mut d2;
    for i in 2..n {
        d1 = (h[i] - h[i-1]).abs();
        d2 = (h[i] - h[i-2]).abs();

        if dp[i-1] + d1 <= dp[i-2] + d2 {
            dp[i] = dp[i-1] + d1;
        } else {
            dp[i] = dp[i-2] + d2;
        }
    }

    println!("{}", dp[n-1])
}
