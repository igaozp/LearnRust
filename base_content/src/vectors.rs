let v = vec![1, 2, 3, 4, 5];
let v = vec![0; 10];

println!("The third element of v is {}", v[2]);

for i in &v {
    println!("A reference to {}", i);
}

for i in &mut v {
    println!("A mutable reference to {}", i);
}

for i in v {
    println!("Take ownership of the vector and it's element {}", i);
}