use super::*;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::{many0, many1};
use nom::sequence::tuple;
use declarations::parse_declaration;

fn parse_block(input: Tokens) -> TLParser<TLDeclarationBlock> {
  let (i, (_, name, _, decls)) = tuple((
    tag(TLTokenEnum::SEPARATOR),
    alt((tag(TLTokenEnum::FUNCTIONS), tag(TLTokenEnum::TYPES))),
    tag(TLTokenEnum::SEPARATOR),
    many0(parse_declaration)
  ))(input)?;

  let block = match name.tok[0].token_type {
    TLTokenEnum::FUNCTIONS => TLDeclarationBlock::Functions(decls),
    _ => TLDeclarationBlock::Types(decls)
  };

  return Ok((i, block));
}

pub fn parse_program(input: Tokens) -> TLParser<TLProgram> {
  let (i, (type_declarations, mut blocks)) = tuple((
    many1(parse_declaration),
    many0(parse_block)
  ))(input)?;
  let mut all_blocks = vec![TLDeclarationBlock::Types(type_declarations)];
  all_blocks.append(&mut blocks);
  return Ok((i, TLProgram{ blocks: all_blocks }));
}