// 枚举类型
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    // 构造器
    Write(String),
}
let x: Message = Message::Move { x: 3, y: 4 };

enum BoardGameTurn {
    Move { squares: i32 },
    Pass,
}
let y: BoardGameTurn = BoardGameTurn::Move { squares: 1 };

// 像函数一样使用构造器
let m = Message::Write("Hello world".to_string());

fn foo(x: String) -> Message {
    Message::Write(x)
}
let m = foo("Hello world".to_string());