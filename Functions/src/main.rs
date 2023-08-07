fn function_name(parameter1: Type1, parameter2: Type2) -> ReturnType {
    // Function body (code to be executed)
    // Optionally, return a value using the `return` keyword
    return some_value;
}

fn add(a: i32, b: i32) -> i32 {
    let result = a + b;
    return result;
}


fn main() {
    let x = 5;
    let y = 10;
    let sum = add(x, y);
    println!("The sum of {} and {} is: {}", x, y, sum);
}
