use proconio::input;

// イテレータ(fold)での実装
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    println!("{}", a.iter().fold(0, |x, y| (x + y)))
}
