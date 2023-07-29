use std::io;

fn get_number(prompt: &str) -> i32 {
    println!("{}", prompt);
    let mut num = String::new();
    io::stdin().read_line(&mut num)
        .expect("Falha ao ler a linha");
    let num: i32 = num.trim().parse()
        .expect("Por favor, insira um número válido!");
    num
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn mul(a: i32, b: i32) -> i32 {
    a * b
}

fn div(a: i32, b: i32) -> i32 {
    a / b
}

fn main() {
    println!("..." );
    println!("Escolha a operação:" );
    println!("1 para adição" );
    println!("2 para subtração" );
    println!("3 para divisão" );
    println!("4 para multiplicação" );
    println!("..." );

    let operation = get_number("Escolha uma operação (1-4):");

    let num1 = get_number("Digite o primeiro número:");
    let num2 = get_number("Digite o segundo número:");

    let result = match operation {
        1 => sum(num1, num2),
        2 => sub(num1, num2),
        3 => {
            if num2 == 0 {
                println!("Erro: divisão por zero é indefinida!");
                return;
            }
            div(num1, num2)
        },
        4 => mul(num1, num2),
        _ => {
            println!("Operação desconhecida!");
            return;
        },
    };

    println!("O resultado é: {}", result);
}
