use proconio::input;

fn main(){
    input!{
        n : usize,
        a : [isize; 2*n],
    }

    let x : Vec<isize> = a.iter().step_by(2).cloned().collect();

    let w = &a[1..];
    let y : Vec<isize> = w.iter().step_by(2).cloned().collect();

    let mut ans = 10000000.0;  // 上限を考慮した大きい値を取る
    let mut dist;

    // 全探索
    for i in 0..n {
        for j in i+1..n {
            // dist は i 番目の点と j 番目の点の距離
            dist = (((x[i] - x[j]) * (x[i] - x[j]) + (y[i] - y[j]) * (y[i] - y[j])) as f64).sqrt();
            if dist < ans {
                ans = dist;
            }
        }
    }

    println!("{}", ans);
}
