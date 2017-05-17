loop {
    println!("Loop forever!");
}

let mut x = 5;
let mut done = false;

while !done {
    x += x - 3;

    println!("{}", x);

    if x % 5 == 0 {
        done = true;
    }
}

for var in expression {
    // do something
}

for (index, value) in (5..10).enumerate() {
    println!("index = {} and value = {}", index, value);
}

'outer: for x in 0..10 {
    'inner: for y in 0.10 {
        if x % 2 == 0 { continue 'outer; }
        if y % 2 == 0 { continue 'inner; }
        println!("x: {}, y: {}", x, y);
    }
}