use std::fmt::Debug;

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

trait HasArea {
    fn area(&self) -> f64;

    fn is_larger(&self, &Self) -> bool;
}

// impl Trait for Item
impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    fn is_larger(&self, other: &Self) -> bool {
        self.area() > other.area()
    }
}

// trait bound， 对泛型进行类型标记
fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}


struct Rectangle<T> {
    x: T,
    y: T,
    width: T,
    height: T,
}

// 对结构体进行类型标记
impl<T: PartialEq> Rectangle<T> {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    let mut r = Rectangle {
        x: 0,
        y: 0,
        width: 47,
        height: 47,
    };

    assert!(r.is_square());

    r.height = 42;
    assert!(!r.is_square());
}

// 普通的泛型绑定写法
fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {
    x.clone();
    y:clone();
    println!("{:?}", y);
}
// 使用 where 语法糖的写法
fn bar<T, K>(x: T, y: K) where T: Clone, K: Clone + Debug {
    x.clone();
    y:clone();
    println!("{:?}", y);
}

trait Foo {
    fn is_valid(&self) -> bool;

    // 默认方法，继承者可以不重写此方法
    fn is_invalid(&self) -> bool { !self.is_valid() }
}


trait Foo {
    fn foo(&self);
}

// trait 继承 trait
trait FooBar: Foo {
    fn foobar(&self);
}

struct Baz;

impl FooBar for Baz {
    fn foo(&self) { println!("foo"); }

    fn foobar(&self) { println!("foobar"); }
}