use azurita::Token;
use azurita::ast;
use azurita::tokens::Tokenizer;

pub fn eval(input: &str) -> Result<(), &'static str> {
    let tokens: Vec<Token>;
    {
        let tokenizer = Tokenizer::new();
        tokens = try!(tokenizer.parse(input));
    }
    return ast::generate(tokens);
}
