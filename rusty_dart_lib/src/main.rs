fn main() {
    let a = 1;
    let b = 2;
    let c = add(a, b);
    println!("{} + {} = {}", a, b, c);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
