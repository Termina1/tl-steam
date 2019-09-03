pub mod ast;
pub mod lexer;
pub mod rules;

use rules::*;

fn main() {
  let mut result = Vec::new();
  let tokens = lexer::lex("antispam.searchPattern#7f639b9d", &mut result);
  println!("{:?}", combinator_name(tokens));
}
