pub mod ast;
pub mod lexer;
pub mod rules;

use rules::expressions::*;

fn main() {
  let mut result = Vec::new();
  let tokens = lexer::lex("!X", &mut result);
  println!("{:#?}", parse_type_term_with_bang(tokens));
}
