use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    }

    let mut count = 0;
    for i in 1..=n {
        if i % x == 0 || i % y == 0 {
            count += 1;
        }
    }
    println!("{}", count);
}
