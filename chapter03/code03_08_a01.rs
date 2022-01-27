use proconio::input;

fn binary_search(v: Vec<usize>, n: usize) -> bool {
    let mut left = 0;
    let mut right = v.len();
    let mut mid;

    while left != right {
        mid = (left + right) / 2;
        if v[mid] == n {
            return true;
        } else if n < v[mid] {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    return false;
}

fn main() {
    input! {
        n: usize,
        x: usize,
        mut a: [usize; n],
    }

    // ベクタをソート
    a.sort_unstable();
    
    if binary_search(a, x) {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}
