// 一个简单的闭包
let plus_one = |x: i32| x + 1;
assert_eq!(2, plus_one(1));

// 一个多行闭包
let plus_two = |x| {
    let mut result: i32 = x;

    result += 1;
    result += 1;

    result
};
assert_eq!(4, plus_two(2));

// 使用 move 强制使闭包取得变量的所有权
let num = 5;
let owns_num = move |x: i32| x + num;

// 闭包作为参数调用
fn call_with_one<F>(some_closure: F) -> i32 where F: Fn(i32) -> i32 {
    some_closure(1)
}
let answer = call_with_one(|x| x + 2);
assert_eq!(3, answer);

