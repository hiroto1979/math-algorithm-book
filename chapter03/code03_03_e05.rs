use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize; n],
    }

    let mut count_v = vec![0_i64; 3+1];
    for i in a.iter() {
        match i {
            1 => count_v[1] += 1,
            2 => count_v[2] += 1,
            3 => count_v[3] += 1,
            _ => ()
        }
    }

    println!("{}", count_v[1] * (count_v[1] - 1) / 2 + count_v[2] * (count_v[2] - 1) / 2 + count_v[3] * (count_v[3] - 1) / 2);
}
