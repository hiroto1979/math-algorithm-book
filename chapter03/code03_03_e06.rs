use proconio::input;

const SUM_NUM: usize = 100000;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut counter = [0_usize; SUM_NUM];

    for i in a {
        counter[i] += 1;
    }

    let mut res = 0;
    for i in 1..(SUM_NUM / 2) {
        res += counter[i] * counter[SUM_NUM - i]
    }
    res += counter[SUM_NUM / 2] * (counter[SUM_NUM / 2] - 1) / 2;

    println!("{}", res);
}
