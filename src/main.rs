use std::io::{self, Write};

fn main() {
    let first = prompt_number("Введите первое число: ");
    let second = prompt_number("Введите второе число: ");

    println!("Результат: {}", first + second);
}

fn prompt_number(msg: &str) -> i32 {
    loop {
        print!("{msg}");
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Сбой ввода");

        match input.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Неверный ввод! Пожалуйста, используйте только цифры.")
            }
        }
    }
}
