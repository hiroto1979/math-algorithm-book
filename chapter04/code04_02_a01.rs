use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        lr: [(usize, usize); q],
    }

    let mut summary = vec![0; n+1];
    for i in 1..n+1 {
        summary[i] = summary[i-1] + a[i-1];
    }

    for i in 0..q {
        let ans = summary[lr[i].1] - summary[lr[i].0-1];
        println!("{}", ans);
    }
}
