use crate::ast::Stmt;
use crate::lexer::{tokenize, TokenKind, Token};

struct Parser{
    tokens: Vec<Token>
}
impl Parser {
    fn new(source_code: String) -> Self{
        Self{
            tokens: tokenize(source_code)
        }
    }
    fn produce_ast(&mut self) -> Stmt{
        let mut body: Vec<Stmt> = Vec::new();

        while !self.is_eof(){
            body.push(self.parse_stmt());
        }

        Stmt::Program { body }
    }
    fn is_eof(&self) -> bool{
        self.tokens[0].kind == TokenKind::EOF
    }

    fn parse_stmt(&mut self) -> Stmt{
        return self.parse_expr()
    }

    fn parse_expr(&mut self) -> Stmt{
        
    }
}
