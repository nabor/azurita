extern crate linenoise;

use azurita::parser;

pub fn run() {
    println!("Azurita 0.0.1  (Press Ctlr+c to exit)");
    loop {
        let val = linenoise::input("(°)> ");
        match val {
            Some(input) => {
                linenoise::history_add(input.as_str());
                if input.as_str() == "clear" {
                  linenoise::clear_screen();
                }
                match parser::eval(&input) {
                    Err(msg) => println!("Error: {}", msg),
                    _ => {},
                }
            },
            None => { break },
        }
    }
}
