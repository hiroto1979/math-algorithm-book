use proconio::input;
use itertools::Itertools;

fn main(){
    input!{
        n: usize,
    };

    let mut ans: Vec<usize> = Vec::new();

    if 2 <= n {
        ans.push(2);
    }

    for i in 2..=n {
        let mut i_flag = false;
        for tmp in ans.iter(){
            if i % tmp == 0 {
                i_flag = true;
                break;
            }
        }
        if i_flag == false {
            ans.push(i);
        }
    }

    println!{"{}", ans.iter().join(" ")};
}