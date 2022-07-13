use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is {}", secret_number);
    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); //mut:  가변의

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 타입 변경
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            // 숫자 비교
            Ordering::Less => println!("Too small!"), // 세미콜론 아님
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
