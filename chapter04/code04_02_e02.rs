use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        a: [(usize, usize); n],
    }

    let mut workers = vec![0; t+1];

    // 階差を計算する
    for i in 0..n {
        workers[a[i].0] += 1;
        workers[a[i].1] -= 1;
    }

    // 階差から累積和を計算する
    for i in 0..t {
        workers[i+1] += workers[i];
        println!("{}", workers[i]);
    }
}
