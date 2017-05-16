fn variable_and_scope() {
    let (x, y) = (1, 2)

    let mut z: i32 = 5;

    let x: i32 = 8;
    {
        println!("{}", x); // Prints "8".
        let x = 12;
        println!("{}", x); // Prints "12".
    }
    println!("{}", x); // Prints "8".
    let x = 42;
    println!("{}", x); // Prints "42".
}
