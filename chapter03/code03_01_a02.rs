use proconio::input;

fn is_prime(n: usize) -> bool {
    // n を 2 以上の整数とし、n が素数であれば true、素数でなければ false を返す
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
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
