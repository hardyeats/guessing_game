use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // let : 변수를 만들 때
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}