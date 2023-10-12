use std::io;

fn main() {
    println!("Loan Calculator");

    // Get principal amount
    let principal = get_input("Enter the principal amount: ");

    // Get annual interest rate
    let interest_rate = get_input("Enter the annual interest rate: ");

    // Get loan duration in months
    let months = get_input("Enter the loan duration in months: ");

    // Calculate monthly interest rate
    let monthly_interest_rate = (interest_rate / 100.0) / 12.0;

    // Calculate monthly payment
    let monthly_payment = principal * monthly_interest_rate / (1.0 - (1.0 + monthly_interest_rate).powf(-months));

    println!("Your monthly payment is: {:.2}", monthly_payment);
}

fn get_input(prompt: &str) -> f64 {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            get_input(prompt)
        },
    }
}
