// if-let
let option = Some(5);
fn foo(x: i32) { }
fn bar() { }
if let Some(x) = option {
    foo(x); // 匹配成功后，x = 5
} else {
    bar();
}

// while-let
let mut v = vec![1, 3, 5, 7, 11];
while let Some(x) = v.pop() {
    println!("{}", x);
}
