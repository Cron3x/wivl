enum TokenKind {
    End = 0,
    Number,
    Identifier,
    Equals,
    OParen,
    CParen,
    BinaryOperator,
    Let,
    Const,
}

struct Loc{
    path: String,
    row: usize,
    col: usize,
}

struct Token{
    kind: TokenKind,
    value: String,
}
impl Token {
    fn new(kind: TokenKind, value: String) -> Self{
        Token{ kind, value}
    }

}

fn tokenize(source: String) -> Vec<Token>{
    let mut tokens: Vec<Token> = vec![];

    let mut src = source.as_bytes().to_vec();

    while src.len() > 0 {
        if src[0] as char == '(' {
            tokens.push( Token::new(TokenKind::OParen, src.remove(0).to_string()) )
        }
        else if src[0] as char == ')' {
            tokens.push( Token::new(TokenKind::CParen, src.remove(0).to_string()) )
        }
        else if src[0] as char == '+' || src[0] as char == '-'|| src[0] as char == '*'|| src[0] as char == '/'{
            tokens.push( Token::new(TokenKind::BinaryOperator, src.remove(0).to_string()) )
        }
        else if src[0] as char == '=' {
            tokens.push( Token::new(TokenKind::Equals, src.remove(0).to_string()) )
        }

        
    }

    tokens
}
