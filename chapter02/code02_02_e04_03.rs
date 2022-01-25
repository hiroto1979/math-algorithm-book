use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    println!("{}", a.iter().fold(0, |x, y| (x + y) % 100))
}
