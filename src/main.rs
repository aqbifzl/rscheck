use std::env::args;

use rscheck::spell_checker::{
    options::{show_manual, Options},
    spell_check,
};

fn main() {
    match Options::create(args()) {
        Ok(options) => {
            if spell_check(&options).is_err() {
                println!("Unknown error occurred while checking");
            }
        }
        Err(error) => {
            show_manual();
            println!("\n\n{error}");
        }
    }
}
