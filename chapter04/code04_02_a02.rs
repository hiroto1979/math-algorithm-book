use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [(usize, usize, usize); q],
    }

    let mut result: Vec<isize> = vec![0; n+1];
    for i in 0..q {
        let l = a[i].0;
        let r = a[i].1;
        let x = a[i].2;
        result[l-1] += x  as isize;
        result[r] -= x  as isize;
        result[0] = 0;
        result[n] = 0;
    }

    for i in 1..n {
        if result[i] == 0 {
            print!("{}", "=");
        } else if result[i] > 0 {
            print!("{}", "<");
        } else {
            print!("{}", ">");
        }
    }
    println!();
}