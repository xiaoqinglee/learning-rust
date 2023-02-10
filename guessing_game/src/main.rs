use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // 1..101 等于 1..=100
    // rand::thread_rng 函数来为我们提供将要使用的特定随机数生成器：它位于当前执行线程的本地环境中，并从操作系统获取 seed
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Secret number is: {}", secret_number);
    loop {
        println!("Guess a number:");
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Fail to read line");
        let number: i64 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number, please retry");
                continue;
            }
        };
        println!("Your guess is: {}", number);
        match number.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
