use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut f = vec![0; n+1];
    // f[1], f[2], ..., f[n] を計算する
    for i in 1..n+1 {
        // f[i], f[2i], f[3i], ... に 1 を加算
        let mut j = i;
        while j <= n {
            f[j] += 1;
            j += i;
        }
    }
    
    let mut ans = 0;
    // 答えを求める
    for i in 1..n+1 {
        ans += i * f[i];
    }
    
    println!("{}", ans);
}
