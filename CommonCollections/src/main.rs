fn main() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Adicionar um novo elemento ao final do vetor
    numbers.push(6);

    // Remover o último elemento do vetor
    numbers.pop();

    // Acessar um elemento do vetor
    let third_number = numbers[2];

    // Iterar sobre os elementos do vetor
    for num in &numbers {
        println!("{}", num);
    }
}

fn main() {
    let mut message = String::from("Hello, ");

    // Concatenar uma string
    message.push_str("Rust!");

    // Obter o comprimento da string
    let length = message.len();

    // Verificar se a string está vazia
    let is_empty = message.is_empty();

    println!("Mensagem: {}", message);
    println!("Comprimento: {}", length);
    println!("Está vazia: {}", is_empty);
}


use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // Inserir pares chave-valor no HashMap
    scores.insert(String::from("Alice"), 100);
    scores.insert(String::from("Bob"), 90);
    scores.insert(String::from("Charlie"), 80);

    // Atualizar um valor existente
    scores.insert(String::from("Bob"), 95);

    // Acessar um valor pelo nome da chave
    if let Some(score) = scores.get("Alice") {
        println!("Pontuação de Alice: {}", score);
    }

    // Verificar se uma chave está no HashMap
    if !scores.contains_key("Dave") {
        println!("Dave não tem pontuação registrada.");
    }
}


use std::collections::HashSet;

fn main() {
    let mut fruits = HashSet::new();

    // Inserir elementos no HashSet
    fruits.insert(String::from("Apple"));
    fruits.insert(String::from("Banana"));
    fruits.insert(String::from("Orange"));

    // Verificar se um elemento está no HashSet
    if fruits.contains("Apple") {
        println!("Tem uma maçã no conjunto.");
    }

    // Remover um elemento do HashSet
    fruits.remove("Orange");

    // Iterar sobre os elementos do HashSet
    for fruit in &fruits {
        println!("{}", fruit);
    }
}
