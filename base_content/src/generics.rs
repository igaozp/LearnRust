// 一个简单的泛型示例
enum Option<T> {
    Some(T),
    None,
}
let x: Option<i32> = Some(5);
let y: Option<f64> = Some(5.0f64);

// 多个泛型
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 泛型函数
fn takes_two_of_the_same_things<T>(x: T, y: T) {
    // do something
}

fn takes_two_things<T, U>(x: T, y: U) {
    // do something
}

// 泛型结构体
struct Point<T> {
    x: T,
    y: T,
}

let int_origin = Point { x: 0, y: 0 };
let float_origin = Point { x: 0.0, y: 0.0 };