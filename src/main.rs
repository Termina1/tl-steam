pub mod ast;
pub mod lexer;
pub mod parser;
pub mod rules;

use lexer::lex;
use nom::multi::many0;
use parser::parse_tl;
use rules::args::*;
use rules::expressions::parse_expression;
use std::fs;

fn main() {
  let contents = fs::read_to_string("/Users/terminal/Work/rust/tl-steam/example.tl")
    .expect("Something went wrong reading the file");
  match parse_tl(contents.as_str()) {
    Ok(tl) => println!("{:#?}", tl),
    Err(err) => print!("{}", err),
  };
  // let mut result = Vec::new();
  // let tokens = lex("1 + 2 + 8", &mut result);
  // println!("{:#?}", parse_expression(tokens));
}
