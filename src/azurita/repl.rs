extern crate rustyline;

use self::rustyline::Editor;
use self::rustyline::error::ReadlineError;

use azurita::parser;

pub fn run() {
    println!("Azurita 0.0.1  (Press Ctlr+c to exit)");
    let mut rl = Editor::<()>::new();
    if let Err(_) = rl.load_history("history.txt") {
        println!("No previous history.");
    }
    loop {
        let val = rl.readline("(°)> ");
        match val {
            Ok(input) => {
                rl.add_history_entry(&input);
                match parser::eval(&input) {
                    Err(msg) => println!("Error: {}", msg),
                    _ => {},
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}
