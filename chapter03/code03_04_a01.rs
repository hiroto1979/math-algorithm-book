use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let expected_value = ((a.iter().sum::<usize>() as f64) + (b.iter().sum::<usize>() as f64)) / (n as f64);
    println!("{}", expected_value);
}
