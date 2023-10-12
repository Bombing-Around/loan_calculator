extern crate clap;
use clap::{Arg, App};
use std::io;

fn main() {
    let matches = App::new("Loan Calculator")
        .arg(Arg::with_name("amortization")
            .short("a")
            .long("amortization")
            .help("Outputs the amortization schedule"))
        .get_matches();

    println!("Loan Calculator");

    let principal = get_input("Enter the principal amount: ");
    let interest_rate = get_input("Enter the annual interest rate: ");
    let months = get_input("Enter the loan duration in months: ");
    let monthly_interest_rate = (interest_rate / 100.0) / 12.0;
    let monthly_payment = principal * monthly_interest_rate / (1.0 - (1.0 + monthly_interest_rate).powf(-months));

    println!("Your monthly payment is: {:.2}", monthly_payment);

    if matches.is_present("amortization") {
        print_amortization_schedule(principal, monthly_interest_rate, months, monthly_payment);
    }
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

fn print_amortization_schedule(principal: f64, monthly_interest_rate: f64, months: f64, monthly_payment: f64) {
    let mut remaining_balance = principal;

    println!("Month | Payment | Interest | Principal | Remaining Balance");

    for month in 1..=(months as usize) {
        let interest_payment = remaining_balance * monthly_interest_rate;
        let principal_payment = monthly_payment - interest_payment;
        remaining_balance -= principal_payment;

        println!("{:<6} | {:<8.2} | {:<9.2} | {:<10.2} | {:<17.2}",
            month, monthly_payment, interest_payment, principal_payment, remaining_balance);
    }
}
