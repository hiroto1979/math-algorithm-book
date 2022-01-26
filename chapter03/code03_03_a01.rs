use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize; n],
    }

    let mut counter: usize = 0;
    for i1 in 0..n {
        for i2 in i1+1..n {
            for i3 in i2+1..n {
                for i4 in i3+1..n {
                    for i5 in i4+1..n {
                        if a[i1] + a[i2] + a[i3] + a[i4] + a[i5] == 1000 {
                            counter += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", counter);
}
