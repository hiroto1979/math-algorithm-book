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
        x: usize,
        y: usize,
    }

    const MOD : usize = 1000000007;

    // 場合分け（a, b が負になってしまう場合）
    if 2 * y - x < 0 || 2 * x - y < 0 {
        println!("{}", 0);
    } else if (2 * y - x) % 3 != 0 || (2 * x - y) % 3 != 0 {     // 場合分け（a, b が整数にならない場合）
        println!("{}", 0);
    } else {
        // 二項係数の分子と分母を求める（手順 1.／手順 2.）
        let mut bunshi = 1;
        let mut bunbo = 1;
        let a = (2 * y - x) / 3;
        let b = (2 * x - y) / 3;
        for i in 1..=a+b {
            bunshi *= i;
            bunshi %= MOD;
        }
        for i in 1..=a {
            bunbo *= i;
            bunbo %= MOD;
        }
        for i in 1..=b {
            bunbo *= i;
            bunbo %= MOD;
        }
    
        // 答えを求める（手順 3.）
        println!("{}", division(bunshi, bunbo, MOD, 30));
    }
}
