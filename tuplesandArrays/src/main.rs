fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup; // Desconstruindo a tupla

    println!("O valor de y é: {}", y);
    println!("O valor de z é: {}", tup.2); // Acessando elementos da tupla
}

fn main2() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    let first = arr[0]; // Acessando o primeiro elemento do array
    let second = arr[1]; // Acessando o segundo elemento do array

    println!("O primeiro elemento é: {}", first);
    println!("O segundo elemento é: {}", second);
}