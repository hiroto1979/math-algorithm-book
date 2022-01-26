use proconio::input;

fn is_prime(n: usize) -> bool {
    // n を 2 以上の整数とし、n が素数であれば true、素数でなければ false を返す
    for i in 2..n {
        // n が i で割り切れた場合、この時点で素数ではないと分かる
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn main(){
    input!{
        n: usize,
    };

    let ans = is_prime(n);
    if ans {
        println!{"{} is prime.", n};
    } else {
        println!{"{} is not prime.", n};
    }
}
