pub mod ast;
pub mod lexer;
pub mod rules;

use rules::expressions::*;

fn main() {
  let mut result = Vec::new();
  let tokens = lexer::lex("Vector<int>", &mut result);
  println!("{:#?}", parse_term(tokens));
}
