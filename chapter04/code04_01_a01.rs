use proconio::input;

fn main(){
    input!{
        a : [isize; 2],
        b : [isize; 2],
        c : [isize; 2],
    }

    // 各ベクトルの成分表示
    let ba_x = a[0] - b[0];
    let ba_y = a[1] - b[1];
    let bc_x = c[0] - b[0];
    let bc_y = c[1] - b[1];
    let ca_x = a[0] - c[0];
    let ca_y = a[1] - c[1];
    let cb_x = b[0] - c[0];
    let cb_y = b[1] - c[1];

    // どのパターンに当てはまるかを判定する
    let mut pattern = 2;
    if ba_x * bc_x + ba_y * bc_y < 0 {
        pattern = 1;
    } else if ca_x * cb_x + ca_y * cb_y < 0 {
        pattern = 3;
    }

    // 点と直線の距離を求める
    let dist;
    if pattern == 1 {
        dist = ((ba_x * ba_x + ba_y * ba_y) as f64).sqrt();
    } else if pattern == 2 {
        dist = ((ba_x * ca_y - ba_y * ca_x) as f64).abs() / ((bc_x * bc_x + bc_y * bc_y) as f64).sqrt();
    } else {
        dist = ((ca_x * ca_x + ca_y * ca_y) as f64).sqrt();
    }

    println!("{}", dist);
}
