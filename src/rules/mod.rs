use super::ast::*;
use super::lexer::*;
use nom::branch::*;
use nom::bytes::complete::tag;
use nom::combinator::*;
use nom::error::ErrorKind;
use nom::sequence::*;
use nom::*;

pub fn lc_ident(input: Tokens) -> IResult<Tokens, TLName> {
  let (i, (ns, name)) = tuple((
    opt(tuple((tag(TLTokenEnum::LCIDENT), tag(TLTokenEnum::STOP)))),
    tag(TLTokenEnum::LCIDENT),
  ))(input)?;
  let result = match ns {
    Some((ns, _)) => TLName::NameNs {
      ns: ns.tok[0].token.clone(),
      name: name.tok[0].token.clone(),
    },
    None => TLName::Name(name.tok[0].token.clone()),
  };
  return Ok((i, result));
}

pub fn uc_ident(input: Tokens) -> IResult<Tokens, TLName> {
  let (i, (ns, name)) = tuple((
    opt(tuple((tag(TLTokenEnum::LCIDENT), tag(TLTokenEnum::STOP)))),
    tag(TLTokenEnum::LCIDENT),
  ))(input)?;
  let result = match ns {
    Some((ns, _)) => TLName::NameNs {
      ns: ns.tok[0].token.clone(),
      name: name.tok[0].token.clone(),
    },
    None => TLName::Name(name.tok[0].token.clone()),
  };
  return Ok((i, result));
}

fn combinator_name_map<'a>(
  (name, magic_parsed): (TLName, Option<(Tokens, Tokens<'a>)>),
) -> Result<TLCName, (Tokens<'a>, ErrorKind)> {
  match magic_parsed {
    Some((_, magic_token)) => {
      let parse_result = u32::from_str_radix(magic_token.tok[0].token.as_str(), 16);
      match parse_result {
        Ok(magic) => Ok(TLCName::FullName(name, magic)),
        Err(_) => Err((magic_token, ErrorKind::Tag)),
      }
    }
    None => Ok(TLCName::Name(name)),
  }
}

fn combinator_name_map_empty<'a>(_: Tokens) -> Result<TLCName, (Tokens<'a>, ErrorKind)> {
  Ok(TLCName::EmptyName)
}

pub fn combinator_name(input: Tokens) -> IResult<Tokens, TLCName> {
  let (i, name) = alt((
    map_res(tag(TLTokenEnum::UNDERLINE), combinator_name_map_empty),
    map_res(
      pair(
        lc_ident,
        opt(pair(tag(TLTokenEnum::NUM), tag(TLTokenEnum::HEXNUMBER))),
      ),
      combinator_name_map,
    ),
  ))(input)?;
  return Ok((i, name));
}

pub fn var_name_optional(input: Tokens) -> IResult<Tokens, TLVarNameOptional> {
  let (i, name) = tag(TLTokenEnum::LCIDENT)(input)?;
  return Ok((i, TLVarNameOptional::Name(name.tok[0].token.clone())));
}

pub fn var_name(input: Tokens) -> IResult<Tokens, TLVarName> {
  let (i, name) = alt((
    map_res(
      tag(TLTokenEnum::LCIDENT),
      |t: Tokens| -> Result<TLVarName, (Tokens, ErrorKind)> {
        Ok(TLVarName::Name(t.tok[0].token.clone()))
      },
    ),
    map_res(
      tag(TLTokenEnum::UNDERLINE),
      |_: Tokens| -> Result<TLVarName, (Tokens, ErrorKind)> { Ok(TLVarName::Empty) },
    ),
  ))(input)?;
  return Ok((i, name));
}
