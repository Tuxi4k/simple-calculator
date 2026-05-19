use std::io::{self, Write};

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LPar,
    RPar,
}

impl Token {
    fn to_char(&self) -> Option<char> {
        match self {
            Token::Plus => Some('+'),
            Token::Minus => Some('-'),
            Token::Multiply => Some('*'),
            Token::Divide => Some('/'),
            _ => None,
        }
    }
}

fn main() {
    println!("=== CLI Математический Калькулятор ===");
    println!("Введите выражение (или 'exit' для выхода):");

    loop {
        print!("> ");
        let _ = io::stdout().flush();

        let mut raw_input = String::new();
        if io::stdin().read_line(&mut raw_input).is_err() {
            println!("Ошибка чтения строки");
            continue;
        }

        let trimmed = raw_input.trim();
        match trimmed {
            "exit" => {
                println!("До свидания!");
                break;
            }
            "" => continue,
            _ => {}
        }

        let input = trimmed.replace(" ", "");

        let tokens = match tokenize(&input) {
            Ok(t) => t,
            Err(e) => {
                println!("Ошибка сканирования: {e}");
                continue;
            }
        };

        match evaluate(tokens) {
            Ok(result) => println!("Результат: {result}"),
            Err(e) => println!("Ошибка вычисления: {e}"),
        }
    }
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '(' => {
                tokens.push(Token::LPar);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RPar);
                chars.next();
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Multiply);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Divide);
                chars.next();
            }

            '-' | '0'..='9' | '.' => {
                let is_unary_minus = c == '-'
                    && (tokens.is_empty()
                        || matches!(
                            tokens.last(),
                            Some(Token::Plus)
                                | Some(Token::Minus)
                                | Some(Token::Multiply)
                                | Some(Token::Divide)
                                | Some(Token::LPar)
                        ));

                if c == '-' && !is_unary_minus {
                    tokens.push(Token::Minus);
                    chars.next();
                    continue;
                }

                let mut num_str = String::new();
                if is_unary_minus {
                    num_str.push('-');
                    chars.next();
                }

                while let Some(&next_c) = chars.peek() {
                    if next_c.is_numeric() || next_c == '.' {
                        num_str.push(next_c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                let num = num_str
                    .parse::<f64>()
                    .map_err(|_| format!("Неверный формат числа: {num_str}"))?;
                tokens.push(Token::Number(num));
            }
            _ => return Err(format!("Неизвестный символ в выражении: {c}")),
        };
    }
    Ok(tokens)
}

fn evaluate(tokens: Vec<Token>) -> Result<f64, String> {
    let mut numbers = Vec::new();
    let mut operators = Vec::new();

    for token in tokens {
        match token {
            Token::Number(n) => numbers.push(n),
            Token::LPar => operators.push('('),
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide => {
                let current_op = token.to_char().unwrap();

                while let Some(&top_op) = operators.last() {
                    if top_op != '(' && precedence(top_op) >= precedence(current_op) {
                        apply_operator(&mut operators, &mut numbers)?;
                    } else {
                        break;
                    }
                }
                operators.push(current_op);
            }
            Token::RPar => {
                while operators.last() != Some(&'(') {
                    apply_operator(&mut operators, &mut numbers)?;
                }
                operators.pop();
            }
        }
    }

    while !operators.is_empty() {
        if let Some(&top_op) = operators.last() {
            if top_op == '(' || top_op == ')' {
                return Err("Ошибка: пропущена закрывающая скобка".to_string());
            }
        }
        apply_operator(&mut operators, &mut numbers)?;
    }

    numbers.pop().ok_or("Пустое выражение".to_string())
}

fn precedence(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

fn apply_operator(operators: &mut Vec<char>, numbers: &mut Vec<f64>) -> Result<(), String> {
    let op = operators.pop().ok_or("Ошибка: нехватка операторов")?;
    let b = numbers.pop().ok_or("Ошибка: нехватка чисел для операции")?;
    let a = numbers.pop().ok_or("Ошибка: нехватка чисел для операции")?;

    let result = match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' if b != 0.0 => a / b,
        '/' => return Err("Ошибка: деление на ноль!".to_string()),
        _ => return Err(format!("Ошибка: неизвестный оператор '{op}'")),
    };

    numbers.push(result);
    Ok(())
}
