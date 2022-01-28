use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        m: usize,
        b: [usize; m],
    }

    let mut dist = vec![0; n+1];
    for i in 1..n {
        dist[i+1] = dist[i] + a[i-1];
    }

    let mut ans = 0;
    for i in 0..m-1 {
        if b[i] < b[i+1] {
            ans += dist[b[i+1]] - dist[b[i]];
        } else {
            ans += dist[b[i]] - dist[b[i+1]];
        }
    }

    println!("{}", ans);
}
