use proconio::input;
 
fn main() {
    input! {
        n: usize,
    }

    let mut dp: Vec<usize> = vec![0; n];

    if n == 1 {
        println!("{}", 1)
    } else {
        // dp[0], dp[1]の決定
        dp[0] = 1;
        dp[1] = 2;

        // dp[2] から dp[n-1] までの決定
        for i in 2..n {
            dp[i] = dp[i-1] + dp[i-2];
        }
        println!("{}", dp[n-1])
    }
}
