// 普通的结构体
struct Point {
    x: i32,
    y: i32,
}

// 带引用的结构体
struct PointRef<'a> {
    x: &'a mut i32,
    y: &'a mut i32,
}

fn main() {
    let mut origin = Point { x: 0, y: 0 };
    origin.x = 1;
    {
        let r = PointRef { x: &mut origin.x, y: &mut origin.y };
        *r.x = 5;
        *r.y = 6;
    }
    println!("The origin is at ({}, {})", origin.x, origin.y);
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}
let mut point = Point3d { x: 0, y: 0, z: 0 };
// 使用其他的结构体变量的数值
point = Point3d { y: 1, ..point };

// 元组结构体
struct Color(i32, i32, i32);
let black = Color(0, 0, 0); 
let black_r = black.0;
let black_g = black.1;
let black_b = black.2;

// 当元组结构体只有一个元素时，可以解构其中的值
struct Inches(i32);
let length = Inches(10);
let Inches(integer_length) = length;
println!("length is {} inches", integer_length);  