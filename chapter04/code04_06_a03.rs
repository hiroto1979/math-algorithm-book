use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    const MOD : usize = 1000000007;
    let mut ans = 1;  // a の 0 乗は 1 なので、ans=1 に初期化しておく

    for _ in 1..=b {
        ans = (ans * a) % MOD;
    }

    println!("{}", ans);
}
