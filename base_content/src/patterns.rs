// 简单的模式匹配
let x = 1;
match x {
    y => println!("x: {} y: {}", x, y), // y = 1
}

// 多重模式
match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}

// 通过模式匹配解构结构体
struct Point {
    x: i32,
    y: i32,
}
let origin = Point { x: 0,  y: 0 };
match origin {
    Point { x, y } => println!("({}, {})", x, y),
}
let point = Point { x: 2. y: 3 };
match point {
    Point { y, .. } => println!("y is {}", y),
}

// 忽略绑定
fn coordinate() -> (i32, i32, i32) {
    (1, 2, 3)
}
let (x, _, z) = coordinate();

// 忽略多个绑定
enum OptionalTuple {
    Value(i32, i32, i32),
    Missing,
}
let x = OptionalTuple::Value(5, -2, 3);
match x {
    OptionalTuple::Value(..) => println!("Got a tuple"),
    OptionalTuple::Missing => println!("No such luck"),
}

// ref 引用
match x {
    ref r => println!("Got a reference to {}", r),
}
// ref 可变引用
match x {
    ref mut mr => println!("Got a mutable reference to {}", mr),
}

// 可以绑定一定范围的值
match x {
    1 ... 5 => println!("one through five"),
    _ => println!("anything"),
}

let emoji = '😀';
match emoji {
    'a' ... 'j' => println!("early letter"),
    'k' ... 'z' => println!("late letter"),
    _ => println!("something else"),
}

// 把值绑定在相应的变量上
match x {
    e @ 1 ... 5 => println!("got a range element {}", e),
    _ => println!("anything"),
}

match x {
    e @ 1 ... 5 | e @ 8 ... 10 => println!("got a range element {}", e),
    _ => println!("anything"),
}

// 对一个复杂的数据结构解构
struct Person {
    name: Option<String>,
}
let name = "Steve".to_string();
let x: Option<Person> = Some(Person { name: Some(name) });
match x {
    Some(Person { name: ref a @ Some(_), .. }) => println!("{:?}", a),
    _ => {}
}

// 使用 if 控制匹配 
enum OptionalInt {
    Value(i32),
    Missing,
}
let x = OptionalInt::Value(5);
match x {
    OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
    OptionalInt::Value(..) => println!("Got an int!"),
    OptionalInt::Missing => println!("No such luck."),
}