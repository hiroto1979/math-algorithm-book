use proconio::input;
 
fn main() {
    input! {
        n:usize,
        a:[usize; n],
    }

    let mut count_v = vec![0_usize; 5];
    for i in a.iter() {
        let j = i / 100;
        count_v[j] += 1;
    }

    println!("{}", count_v[1] * count_v[4] + count_v[2] * count_v[3]);
}
