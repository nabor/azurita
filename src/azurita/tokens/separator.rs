use azurita::tokens::{Tokenizer, State, Number, Symbol, Text, Whitespace};

static VALID_CHARS: &'static str = ",";

pub struct Separator {}

impl Separator {
    pub fn is_valid(c: char) -> bool {
        return VALID_CHARS.contains(c);
    }
    pub fn handle(c: char, tokenizer: &mut Tokenizer) -> bool {
        if Number::is_valid(c) {
            tokenizer.add_token();
            tokenizer.add_to_buffer(c);
            tokenizer.set_state(State::Number);
        } else if Symbol::is_valid(c) {
            tokenizer.add_token();
            tokenizer.add_to_buffer(c);
            tokenizer.set_state(State::Symbol);
        } else if Text::is_delimiter(c) {
            tokenizer.add_token();
            tokenizer.set_state(State::Text);
        } else if Whitespace::is_valid(c) {
            tokenizer.add_token();
            tokenizer.set_state(State::Whitespace);
        } else {
            return false;
        }
        return true;
    }
}
