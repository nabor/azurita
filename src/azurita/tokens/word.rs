use azurita::tokens::{Tokenizer, State, Separator, Symbol, Whitespace};

static VALID_CHARS: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";

pub struct Word {}

impl Word {
    pub fn is_valid(c: char) -> bool {
        return VALID_CHARS.contains(c);
    }

    pub fn handle(c: char, tokenizer: &mut Tokenizer) -> bool {
        if Separator::is_valid(c) {
            tokenizer.add_token();
            tokenizer.set_state(State::Separator);
        } else if Symbol::is_valid(c) {
            tokenizer.add_token();
            tokenizer.add_to_buffer(c);
            tokenizer.set_state(State::Symbol);
        }else if Whitespace::is_valid(c) {
            tokenizer.add_token();
            tokenizer.add_to_buffer(c);
            tokenizer.set_state(State::Whitespace);
        } else if Word::is_valid(c) {
            tokenizer.add_to_buffer(c);
        } else {
            return false;
        }
        return true;
    }
}
