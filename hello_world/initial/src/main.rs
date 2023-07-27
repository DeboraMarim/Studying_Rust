// Sintaxe Básica

// Funções: Em Rust, você define funções usando a palavra-chave fn. 
// Por exemplo, a função main é o ponto de entrada do seu programa:

fn hallo() {
    println!("Hello, World!");
}

// Blocos de código: 
// Em Rust, você usa chaves {} para definir blocos de código. 
// As variáveis em Rust têm um conceito chamado "escopo", que é determinado pelo bloco em que estão.

// Instruções e Expressões: 
// Em Rust, quase tudo é uma expressão. 
// Expressões retornam um valor e podem ser parte de instruções. 
// Instruções não retornam um valor e são seguidas por um ponto-e-vírgula.

// Variáveis e Tipos de Dados

// Variáveis Imutáveis: Por padrão, as variáveis em Rust são imutáveis. 
// Isso significa que uma vez que um valor é vinculado a um nome, você não pode mudar esse valor. 
// Por exemplo:

// let x = 5;
// x = 6; // isso vai causar um erro

// Variáveis Mutáveis: 
// Se você precisar alterar o valor de uma variável, 
// você pode usar a palavra-chave mut:

// let mut x = 5;
// x = 6; // isso é ok


// Tipos de Dados: Rust é uma linguagem estaticamente tipada, 
// o que significa que o tipo de cada variável 
// deve ser conhecido em tempo de compilação. 
// Rust tem uma inferência de tipos, então na maioria das vezes, 
// o compilador Rust pode inferir o tipo de variáveis. No entanto, 
// você também pode especificar explicitamente o tipo, se necessário:

// let x: i32 = 5;

// Neste exemplo, i32 é um tipo de dados de inteiro de 32 bits.

// Esses são os conceitos mais básicos de sintaxe e variáveis em Rust. 
// À medida que você se aprofundar em Rust, você encontrará
//  conceitos mais complexos, como empréstimo e propriedade, 
//  que são partes fundamentais da maneira como Rust lida com a memória.


// EXEMPLO DE FUNÇÃO: FORMULA DE BHASKARA

use std::f64;

fn bhaskara(a: f64, b: f64, c: f64) -> (f64, f64) {
    let delta = b.powi(2) - 4.0*a*c;
    
    if delta < 0.0 {
        panic!("No real roots exist for the given coefficients");
    }
    
    let root_delta = delta.sqrt();
    let root1 = (-b + root_delta) / (2.0*a);
    let root2 = (-b - root_delta) / (2.0*a);
    
    (root1, root2)
}

fn main() {
    let (root1, root2) = bhaskara(1.0, -3.0, 2.0);
    
    println!("The roots of the equation are: {} and {}", root1, root2);
}
