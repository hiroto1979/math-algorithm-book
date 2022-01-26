use proconio::input;

pub fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;

    while 1 <= a  && 1 <= b {
        if a < b {
            b = b % a;
        } else {
            a = a % b;
        }
    }

    if 1 <= a {
        a
    } else {
        b
    }
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut result = a[0];
    for i in 1..n {
        result = gcd(result, a[i]);
    }
    println!("{}", result);
}
