use search_engine::lexer::Lexer;
use std::env::args;

fn main() {
    let mut query = String::new();
    for word in args() {
        if query != "".to_string() {
            query.push(' ');
        }
        query.push_str(&word);
    }
    let tokens = Lexer::new(&query.chars().collect::<Vec<char>>()).collect::<Vec<_>>();
    println!("stemmed: {:?}", tokens);
}
