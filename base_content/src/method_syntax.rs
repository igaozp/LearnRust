struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

// 可以调用的方法
impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    fn grow(&self, increment: f64) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius + increment }
    }

    fn reference(&self) {
        println!("taking self by reference!");
    }

    fn mutable_reference(&mut self) {
        println!("taking self by mutable reference!");
    }

    fn takes_ownership(self) {
        println!("taking ownership of self!");
    }
}

fn main() {
    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("{}", c.area());

    // 方法的链式调用
    let d = c.grow(2.0).area();
    println!("{}", d);
}

impl Circle {
    // 关联函数，可以通过结构体名称直接调用
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
          x: x,
          y: y,
          radius: radius,  
        }
    }
}
fn test() {
    let c = Circle::new(0.0, 0.0, 2.0);
}

// Builder Pattern
struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}

impl CircleBuilder {
    fn new() -> CircleBuilder {
        CircleBuilder { x: 0.0, y: 0.0, radius: 1.0 }
    }

    fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.x = coordinate;
        self
    }

    fn y(&mut self, coordinate: f64) -> &mut CircleBuilder{
        self.y = coordinate;
        self
    }

    fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
        self.radius = radius;
        self
    }

    fn finalize(&self) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius }
    }
}

fn test_circle_builder() {
    let c = CircleBuilder::new().x(1.0).y(2.0).radius(2.0).finalize();

    println!("area: {}", c.area());
    println!("x: {}", c.x);
    println!("y: {}", c.y);
}