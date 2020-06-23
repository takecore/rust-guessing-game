use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("秘密の数字を推測してね!");

    let number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("推測を入力してね");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("問題が発生しました");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("あなたの推測は {}", guess);

        match guess.cmp(&number) {
            Ordering::Less => println!("小さいよ"),
            Ordering::Greater => println!("大きいよ"),
            Ordering::Equal => {
                println!("正解！");
                break;
            }
        }
    }
}
