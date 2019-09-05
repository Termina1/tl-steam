use super::super::lexer::TLTokenEnum;
use super::*;
use args::{parse_args, parse_optional_args};
use expressions::parse_result_type;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::Err;

fn flatten_args(accum_args: &mut Vec<TLArg>, args_args: Vec<Vec<TLArg>>) {
  for args in args_args {
    for arg in args {
      accum_args.push(arg);
    }
  }
}

fn parse_combinator(input: Tokens) -> TLParser<TLCombinator> {
  let (i, (name, opt_args, args, _, result_type, _)) = tuple((
    combinator_name,
    many0(parse_optional_args),
    many0(parse_args),
    tag(TLTokenEnum::EQ),
    parse_result_type,
    tag(TLTokenEnum::SEMICOLON),
  ))(input)?;

  let mut all_args: Vec<TLArg> = vec![];
  flatten_args(&mut all_args, opt_args);
  flatten_args(&mut all_args, args);

  let combinator = TLCombinator {
    identifier: name,
    args: all_args,
    result_type: result_type,
  };

  return Ok((i, combinator));
}

fn parse_builtin(input: Tokens) -> TLParser<TLCombinator> {
  let (i, (name, _, _, result_type, _)) = tuple((
    combinator_name,
    tag(TLTokenEnum::QMARK),
    tag(TLTokenEnum::EQ),
    parse_result_type,
    tag(TLTokenEnum::SEMICOLON),
  ))(input)?;

  let combinator = TLCombinator {
    identifier: name,
    args: vec![],
    result_type: result_type,
  };

  return Ok((i, combinator));
}

fn parse_final(input: Tokens) -> TLParser<TLFinal> {
  let (i, (id, type_name, _)) = tuple((uc_ident, uc_ident, tag(TLTokenEnum::SEMICOLON)))(input)?;

  let fin = match id {
    TLUpperName::Name(id) => match id.as_str() {
      "Final" => TLFinal::Final(type_name),
      "New" => TLFinal::New(type_name),
      "Empty" => TLFinal::Empty(type_name),
      _ => return Err(Err::Failure(VerboseError::from_error_kind(
        input,
        ErrorKind::Tag,
      )))
    },
    TLUpperName::NameNs { ns: _, name: _ } => {
      return Err(Err::Failure(VerboseError::from_error_kind(
        input,
        ErrorKind::Tag,
      )))
    }
  };
  return Ok((i, fin));
}

pub fn parse_declaration(input: Tokens) -> TLParser<TLDeclaration> {
  let (i, declaration) = alt((
    map_res(parse_combinator, |comb| -> ParserM<TLDeclaration> {
      Ok(TLDeclaration::Combinator(comb))
    }),
    map_res(parse_builtin, |builtin| -> ParserM<TLDeclaration> {
      Ok(TLDeclaration::BuiltIn(builtin))
    }),
    map_res(parse_final, |fin| -> ParserM<TLDeclaration> {
      Ok(TLDeclaration::Final(fin))
    }),
  ))(input)?;
  return Ok((i, declaration));
}
