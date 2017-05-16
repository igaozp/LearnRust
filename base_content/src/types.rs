// boolean
let x = true;
let y: bool = false;

// char
let x = 'x';
let frog = '🐸';

// number
let x = 42; // i32
let y = 1.0; // f64

// array
let a = [0, 1, 2, 3, 4]; // [i32; 3]
let mut m = [1, 2, 3]; // [i32; 3]
let b = [0; 20];
let complete = &a[..]; // 0, 1, 2, 3, 4
let middle = &a[1..4]; // 1, 2, 3

// tuples
let x = (1, "hello");
let x: (i32, &str) = (1, "hello");
let (x, y, z) = (1, 2, 3);
let tuple = (1, 2, 3);
let x = tuple.0;