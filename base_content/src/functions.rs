fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn add_one(x: i32) -> i32 {
    // return x + 1
    x + 1
}

fn foo(x: i32) -> i32 {
    return x;
    // never run this code
    x + 1
}

// diverging functions 发散函数 
fn diverges() -> ! {
    panic!("This function never returns!");
}
let x: i32 = diverges();

// pointer functions 指针函数
fn plus_one(i: i32) -> i32 {
    i + 1
}
let f: fn(i32) -> i32 = plus_one;
let f = plus_one
let six = f(5);