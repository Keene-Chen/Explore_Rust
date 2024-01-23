fn main() {
    // ! error
    // let x: i32 = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    let mut x: i32 = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // 定义常量
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");
}
