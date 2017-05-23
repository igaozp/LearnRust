trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { 
        format!("u8: {}", *self)
    }
}

impl Foo for String {
    fn method(&self) -> String {
        format!("string: {}", *self)
    }
}

fn do_something<T: Foo>(x: T) {
    x.method();
}

fn main() {
    let x = 5u8;
    let y = "Hello".to_string()

    // 静态分发
    do_something(x);
    do_something(y);

    // 通过转型动态分发
    do_something(&x as &Foo);

    // 通过强制转型动态分发
    do_something(&x);
}
