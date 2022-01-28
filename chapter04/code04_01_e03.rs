use proconio::input;

fn main(){
    input!{
        c1 : [f64; 3],
        c2 : [f64; 3],
    }

    // 円の中心間距離を求める
    let dist = ((c1[0] - c2[0]) * (c1[0] - c2[0]) + (c1[1] - c2[1]) * (c1[1] - c2[1])).sqrt();
 
    if c1[2] - c2[2] < - dist || dist < c1[2] - c2[2] {
        println!("{}", 1);
    } else if -(c1[2] - c2[2]) == dist || dist == c1[2] - c2[2] {
        println!("{}", 2);
    } else if dist < c1[2] + c2[2] {
        println!("{}", 3);
    } else if dist == c1[2] + c2[2] {
        println!("{}", 4);
    } else {
        println!("{}", 5);
    }
}