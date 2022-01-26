use proconio::input;

fn main() {
    input! {
        n: usize,
        r: usize,
    }

    let mut s = n - r;
    if r <= n - r {
        s = r;
    }

    let mut comb = 1;
    for i in 1..s+1 {
        comb = comb * (n-i+1) / i;
    }

    println!("{}", comb);
}
