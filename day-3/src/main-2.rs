
#![allow(unused)]
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    let x = 2.0;
    let y: f32 = 3.0;
    
    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    let remainder = 43 % 5;

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    let tap: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tap;
    println!("The value of y is: {}", y);


    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    let a = [1,2,3,4,5];
    let first = a[0];
    let second = a[1];
    let index = 10;
    // エラーが出るはず
    let element = a[index];
    println!("The value of element is: {}", element);

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "Octorber", "November", "December"];
    
}
