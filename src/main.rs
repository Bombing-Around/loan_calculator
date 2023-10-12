use std::io;

fn main() {
    println!("Welcome to the loan calculator!");

    let principal: f64 = get_principal();
    let interest_rate: f64 = get_interest_rate();
    let loan_term: u32 = get_loan_term();

    let loan_amount = calculate_loan_amount(principal, interest_rate, loan_term);

    println!("Your loan amount is: {}", loan_amount);
}

fn get_principal() -> f64 {
    println!("Enter the principal amount:");
    let mut principal = String::new();
    io::stdin().read_line(&mut principal).unwrap();
    principal.trim().parse().unwrap()
}

fn get_interest_rate() -> f64 {
    println!("Enter the interest rate:");
    let mut interest_rate = String::new();
    io::stdin().read_line(&mut interest_rate).unwrap();
    interest_rate.trim().parse().unwrap()
}

fn get_loan_term() -> u32 {
    println!("Enter the loan term:");
    let mut loan_term = String::new();
    io::stdin().read_line(&mut loan_term).unwrap();
    loan_term.trim().parse().unwrap()
}

fn calculate_loan_amount(principal: f64, interest_rate

==================================
