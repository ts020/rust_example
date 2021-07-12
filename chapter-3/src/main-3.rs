fn main() {
    another_function(5, 6);
    second();
    let _five = five();
    println!("five {}", _five);
    let x = plus_one(5);
    println!("plug one x is: {}", x);
    
}

fn another_function(x: i32, y: i32) {
    println!("another_function value of x is: {} {} ", x, y);
}

fn second() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("second value of y is {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}