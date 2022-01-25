use proconio::input;

// イテレータ(sum)での実装
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let total : usize = a.iter().sum();
    println!("{}", total);
}
