use super::ast::*;
use super::lexer::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map_res, opt};
use nom::error::{ErrorKind, ParseError, VerboseError};
use nom::sequence::{pair, tuple};
use nom::IResult;

pub mod args;
pub mod declarations;
pub mod expressions;
pub mod program;

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

fn hex_number(input: Tokens) -> TLParser<Nat> {
  let (i, number) = map_res(
    tag(TLTokenEnum::HEXNUMBER),
    |number: Tokens| -> ParserM<Nat> {
      match u32::from_str_radix(&number.tok[0].token.as_str()[1..], 16) {
        Ok(num) => Ok(num),
        Err(_) => Err(VerboseError::from_error_kind(number, ErrorKind::Tag)),
      }
    },
  )(input)?;

  return Ok((i, number));
}

pub fn combinator_name(input: Tokens) -> TLParser<TLCName> {
  let (i, name) = alt((
    map_res(tag(TLTokenEnum::UNDERLINE), |_| -> ParserM<TLCName> {
      Ok(TLCName::EmptyName)
    }),
    map_res(
      pair(lc_ident, opt(hex_number)),
      |(name, magic_opt)| -> ParserM<TLCName> {
        match magic_opt {
          Some(magic) => Ok(TLCName::FullName(name, magic)),
          None => Ok(TLCName::Name(name)),
        }
      },
    ),
  ))(input)?;
  return Ok((i, name));
}

pub fn var_name(input: Tokens) -> TLParser<TLVarName> {
  let (i, name) = map_res(
    alt((
      tag(TLTokenEnum::LCIDENT),
      tag(TLTokenEnum::UCIDENT),
      tag(TLTokenEnum::TYPES),
      tag(TLTokenEnum::FUNCTIONS),
    )),
    |t: Tokens| -> ParserM<TLVarName> { Ok(TLVarName::Name(t.tok[0].token.clone())) },
  )(input)?;
  return Ok((i, name));
}

pub fn var_name_optional(input: Tokens) -> TLParser<Option<TLVarName>> {
  let (i, name) = alt((
    map_res(tag(TLTokenEnum::UNDERLINE), |_| -> ParserM<Option<TLVarName>> { Ok(None) }),
    map_res(var_name, |name| -> ParserM<Option<TLVarName>> { Ok(Some(name)) })
  ))(input)?;

  return Ok((i, name))
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

pub fn nat_const(input: Tokens) -> TLParser<Nat> {
  let (i, nat) = map_res(tag(TLTokenEnum::NUMBER), |t: Tokens| -> ParserM<Nat> {
    let num_str = t.tok[0].token.as_str();
    match u32::from_str_radix(num_str, 10) {
      Ok(num) => Ok(num),
      Err(_) => Err(VerboseError::from_error_kind(t, ErrorKind::Tag)),
    }
  })(input)?;
  return Ok((i, nat));
}
