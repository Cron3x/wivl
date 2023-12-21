use phf::phf_map;
static KEY_WORDS: phf::Map<&'static str, TokenKind> = phf_map! {
    "let"   => TokenKind::Let,
    "const" => TokenKind::Const,
    "fn"    => TokenKind::Fn
};

#[derive(Copy, Clone, Debug)]
enum TokenKind {
    EOF = 0,
    Number,
    Identifier,
    Equals,
    OParen,
    CParen,
    BinaryOperator,
    Let,
    Const,
    Fn,
    Comment,
}

struct Loc{
    path: String,
    row: usize,
    col: usize,
}
#[derive(Debug)]
pub(crate) struct Token{
    kind: TokenKind,
    value: String,
}
impl Token {
    fn new(kind: &TokenKind, value: String) -> Token{
        Token{ kind: kind.clone(), value}
    }
}

pub fn tokenize(source: String) -> Vec<Token>{
    let mut tokens: Vec<Token> = vec![];

    let mut src = source.as_bytes().to_vec();

    while src.len() > 0 {
        if src[0] as char == '(' {
            tokens.push( Token::new(&TokenKind::OParen, src.remove(0).to_string()) )
        }
        else if src[0] as char == ')' {
            tokens.push( Token::new(&TokenKind::CParen, src.remove(0).to_string()) )
        }
        else if src[0] as char == '+' || src[0] as char == '-'|| src[0] as char == '*'|| src[0] as char == '/'{
            tokens.push( Token::new(&TokenKind::BinaryOperator, src.remove(0).to_string()) )
        }
        else if src[0] as char == '=' {
            tokens.push( Token::new(&TokenKind::Equals, src.remove(0).to_string()) )
        } else {
            if (src[0] as char ).is_numeric() {
                let mut num: usize = 0;
                while src.len() > 0 && (src[0] as char).is_numeric() {
                    println!("{}", src[0] as u32);
                    num = (num * 10) + (src.remove(0) - 48 ) as usize
                }
                tokens.push( Token::new(&TokenKind::Number, num.to_string()) )
            } else if (src[0] as char ).is_alphabetic() {
                let mut ident: String = String::new();
                while src.len() > 0 && (src[0] as char).is_alphabetic() {
                    ident.push((src.remove(0)) as char)
                }

                match &KEY_WORDS.get(ident.as_str()).cloned() {
                    Some(kind) => tokens.push( Token::new(kind, ident)),
                    None       => tokens.push( Token::new(&TokenKind::Identifier, ident)),
                }
            } else if (src[0] as char).is_whitespace() {
                src.remove(0);
            } else {
                eprintln!("found unexpected character {}", src[0] as char);
                src.remove(0);
            }
        }        
    }

    tokens.push(Token::new(&TokenKind::EOF, "".to_string()));
    tokens
}
