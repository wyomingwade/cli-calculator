use text_io::read;

const VERBOSE: bool = false;

fn main() {
    // get user input
    println!("Enter the first number: ");
    let a: f64 = read!();
    println!("Enter the operator (+, -, *, /): ");
    let operator: char = read!();
    println!("Enter the second number: ");
    let b: f64 = read!();

    // calculate the result
    let result: f64 = calculate(a, operator, b);
    println!("Answer:");
    // write out the whole expression if verbose is true, otherwise just the answer
    if VERBOSE {
        println!("{} {} {} = {}", a, operator, b, result);
    }
    else {
        println!("{}", result);
    }
    
}

fn calculate(a: f64, operator: char, b: f64) -> f64 {
    let mut result = 0.0;

    match operator {
        '+' | 'p' => result = a + b,
        '-' | 'm' => result = a - b,
        '*' | 't' => result = a * b,
        '/' | 'd' => result = a / b,
        _ => println!("Invalid operator given. Returning 0.0"),
    }

    return result;
}
