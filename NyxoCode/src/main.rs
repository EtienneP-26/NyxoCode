//
// EtienneP-26 PROJECT, 2025
// NyxoCode
// File description:
// main file
//

use colored::*;
use std::io::{self, Write};
use std::process;

///// Colors /////

fn printc(text: &str, color: Color) {
    print!("{}", text.color(color));
}

fn main() {
    printc("NyxoCode ", Color::Yellow);
    printc("v0.1.0 ", Color::Green);
    printc("by EtienneP-26\n", Color::Cyan);
    printc("Welcome to NyxoCode!\n", Color::White);

    loop {
        printc("NyxoCode-? >> ", Color::Yellow);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();
        println!("You entered: {}", input);
        if input == "exit" {
            printc("Exiting NyxoCode...\n", Color::Cyan);
            process::exit(0);
        }
    }
}
