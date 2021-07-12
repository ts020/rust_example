fn main() {
    step_1();
    step_2();
    step_3();
}

fn step_1() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn step_2() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("step-2 {}", s)
}
fn change(str: &mut String) {
    str.push_str(", world")
}

fn step_3() {
    let mut s = String::from("hello");

    let r1 = &s; // 問題なし
    let r2 = &s; // 問題なし
    // 一つの変数の参照にmutableとimutableを組み合わせる事はできない。
    // let r3 = &mut s; // 大問題！
    println!("{} {}", r1, r2);
}