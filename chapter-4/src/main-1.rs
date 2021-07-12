#![allow(unused)]
fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    println!("length {}", s.len());
    string_clone();
    premitive_clone();
    scope_string();
    return_scope();
    return_taple();
}

fn string_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn premitive_clone() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}


fn scope_string() {
    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("takes_ownership {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("makes_copy {}", some_integer);
}


fn return_scope() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some = String::from("hello");
    some
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}



fn return_taple() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}