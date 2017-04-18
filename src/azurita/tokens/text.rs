use azurita::tokens::{Tokenizer, State};

static DELIMITER: char = '"';

pub struct Text {}

impl Text {
    pub fn is_valid(c: char) -> bool {
        return c != '\n';
    }
    pub fn is_delimiter(c: char) -> bool {
        return c == DELIMITER;
    }
    pub fn handle(c: char, tokenizer: &mut Tokenizer) -> bool {
        if Text::is_delimiter(c) {
            tokenizer.add_token();
            tokenizer.set_state(State::Whitespace);
        } else if Text::is_valid(c) {
            tokenizer.add_to_buffer(c);
        } else {
            tokenizer.set_error("Found line break while parsing string");
            return false;
        }
        return true;
    }
}
