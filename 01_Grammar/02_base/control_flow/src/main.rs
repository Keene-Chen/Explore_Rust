fn main() {
    // test1();
    // test2();
    // test3();
    // test4();
    // test5();
    test6();
}

/**
 * if 表达式
 */
#[allow(dead_code)]
fn test1() {
    let num: i32 = 1;

    if num < 5 {
        println!("true");
    } else {
        println!("false");
    }
}

/**
 * 在 let 语句中使用 if
 * if else 中的类型必须一致
 */
#[allow(dead_code)]
fn test2() {
    let condition: bool = true;
    let num1: i32 = if condition { 5 } else { 6 };
    println!("{num1}");
}

/**
 * loop 表达式为无限循环
 */
#[allow(dead_code)]
fn test3() {
    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("The result is {result}");
}

/**
 * break continue
 */
#[allow(dead_code)]
fn test4() {
    let mut count: i32 = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining: i32 = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

/**
 * while 语句
 */
#[allow(dead_code)]
fn test5() {
    let mut num = 5;

    while num != 1 {
        num -= 1;
    }

    println!("{num}")
}

/**
 * for 语句
 */
#[allow(dead_code)]
fn test6() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    for i in arr {
        println!("{i}");
    }

    let mut index: usize = 0;
    while index < arr.len() {
        println!("{}", arr[index]);
        index += 1;
    }
}
