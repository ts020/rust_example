

fn main() {
    let number = 3;
    if number != 0 {
        println!("number was somthing other than zero");
    }
    next();
    three();
    four();
    loop_example();
    loop_print();
    loop_iterator();
    loop_revert();
}

fn next() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}


fn three() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}

fn four() {
    let condition = true;
    let number = if condition {
        "5"
    } else {
        "six"
    };
    println!("The value of number is: {}", number);
}

fn loop_example() {
    let mut number = 3;
    while number != 0 {
        number -= 1;
    }
    println!("LISTOFF!!!");
}

fn loop_print() {
    let a = [10,20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }
}

fn loop_iterator () {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("iterator value is: {}", element / 10);
    }
}

fn loop_revert() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFFTOFF!!!!")
}