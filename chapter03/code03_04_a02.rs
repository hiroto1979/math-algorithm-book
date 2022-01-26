use proconio::input;

fn main() {
    input! {
        n: usize,
        pq: [(usize, usize); n],
    }

    let mut expected_value : f64 = 0.0;
    for i in 0..n {
        expected_value += (pq[i].1 as f64) / (pq[i].0 as f64);
    }

    println!("{}", expected_value);
}
