use super::super::lexer::TLTokenEnum;
use super::*;
use expressions::{parse_term, parse_type_term_with_bang};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::{many1, many0};
use nom::sequence::tuple;

fn conditional(input: Tokens) -> TLParser<TLCondition> {
  let (i, (name, _, nat, _)) = tuple((
    var_name,
    tag(TLTokenEnum::STOP),
    nat_const,
    tag(TLTokenEnum::QMARK),
  ))(input)?;

  return Ok((i, TLCondition::Condition(name, nat)));
}

fn flatten(m: Option<Option<TLVarName>>) -> Option<TLVarName> {
  match m {
    Some(m) => m,
    None => None,
  }
}

pub fn parse_multiplicity_args(input: Tokens) -> TLParser<Vec<TLArg>> {
  let (i, (name_opt, term_opt, _, args, _)) = tuple((
    opt(map_res(
      tuple((var_name_optional, tag(TLTokenEnum::COLON))),
      |(name, _)| -> ParserM<Option<TLVarName>> { Ok(name) },
    )),
    opt(map_res(
      tuple((parse_term, tag(TLTokenEnum::MULT))),
      |(expr, _)| -> ParserM<TLExpression> { Ok(expr) },
    )),
    tag(TLTokenEnum::OPSBR),
    many0(parse_args),
    tag(TLTokenEnum::CLSBR),
  ))(input)?;
  let mut all_args: Vec<TLArg> = vec![];
  for args_args in args {
    for arg in args_args {
      all_args.push(arg);
    }
  }
  return Ok((
    i,
    vec![TLArg::MultiplicityArg(flatten(name_opt), term_opt, all_args)],
  ));
}

fn parse_simple_arg(input: Tokens) -> TLParser<Vec<TLArg>> {
  let (i, (name, _, cond, expr)) = tuple((
    var_name_optional,
    tag(TLTokenEnum::COLON),
    opt(conditional),
    parse_type_term_with_bang,
  ))(input)?;

  let arg = match cond {
    Some(conditional) => TLArg::ConditionalArg(name, conditional, expr),
    None => TLArg::Arg(name, expr),
  };

  return Ok((i, vec![arg]));
}

fn parse_list_args(input: Tokens) -> TLParser<Vec<TLArg>> {
  let (i, (_, names, _, expr, _)) = tuple((
    tag(TLTokenEnum::OPBR),
    many1(var_name_optional),
    tag(TLTokenEnum::COLON),
    parse_type_term_with_bang,
    tag(TLTokenEnum::CLBR),
  ))(input)?;
  let args = names
    .iter()
    .map(|name| TLArg::Arg(name.clone(), expr.clone()))
    .collect::<Vec<_>>();
  return Ok((i, args));
}

fn parse_short_arg(input: Tokens) -> TLParser<Vec<TLArg>> {
  let (i, term) = parse_type_term_with_bang(input)?;
  let arg = TLArg::Arg(None, term);
  return Ok((i, vec![arg]));
}

pub fn parse_args(input: Tokens) -> TLParser<Vec<TLArg>> {
  let (i, args) = alt((
    parse_multiplicity_args,
    parse_simple_arg,
    parse_list_args,
    parse_short_arg,
  ))(input)?;
  return Ok((i, args));
}

pub fn parse_optional_args(input: Tokens) -> TLParser<Vec<TLArg>> {
  let (i, (_, names, _, expr, _)) = tuple((
    tag(TLTokenEnum::OPCBR),
    many1(var_name),
    tag(TLTokenEnum::COLON),
    parse_type_term_with_bang,
    tag(TLTokenEnum::CLSCBR),
  ))(input)?;

  let args = names
    .iter()
    .map(|name| TLArg::OptArg(name.clone(), expr.clone()))
    .collect::<Vec<_>>();
  return Ok((i, args));
}
