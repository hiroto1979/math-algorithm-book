use proconio::input;

fn solve(l: usize, r: usize, a: Vec<usize>) -> usize {
    if r - l == 1 {
        return a[l];
    }
    let m = (l + r) / 2; // 区間 [l, r) の中央で分割する
    let s1 = solve(l, m, a.clone()); // s1 は A[l]+...+A[m-1] の合計値となる
    let s2 = solve(m, r, a); // s2 は A[m]+...+A[r-1] の合計値となる
    return s1 + s2;
}


fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    
    // 再帰呼び出し → 答えの出力
    println!("{}", solve(0, n, a));
}
