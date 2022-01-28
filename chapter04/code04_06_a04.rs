use proconio::input;

fn modpow(a: usize, b: usize, m: usize) -> usize {
    // 繰り返し二乗法（p は a^1, a^2, a^4, a^8, ... といった値をとる）
    let mut p = a;
    let mut ans = 1;
    for i in 0..30 {  // b =< 10^9 のため、2^30 あたりまで計算しておきたい
        if (b & (1 << i)) != 0 {
            ans *= p;
            ans %= m;
        }
        p *= p;
        p %= m;
    }
    return ans;
}

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    const MOD : usize = 1000000007;
    println!("{}", modpow(a, b, MOD));
}
