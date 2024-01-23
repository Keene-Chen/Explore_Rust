fn main() {
    let a: i32 = 10;
    let b: i32 = 10;
    // let c: i32 = add(a, b);

    println!("{}", add(a, b));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
