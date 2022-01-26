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

pub fn lcm(a: usize, b: usize) -> usize {
    return a / gcd(a, b) * b 
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
 
    let mut result = a[0];
    for i in 1..n {
        result = lcm(result, a[i]);
    }
    println!("{}", result);
}

