use proconio::input;

fn modpow(a: usize, b: usize, m: usize, order: usize) -> usize {
    // 繰り返し二乗法（p は a^1, a^2, a^4, a^8, ... といった値をとる）
    let mut p = a;
    let mut ans = 1;
    for i in 0..order {
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
fn division(a: usize, b: usize, m: usize, order: usize) -> usize {
    return (a * modpow(b, m - 2, m, order)) % m;
}

fn main() {
    input! {
        n: usize,
    }

    const MOD : usize = 1000000007;
    const ORDER : usize = 60;

    // 答えの計算
    let v = modpow(4, n + 1, MOD, ORDER) - 1;
    let ans = division(v, 3, MOD, ORDER);

    println!("{}", ans);
}
