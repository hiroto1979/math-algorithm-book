use proconio::input;

// A[l], A[l+1], ..., A[r-1] を小さい順に整列する関数
fn merge_sort(l: usize, r: usize, mut a: Vec<usize>) -> Vec<usize> {
    // r-l=1 の場合、すでにソートされているので何もしない
    if r - l == 1 {
        return a;
    }

    // 2 つに分割した後、小さい配列をソート
    let m = (l + r) / 2;
    merge_sort(l, m, a.clone());
    merge_sort(m, r, a.clone());
    
    // この時点で以下の 2 つの配列がソートされている：
    // 列 A' に相当するもの [A[l], A[l+1], ..., A[m-1]]
    // 列 B' に相当するもの [A[m], A[m+1], ..., A[r-1]]
    // 以下が Merge 操作となります。
    let mut c1 = l;
    let mut c2 = m;
    let mut cnt = 0;
    let mut c = vec![0; a.len()];
    while c1 != m || c2 != r {
        if c1 == m {
            // 列 A' が空の場合
            c[cnt] = a[c2];
            c2 += 1;
        }
        else if c2 == r {
            // 列 B' が空の場合（抜けている部分）
            c[cnt] = a[c1];
            c1 += 1;
        }
        else {
            // そのいずれでもない場合（抜けている部分）
            if a[c1] <= a[c2] {
                c[cnt] = a[c1];
                c1 += 1;
            }
            else {
                c[cnt] = a[c2];
                c2 += 1;
            }
        }
        cnt += 1;
    }
    
    // 列 A', 列 B' を合併した配列 C を A に移す
    // [C[0], ..., C[cnt−1]] −> [A[l], ..., A[r−1]]
    for i in 0..cnt {
        a[l + i] = c[i];
    }
    return a;
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    
    // マージソート → 答えの出力
    println!("{:?}", merge_sort(0, n, a));
}
