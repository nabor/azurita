use azurita::tokens::{Tokenizer, State, Separator, Whitespace, Symbol};

pub struct Number {}

impl Number {
    pub fn is_valid(c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    pub fn handle(c: char, tokenizer: &mut Tokenizer) -> bool {
        if Number::is_valid(c) {
            tokenizer.add_to_buffer(c);
        } else if Separator::is_valid(c) {
            tokenizer.add_token();
            tokenizer.set_state(State::Separator);
        } else if Symbol::is_valid(c) {
            tokenizer.add_token();
            tokenizer.add_to_buffer(c);
            tokenizer.set_state(State::Symbol);
        } else if Whitespace::is_valid(c) {
            tokenizer.add_token();
            tokenizer.set_state(State::Whitespace);
        } else {
            return false;
        }
        return true;
    }
}
