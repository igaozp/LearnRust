struct Foo<'a> {
    x: &'a i32;
}

fn main() {
    let x;

    {
        let y = &5;
        let f = Foo { x: y };
        x = &f.x
    }

    println!("{}", x);
}

// 多个生命周期
fn x_or_y<'a>(x: &'a str, y: &'a str) -> &'a str {
    x
}

fn x_or_y<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}

// 全局字符串
let x: &'static str = "Hello, world."; 

// 省略生命周期和非省略的对比
fn substr(s: &str, until: u32) -> &str; // elided
fn substr<'a>(s: &'a str, until: u32) -> &'a str; // expanded

fn get_str() -> &str; // ILLEGAL, no inputs

fn frob(s: &str, t: &str) -> &str; // ILLEGAL, two inputs
fn frob<'a, 'b>(s: &'a str, t: &'b str) -> &str; // Expanded: Output lifetime is ambiguous

fn get_mut(&mut self) -> &mut T; // elided
fn get_mut<'a>(&'a mut self) -> &'a mut T; // expanded

fn args<T: ToCStr>(&mut self, args: &[T]) -> &mut Command; // elided
fn args<'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command; // expanded

fn new(buf: &mut [u8]) -> BufWriter; // elided
fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>; // expanded