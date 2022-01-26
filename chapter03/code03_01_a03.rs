use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }

    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            if i * i == n {
                println!("{}", i);
            } else {
                println!("{}", i);
                println!("{}", n/i);
            }
        }
        i += 1;
    }
}
