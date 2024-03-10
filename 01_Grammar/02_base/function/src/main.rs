fn main() {
    let a: i32 = 10;
    let b: i32 = 10;
    // let c: i32 = add(a, b);

    println!("{}", add(a, b));
    println!("{}", test());
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

/**
 * 在rust中函数的返回值是块中最后一个表达式的值
 */
fn test() -> i32 {
    6
}
