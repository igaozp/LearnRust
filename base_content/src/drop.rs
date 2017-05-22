struct Firework {
    strength: i32,
}

impl Drop for Firework {
    fn drop(&mut self) {
        println!("BOOM times {}!!!", self.strength);
    }
}

fn test_drop() {
    let firecraker = Firework { strength: 1 };
    let tnt = Firework { strength: 100 };
} // Drop 的代码执行，
// BOOM times 100!!!
// BOOM times 1!!!