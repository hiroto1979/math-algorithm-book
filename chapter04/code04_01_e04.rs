use proconio::input;
use std::cmp::{min, max};

fn cross_product(ax: isize, ay: isize, bx: isize, by: isize) -> isize {
    // ベクトル (ax, ay) と (bx, by) の外積の大きさ
    return ax * by - ay * bx;
}

fn main() {
    input! {
        x1: isize,
        y1: isize,
        x2: isize,
        y2: isize,
        x3: isize,
        y3: isize,
        x4: isize,
        y4: isize,
    }

    // cross_product(AB, AC) を計算
    let ans1 = cross_product(x2-x1, y2-y1, x3-x1, y3-y1);
    let ans2 = cross_product(x2-x1, y2-y1, x4-x1, y4-y1);
    let ans3 = cross_product(x4-x3, y4-y3, x1-x3, y1-y3);
    let ans4 = cross_product(x4-x3, y4-y3, x2-x3, y2-y3);

    let a;
    let b;
    let c;
    let d;
    // すべて一直線上に並んでいる場合（コーナーケース）
    if ans1 == 0 && ans2 == 0 && ans3 == 0 && ans4 == 0 {
        // A, B, C, D を数値（正確には pair 型）とみなす
        // 適切に swap することで A<B, C<D を仮定できる
        // そうすると、区間が重なるかの判定（節末問題 2.5.6）に帰着できる
        if (x1 == x2 && x2 == x3 && x3 == x4) {
            if y1 < y2 {
                a = y1;
                b = y2;
            } else {
                a = y2;
                b = y1;
            }
            if y3 < y4 {
                c = y3;
                d = y4;
            } else {
                c = y4;
                d = y3;
            }
        } else {
            if x1 < x2 {
                a = x1;
                b = x2;
            } else {
                a = x2;
                b = x1;
            }
            if x3 < x4 {
                c = x3;
                d = x4;
            } else {
                c = x4;
                d = x3;
            }
        }

        if max(a, c) <= min(b, d) {
            println!("{}", "Yes");
        } else {
            println!("{}", "No");
        }
    } else {
        // すべて一直線上に並んでいない場合
        let mut is_ab = false;
        let mut is_cd = false;
        // is_ab: 線分 AB が点 C, D を分けるか？
        if (ans1 >= 0 && ans2 <= 0) ||  (ans1 <= 0 && ans2 >= 0) {
            is_ab = true;
        }
        // is_cd: 線分 CD が点 A, B を分けるか？
        if (ans3 >= 0 && ans4 <= 0) || (ans3 <= 0 && ans4 >= 0) {
            is_cd = true;
        }
        // 答えの出力
        if is_ab == true && is_cd == true {
            println!("{}", "Yes");
        } else {
            println!("{}", "No");
        }
    }
}
