fn main() {
    test1();
    test2();
    test3();
}

fn test1() {
    let mut str: String = String::from("hello");

    for i in str.chars() {
        println!("{i}")
    }

    str.push('a');
    str.push_str(",world");
    println!("{str}");
}

/**
 * 引用 &
 */
fn test2() {
    let str: String = String::from("hello");

    let len: usize = cal_len(&str);
    println!("{len}")
}

fn cal_len(str: &String) -> usize {
    str.len()
}

/**
 * 可变引用 &mut
 */
fn test3() {
    let mut str = String::from("hello");
    push_str(&mut str);
}

fn push_str(str: &mut String) {
    str.push_str("world");
    println!("{str}");
}
