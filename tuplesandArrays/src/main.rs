fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup; // Desconstruindo a tupla

    println!("O valor de y é: {}", y);
    println!("O valor de z é: {}", tup.2); // Acessando elementos da tupla
}

