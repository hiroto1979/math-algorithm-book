use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let expected_value = (2.0 * (a.iter().sum::<usize>() as f64) + 4.0 * (b.iter().sum::<usize>() as f64)) / 6.0 ;
    println!("{}", expected_value);
}