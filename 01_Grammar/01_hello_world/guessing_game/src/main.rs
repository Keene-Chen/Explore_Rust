use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数字!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("{secret_number}");

    loop {
        println!("请输入你猜测的数字");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了!"),
            Ordering::Greater => println!("太大了!"),
            Ordering::Equal => {
                println!("猜对了!");
                break;
            }
        }
    }
}
