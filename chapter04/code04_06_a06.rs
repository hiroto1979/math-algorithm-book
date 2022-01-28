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

fn ncr(n: usize, r: usize, m: usize, f: Vec<usize>) -> usize {
    // ncr は n! を r! × (n-r)! で割った値
    return division(f[n], f[r] * f[n - r] % m, m);
}

fn main() {
    input! {
        x: usize,
        y: usize,
    }

    const MOD : usize = 1000000007;
    let mut fact =  vec![0; 200009];

    // 配列の初期化（fact[i] は i の階乗を 1000000007 で割った余り）
    fact[0] = 1;
    for i in 1..=200000 {
        fact[i] = i * fact[i - 1] % MOD;
    }

    // 入力 → 答えの出力
    println!("{}", ncr(x+y, y, MOD, fact));
}
