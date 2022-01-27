use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    
    // sort 関数により、配列の中身が [3,1,4,1,5] から [1,1,3,4,5] に書き換えられる
    a.sort();

    println!("{:?}", a);
}
