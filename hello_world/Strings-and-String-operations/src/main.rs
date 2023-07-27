// Criando novas strings
let mut s = String::new();
let data = "initial contents";
let s = data.to_string();
let s = "initial contents".to_string();

// Atualizando uma string
let mut s = String::from("foo");
s.push_str("bar"); // s agora é "foobar"
s.push('l'); // s agora é "foobarl"

// Concatenando com o operador + ou o macro format!
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s3 é "Hello, world!"

let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{}-{}-{}", s1, s2, s3); // s é "tic-tac-toe"

// Acessando elementos de string
let hello = "Здравствуйте";
let s = &hello[0..4]; // s é "Зд"

// Iterando sobre strings
for c in "नमस्ते".chars() {
    println!("{}", c);
}
for b in "नमस्ते".bytes() {
    println!("{}", b);
}