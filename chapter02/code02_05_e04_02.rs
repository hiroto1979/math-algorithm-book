use proconio::input;

// エラトステネスのふるいを用いた実装
pub fn sieve_of_Eratosthenes(n: usize) -> Vec<bool> {
    let mut primes = vec![true; n+1];
 
    let mut i = 2;
    while i * i <= n {
        if primes[i] {
            let mut j = 2;
            while i * j <= n {
                primes[i*j] = false;
                j += 1;
            }
        }
        i += 1;
    }
 
    return primes;
}
 
fn main() {
    input! {
        n: usize,
    }
 
    // n 以下の素数を小さい順に出力
    let mut answers = Vec::new();
    let primes = sieve_of_Eratosthenes(n);
    for i in 2..=n {
        if primes[i] {
            answers.push(i.to_string());
        }
    }
    println!("{}", answers.join(" "));
}