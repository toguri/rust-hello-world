use std::io;

fn main() {
    // 整数a、b、cを読み取る
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("failed to read a");
    let a: i32 = a.trim().parse().expect("failed to parse a");

    let mut b_c = String::new();
    io::stdin()
        .read_line(&mut b_c)
        .expect("failed to read b and c");
    let mut b_c = b_c.split_whitespace();
    let b: i32 = b_c.next().unwrap().parse().expect("failed to parse b");
    let c: i32 = b_c.next().unwrap().parse().expect("failed to parse c");

    // 文字列sを読み取る
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("failed to read s");
    let s = s.trim();

    // a+b+cとsを空白で区切って1行に出力する
    println!("{} {}", a + b + c, s);
}
