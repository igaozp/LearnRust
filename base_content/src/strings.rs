let greeting = "Hello"; // greeting: &'static str

let mut s = "Hello".to_string(); // mut s: String
s.push_str(", world");

// 字符串索引
let name = "江沢妮可";
let c = name.chars().nth(1);

// 字符串切片，切片按字节偏移
let name = "123456";
let c = &name[0..5];

// 字符串连接
let hello = "Hello ".to_string();
let world = "world!";

let hello_world = hello + world;

let hello = "Hello".to_string();
let world = "world".to_string();

let hello_world = hello + &world;