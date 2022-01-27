use proconio::input;
use rand::Rng;

fn main() {
    input! {
        n: usize,
    }

    let mut m = 0;
    let mut rng = rand::thread_rng(); // デフォルトの乱数生成器を初期化する
    let mut px: f64;
    let mut py: f64;

    for _ in 0..n {
        px = rng.gen();  // 0 以上 1 以下の乱数（ランダムな数）を生成
        py = rng.gen();  // 0 以上 1 以下の乱数（ランダムな数）を生成

        // 原点からの距離は sqrt(px * px + py * py)
        // これが 1 以下であれば良いので、条件は「px * px + py * py <= 1」
        if px * px + py * py <= 1.0 {
            m += 1;
        }
    }

    println!("{}", 4.0*(m as f64)/(n as f64));
}
