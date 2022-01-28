use proconio::input;

fn modpow(a: usize, b: usize, m: usize) -> usize {
    // 繰り返し二乗法（p は a^1, a^2, a^4, a^8, ... といった値をとる）
    let mut p = a;
    let mut ans = 1;
    for i in 0..30 {
        if (b & (1 << i)) != 0 {
            ans *= p;
            ans %= m;
        }
        p *= p;
        p %= m;
    }
    return ans;
}

// division(a, b, m) は a/b mod m を返す関数
fn division(a: usize, b: usize, m: usize) -> usize {
    return (a * modpow(b, m - 2, m)) % m;
}

fn main() {
    input! {
        x: usize,
        y: usize,
    }

    const MOD : usize = 1000000007;

    // 二項係数の分子と分母を求める（手順 1.／手順 2.）
    let mut bunshi = 1;
    let mut bunbo = 1;
    for i in 1..=x+y {
        bunshi *= i;
        bunshi %= MOD;
    }
    for i in 1..=x {
        bunbo *= i;
        bunbo %= MOD;
    }
    for i in 1..=y {
        bunbo *= i;
        bunbo %= MOD;
    }

    // 答えを求める（手順 3.）
    println!("{}", division(bunshi, bunbo, MOD));
}
