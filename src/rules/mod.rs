use super::ast::*;
use super::lexer::*;
use nom::branch::*;
use nom::bytes::complete::tag;
use nom::combinator::*;
use nom::error::*;
use nom::error::{ErrorKind, VerboseError};
use nom::sequence::*;
use nom::*;

pub mod expressions;

type ParserM<'a, T> = Result<T, VerboseError<Tokens<'a>>>;
type TLParser<'a, T> = IResult<Tokens<'a>, T, VerboseError<Tokens<'a>>>;

pub fn lc_ident(input: Tokens) -> TLParser<TLLowerName> {
  let (i, (ns, name)) = tuple((
    opt(tuple((tag(TLTokenEnum::LCIDENT), tag(TLTokenEnum::STOP)))),
    tag(TLTokenEnum::LCIDENT),
  ))(input)?;
  let result = match ns {
    Some((ns, _)) => TLLowerName::NameNs {
      ns: ns.tok[0].token.clone(),
      name: name.tok[0].token.clone(),
    },
    None => TLLowerName::Name(name.tok[0].token.clone()),
  };
  return Ok((i, result));
}

pub fn uc_ident(input: Tokens) -> TLParser<TLUpperName> {
  let (i, (ns, name)) = tuple((
    opt(tuple((tag(TLTokenEnum::LCIDENT), tag(TLTokenEnum::STOP)))),
    tag(TLTokenEnum::UCIDENT),
  ))(input)?;
  let result = match ns {
    Some((ns, _)) => TLUpperName::NameNs {
      ns: ns.tok[0].token.clone(),
      name: name.tok[0].token.clone(),
    },
    None => TLUpperName::Name(name.tok[0].token.clone()),
  };
  return Ok((i, result));
}

fn combinator_name_map(
  (name, magic_parsed): (TLLowerName, Option<(Tokens, u32)>),
) -> ParserM<TLCName> {
  match magic_parsed {
    Some((_, magic_token)) => Ok(TLCName::FullName(name, magic_token)),
    None => Ok(TLCName::Name(name)),
  }
}

fn combinator_name_map_empty<'a>(_: Tokens) -> ParserM<'a, TLCName> {
  Ok(TLCName::EmptyName)
}

pub fn combinator_name(input: Tokens) -> TLParser<TLCName> {
  let (i, name) = alt((
    map_res(tag(TLTokenEnum::UNDERLINE), combinator_name_map_empty),
    map_res(
      pair(lc_ident, opt(pair(tag(TLTokenEnum::NUM), nat_const))),
      combinator_name_map,
    ),
  ))(input)?;
  return Ok((i, name));
}

pub fn var_name_optional(input: Tokens) -> TLParser<TLVarNameOptional> {
  let (i, name) = tag(TLTokenEnum::LCIDENT)(input)?;
  return Ok((i, TLVarNameOptional::Name(name.tok[0].token.clone())));
}

pub fn var_name(input: Tokens) -> TLParser<TLVarName> {
  let (i, name) = alt((
    map_res(
      tag(TLTokenEnum::LCIDENT),
      |t: Tokens| -> ParserM<TLVarName> { Ok(TLVarName::Name(t.tok[0].token.clone())) },
    ),
    map_res(
      tag(TLTokenEnum::UNDERLINE),
      |_: Tokens| -> ParserM<TLVarName> { Ok(TLVarName::Empty) },
    ),
  ))(input)?;
  return Ok((i, name));
}

pub fn type_ident(input: Tokens) -> TLParser<TLTypeIdent> {
  let (i, name) = alt((
    map_res(lc_ident, |ident: TLLowerName| -> ParserM<TLTypeIdent> {
      Ok(TLTypeIdent::Lower(ident))
    }),
    map_res(uc_ident, |ident: TLUpperName| -> ParserM<TLTypeIdent> {
      Ok(TLTypeIdent::Upper(ident))
    }),
  ))(input)?;

  return Ok((i, name));
}

pub fn nat_const(input: Tokens) -> TLParser<u32> {
  let (i, nat) = map_res(tag(TLTokenEnum::NUMBER), |t: Tokens| -> ParserM<u32> {
    let num_str = t.tok[0].token.as_str();
    match u32::from_str_radix(num_str, 10) {
      Ok(num) => Ok(num),
      Err(_) => Err(VerboseError::from_error_kind(t, ErrorKind::Tag)),
    }
  })(input)?;
  return Ok((i, nat));
}
