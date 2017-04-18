use azurita::Token;
use azurita::Expresion;
use azurita::expresions::{AzInteger, AzOperator};

pub fn generate(tokens: Vec<Token>) -> Result<(), &'static str>{
    let debug = false;
    let mut stack = Vec::<Expresion>::new();
    for token in tokens {
        match token {
            Token::Number(number) => {
                if debug { println!("Number: {}", number) }
                match stack.pop() {
                    Some(mut e) => {
                        match e {
                            Expresion::Operation{ operator: _, left: ref l, right: ref mut r } => {
                                match **l {
                                    Expresion::Number(_) => {
                                        *r = Box::new(Expresion::Number(AzInteger::new(number)));
                                    },
                                    _ => return Err("Invalid Syntax"),
                                }
                            },
                            _ => return Err("Invalid Syntax"),
                        }
                        stack.push(e);
                    },
                    None => { stack.push(Expresion::Number(AzInteger::new(number))) },
                }
            },
            Token::Separator(separator) => {
                if debug { println!("Separator: {}", separator) }
            },
            Token::Symbol(symbol) => {
                if debug { println!("Symbol: {}", symbol) }
                match stack.pop() {
                    Some(e) => {
                        match e {
                            Expresion::Number(_) => {
                                stack.push(Expresion::Operation{
                                    operator: AzOperator::new(symbol).expect("Invalid operator"),
                                    left: Box::new(e),
                                    right: Box::new(Expresion::Void)
                                });
                            },
                            _ => return Err("Invalid Syntax"),
                        }
                    },
                    None => return Err("Invalid Syntax"),
                }
            },
            Token::Text(text) => {
                if debug { println!("Text: \"{}\"", text) }
            },
            Token::Word(word) => {
                if debug { println!("Word: {}", word) }
            },
        }
    }
    match stack.pop() {
        Some(e) => {
            let r = try!(e.eval());
            println!("{}", r);
            return Ok(());
        },
        None => return Err("Invalix Syntax")
    }
}
