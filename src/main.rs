use std::io;

mod parser;
mod types;

pub use types::*;

fn main() -> Result<(), &'static str> {
    let mut input = String::new();
    let mut cart = Cart::new();
    let stdin = io::stdin();

    loop {
        let bytes = stdin.read_line(&mut input).map_err(|_| "I/O error.")?;

        if bytes == 0 {
            break;
        }

        match parser::parse(&input) {
            Ok((quantity, item)) => {
                cart.add(quantity, item);
            }
            Err(e) => {
                eprintln!("Failed to parse input: {}. Error {}", input, e);
            }
        }

        input.clear();
    }

    for (item, price) in cart.receipt().ok_or_else(|| "Overflow error")? {
        println!("{} {:.2}", item, price);
    }

    Ok(())
}
