use proconio::input;

// for文での実装
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut total = 0;
    for i in 0..n {
        total += a[i]
    }

    println!("{}", total);
}
