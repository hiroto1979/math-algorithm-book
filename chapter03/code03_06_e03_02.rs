use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort_unstable();
    println!("{}", a.iter().join(" "));
}
