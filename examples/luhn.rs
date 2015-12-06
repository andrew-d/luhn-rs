extern crate luhn;

use std::env::args;

use luhn::Luhn;


fn main() {
    let args: Vec<_> = args().collect();

    if args.len() != 3 {
        println!("Usage: {} <alphabet> <input>", args[0]);
        return;
    }

    let l = match Luhn::new(&args[1]) {
        Ok(l) => l,
        Err(e) => {
            println!("Error creating Luhn: {:?}", e);
            return;
        }
    };

    let ch = match l.generate(&args[2]) {
        Ok(ch) => ch,
        Err(e) => {
            println!("Error generating check digit: {:?}", e);
            return;
        }
    };

    println!("The check digit is: {}", ch);
}
