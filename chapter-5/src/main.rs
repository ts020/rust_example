fn main() {
    step_1();
    step_2();
    step_3();
}

fn step_1() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("1こ目のワード {}", word);
    s.clear();
    println!("空のはず [{}]", s);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item ==b' ' {
            return &s[0..i];
        } 
    }
    return &s[..];
}

fn step_2() {
    let s = String::from("hello world");
    println!("2文字目から 6文字目 {}", &s[1..7]);
    println!("4文字目以降 {}", &s[3..]);
    println!("頭から8文字目 {}", &s[..8]);
}

fn step_3() {
    let s = String::from("hello world");
    println!("2こ目のワード {}", second_word(&s));
}
fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item ==b' ' {
            return &s[i..];
        } 
    }
    return &s[..];
}