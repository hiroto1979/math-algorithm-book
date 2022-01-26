use proconio::input;
 
fn main() {
    input! {
        n: usize,
    }
 
    let mut expected_value : f64 = 0.0;
    for i in 1..=n {
        expected_value += (n as f64) / (i as f64);
    }
 
    println!("{}", expected_value);
}