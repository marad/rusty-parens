pub enum TokenKind {
    Identifier,
    LeftParen,
    RightParen,
    Number,
    String,
    Keyword,
    ReaderMacro
}
pub struct Token {
    pub text: String,
    pub kind: TokenKind,
}

pub struct Tokenizer {
    to_read: String,
    position: usize,
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Item> {

        let mut current_token = String::new();


        for char in self.to_read.chars() {
            // TODO
        }

        None
    }
}


#[test]
fn hello() {
    let code = "(println \"Hello World\")";
    let mut tokenizer = Tokenizer {
        to_read: code.to_owned(),
        position: 0
    };

    println!(tokenizer.next())
}


