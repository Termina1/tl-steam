use super::ast::TLProgram;
use super::lexer::lex;
use super::rules::program::parse_program;

pub fn parse_tl(tl: &str) -> Result<TLProgram, String> {
  let mut result = Vec::new();
  let tokens = lex(tl, &mut result);
  let parse_result = parse_program(tokens);
  return match parse_result {
    Ok((tokens, program)) => {
      if tokens.tok.len() != 0 {
        Err(format!("Not all tokens parsed:\n {:#?}", tokens))
      } else {
        Ok(program)
      }
    }
    Err(err) => Err(format!("Error:\n {:#?}", err)),
  };
}
