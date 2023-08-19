use std::io;
use chrono::{Utc, Datelike, Timelike};

// Função para exibir o menu
fn display_menu() {
    println!("Escolha uma opção:");
    println!("1 - Calculadora");
    println!("2 - Quiz");
    println!("3 - Hora atual");
    println!("4 - Data de nascimento");
    println!("0 - Sair");
}

// Função para calcular a idade com base na data de nascimento
fn calculate_age(birth_year: i32) -> i32 {
    let current_year = Utc::now().year();
    current_year - birth_year
}

// Função para o quiz
fn run_quiz() {
    let questions = vec![
        ("Qual é a capital da França?", "Paris"),
        ("Quanto é 2 + 2?", "4"),
        // Adicione mais perguntas e respostas aqui
    ];

    let mut score = 0;

    for (question, answer) in questions {
        println!("{}", question);
        let mut user_answer = String::new();
        io::stdin().read_line(&mut user_answer).expect("Falha ao ler a linha");
        if user_answer.trim() == answer {
            println!("Resposta correta!");
            score += 1;
        } else {
            println!("Resposta incorreta. A resposta correta era: {}", answer);
        }
    }

    println!("Você acertou {} de {} perguntas!", score, questions.len());
}

fn main() {
    loop {
        display_menu();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Falha ao ler a linha");

        match choice.trim() {
            "1" => {
                println!("Digite o primeiro número:");
                let mut num1 = String::new();
                io::stdin().read_line(&mut num1).expect("Falha ao ler a linha");
                
                println!("Digite o segundo número:");
                let mut num2 = String::new();
                io::stdin().read_line(&mut num2).expect("Falha ao ler a linha");
                
                let num1: f64 = num1.trim().parse().expect("Por favor, insira um número válido!");
                let num2: f64 = num2.trim().parse().expect("Por favor, insira um número válido!");

                println!("Escolha uma operação: +, -, *, /");
                let mut operator = String::new();
                io::stdin().read_line(&mut operator).expect("Falha ao ler a linha");

                let result = match operator.trim() {
                    "+" => num1 + num2,
                    "-" => num1 - num2,
                    "*" => num1 * num2,
                    "/" => num1 / num2,
                    _ => {
                        println!("Operação inválida");
                        continue;
                    }
                };

                println!("Resultado: {}", result);
            },
            "2" => {
                println!("Iniciando o Quiz...");
                run_quiz();
            },
            "3" => {
                let now = Utc::now();
                println!("Hora atual: {}:{}:{}", now.hour(), now.minute(), now.second());
            },
            "4" => {
                println!("Digite o ano do seu nascimento:");
                let mut birth_year = String::new();
                io::stdin().read_line(&mut birth_year).expect("Falha ao ler a linha");

                let birth_year: i32 = birth_year.trim().parse().expect("Por favor, insira um ano válido!");

                let age = calculate_age(birth_year);
                println!("Sua idade é: {}", age);
            },
            "0" => {
                println!("Saindo do programa. Até logo!");
                break;
            },
            _ => {
                println!("Opção inválida. Por favor, escolha uma opção válida.");
            }
        }
    }
}
