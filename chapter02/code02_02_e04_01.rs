use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut total = 0;
    for i in 0..n {
        total += a[i];
    }
    println!("{}", total%100);
}
