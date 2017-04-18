use azurita::tokens::{Tokenizer, State, Number, Separator, Symbol, Text, Word};

pub struct Whitespace {}

impl Whitespace {
    pub fn is_valid(c: char) -> bool {
        return c == ' ' || c == '\t' || c == '\n' || c == '\r';
    }

    pub fn handle(c: char, tokenizer: &mut Tokenizer) -> bool {
        if Number::is_valid(c) {
            tokenizer.add_to_buffer(c);
            tokenizer.set_state(State::Number);
        } else if Separator::is_valid(c) {
            tokenizer.add_to_buffer(c);
            tokenizer.set_state(State::Separator);
        } else if Symbol::is_valid(c) {
            tokenizer.add_to_buffer(c);
            tokenizer.set_state(State::Symbol);
        } else if Text::is_delimiter(c) {
            tokenizer.set_state(State::Text);
        } else if Whitespace::is_valid(c) {
        } else if Word::is_valid(c)  {
            tokenizer.add_to_buffer(c);
            tokenizer.set_state(State::Word);
        } else {
            return false;
        }
        return true;
    }
}
