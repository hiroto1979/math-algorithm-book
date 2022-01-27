use proconio::input;

fn swap(n: usize, m: usize, mut a: Vec<usize>) -> Vec<usize> {
    let tmp = a[n];
    a[n] = a[m];
    a[m] = tmp;
    return a;
}

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    
    let mut min_index;
    let mut min_value;
    // 選択ソート
    for i in 0..n-1 {
        min_index = i;
        min_value = a[i];
        for j in i+1..n {
            if a[j] < min_value {
                min_index = j;  // min_index は最小値のインデックス（1～n）
                min_value = a[j];  // min_value は現時点での最小値
            }
        }
        a = swap(i, min_index, a);
    }

    // 出力
    println!("{:?}", a);
}
