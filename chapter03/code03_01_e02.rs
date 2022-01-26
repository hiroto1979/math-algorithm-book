use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }

    let mut v: Vec<usize> = Vec::new();

    // 2の倍数の抽出
    loop {
        if n % 2 == 0 {
            n = n / 2;
            v.push(2);
        } else {
            break;
        }
    }

    // 3以上の奇数の場合のみ走査
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            n = n / i;
            v.push(i);
            continue;
        }
        i += 2;
    }

    if n != 1 {
        v.push(n);
    }

    println!(
        "{}",
        v.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
