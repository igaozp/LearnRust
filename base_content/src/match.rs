// 简单的模式匹配
let x = 5;
match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => println!("something else"),
}

// 通过模式匹配为变量赋值
let number = match x {
    1 => "one",
    2 => "two",
    3 => "three",
    4 => "four",
    5 => "five",
    _ => "something else",
};

// 匹配枚举
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

fn quit() {}
fn change_color(r: i32, g: i32, b: i32) {}
fn move_cursor(x: i32, y: i32) {}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => quit(),
        Message::ChangeColor(r, g, b) => change_color(r, g, b),
        Message::Move { x, y: new_name_for_y } => move_cursor(x, new_name_for_y),
        Message::Write(s) => println!("{}", s),
    };
}