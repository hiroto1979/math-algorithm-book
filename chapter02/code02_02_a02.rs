use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    println!("{}", (a & b));  // a AND b の値を出力する
    println!("{}", (a | b));  // a OR b の値を出力する
    println!("{}", (a ^ b));  // a XOR b の値を出力する
}
