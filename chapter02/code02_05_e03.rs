use proconio::input;
 
fn main() {
    input! {
        n: usize,
    }
 
    let mut prod = 1;
    for i in 2..=n {
        prod *= i;
    }
 
    println!("{}", prod);
}
