fn main() {
    let mut v = vec![1, 2, 3, 4, 5]; // Criação de um vetor

    v.push(6); // Adiciona um novo elemento ao vetor

    for element in &v {
        println!("Elemento: {}", element);
    }
}