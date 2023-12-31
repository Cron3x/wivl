mod ast;
mod lexer;
mod parser;

fn main() {
    let source_code = std::fs::read_to_string("tst-proj/entry.wvl").unwrap();
    let tokens = lexer::tokenize(source_code);
    println!("{:#?}", tokens);
}
