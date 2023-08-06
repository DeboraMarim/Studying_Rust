enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let color = Color::Red;

    match color {
        Color::Red => println!("A cor é vermelha!"),
        Color::Green => println!("A cor é verde!"),
        Color::Blue => println!("A cor é azul!"),
    }
}
