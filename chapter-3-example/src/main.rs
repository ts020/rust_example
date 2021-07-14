use core::f64;
use std::io;

fn main() {
    let counts = [1,2,3];

    for count in counts.iter() {
        let mut buf  = String::new();
        println!("{}回目の入力よろ", count);
        io::stdin().read_line(&mut buf).expect("読むの失敗した");
        let price: i64 = match  buf.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("変換エラーでちった {}", &e);
                0
            },
        };
        println!("{}件目 price: {}, tax: {}",count, price, tax(price));
    }
}


fn tax(price: i64) -> i64 {
    ((price as f64) * 0.1).floor() as i64
}
