use proconio::input;

fn main() {
    input! {
        n
        n: [usize; 3],
    }

    let sum = n[0] + n[1] + n[2];
    println!("{}", sum);
}
