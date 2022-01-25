use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
    }

    let mut count = 0;
    for i in 1..=n {
        for j in 1..=n {
            if i + j <= s {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
