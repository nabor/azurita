use std::fmt;
use azurita::tokens::{Number, Separator, Symbol, Text, Whitespace, Word};

pub enum Token {
    Number(String),
    Separator(String),
    Symbol(String),
    Text(String),
    Word(String),
}

pub enum State {
    Number,
    Separator,
    Symbol,
    Text,
    Whitespace,
    Word,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            State::Number => write!(f, "State::Number"),
            State::Separator => write!(f, "State::Separator"),
            State::Symbol => write!(f, "State::Symbol"),
            State::Text => write!(f, "State::Text"),
            State::Whitespace => write!(f, "State::Whitespace"),
            State::Word => write!(f, "State::Word"),
        }
    }
}

pub struct Tokenizer {
    valid: bool,
    state: State,
    error: &'static str,
    buffer: String,
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new() -> Tokenizer{
        return Tokenizer{
            valid: true,
            error: "Invalid Syntax",
            state: State::Whitespace,
            buffer: String::new(),
            tokens: Vec::<Token>::new(),
        };
    }
    pub fn add_to_buffer(&mut self, c: char) {
        self.buffer.push(c);
    }

    pub fn add_token(&mut self) {
        let buffer = self.buffer.clone();
        self.buffer = String::new();
        match self.state {
            State::Number => self.tokens.push(Token::Number(buffer)),
            State::Separator => self.tokens.push(Token::Separator(buffer)),
            State::Symbol => self.tokens.push(Token::Symbol(buffer)),
            State::Text => self.tokens.push(Token::Text(buffer)),
            State::Whitespace => {},
            State::Word => self.tokens.push(Token::Word(buffer)),
        }
    }

    pub fn set_state(&mut self, s: State) {
        self.state = s;
    }

    pub fn set_error(&mut self, e: &'static str) {
        self.error = e;
    }

    pub fn parse(mut self, input: &str) -> Result<Vec<Token>, &'static str> {
        for (i, c) in input.chars().enumerate() {
            match self.state {
                State::Number => {
                    self.valid = Number::handle(c, &mut self);
                }
                State::Separator => {
                    self.valid = Separator::handle(c, &mut self);
                }
                State::Symbol => {
                    self.valid = Symbol::handle(c, &mut self);
                }
                State::Text => {
                    self.valid = Text::handle(c, &mut self);
                }
                State::Whitespace => {
                    self.valid = Whitespace::handle(c, &mut self);
                }
                State::Word => {
                    self.valid = Word::handle(c, &mut self);
                }
            }
            if !self.valid {
                println!("{}", &input[0..input.len()-1]);
                let mut caret = String::new();
                for _ in 0..i {
                    caret.push(' ');
                }
                caret.push('^');
                println!("{}", caret);
                return Err(self.error);
            }
        }
        if self.buffer.len() > 0 {
            self.add_token();
        }
        return Ok(self.tokens);
    }
}
