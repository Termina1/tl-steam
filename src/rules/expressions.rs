use super::*;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::{many0, separated_nonempty_list, many1};
use nom::sequence::tuple;

pub fn parse_term(input: Tokens) -> TLParser<TLExpression> {
  let (i, term) = alt((
    parse_term_full_expression,
    parse_term_operator,
    map_res(tag(TLTokenEnum::NUM), |_| -> ParserM<TLExpression> {
      Ok(TLExpression::Hash)
    }),
    map_res(nat_const, |num| -> ParserM<TLExpression> {
      Ok(TLExpression::Nat(num))
    }),
    parse_term_brackets,
    map_res(type_ident, |ident| -> ParserM<TLExpression> {
      Ok(TLExpression::Ident(ident))
    }),
  ))(input)?;
  return Ok((i, term));
}

fn parse_term_brackets(input: Tokens) -> TLParser<TLExpression> {
  let(i, (ident, _, exprs, _)) = tuple((
    type_ident,
    tag(TLTokenEnum::LESSTHAN),
    separated_nonempty_list(
      tag(TLTokenEnum::COMA),
      map_res(many1(parse_expression), |res| -> ParserM<TLExpression> { Ok(TLExpression::Expression(res)) })
    ),
    tag(TLTokenEnum::GREATERTHAN),
  ))(input)?;

  return Ok((i, TLExpression::Expression(vec![TLExpression::Ident(ident), TLExpression::Expression(exprs)])));
}

fn parse_term_full_expression(input: Tokens) -> TLParser<TLExpression> {
  let (i, term) = tuple((
    tag(TLTokenEnum::OPBR),
    parse_full_expression,
    tag(TLTokenEnum::CLBR),
  ))(input)?;
  return Ok((i, term.1));
}

fn parse_term_operator(input: Tokens) -> TLParser<TLExpression> {
  let (i, term) = tuple((tag(TLTokenEnum::PERCENT), parse_term))(input)?;
  return Ok((
    i,
    TLExpression::Operator(TLOperator::Bang, Box::from(term.1)),
  ));
}

fn parse_expression_hp(input: Tokens) -> TLParser<TLExpression> {
  let (i, (_, nat, expr)) = tuple((
    tag(TLTokenEnum::PLUS),
    nat_const,
    empty(parse_expression),
  ))(input)?;

  let expr = TLExpression::Expression(vec![TLExpression::Nat(nat), expr]);
  return Ok((i, TLExpression::Operator(TLOperator::Plus, Box::from(expr))));
}

pub fn parse_full_expression(input: Tokens) -> TLParser<TLExpression> {
  let (i, terms) = many0(parse_expression)(input)?;
  return Ok((i, TLExpression::Expression(terms)));
}

pub fn empty<I:Clone, E: ParseError<I>, F>(f: F) -> impl Fn(I) -> IResult<I, TLExpression, E>
where
  F: Fn(I) -> IResult<I, TLExpression, E>,
{
  map_res(opt(f), |res| -> ParserM<TLExpression> {
    match res {
      Some(r) => Ok(r),
      None => Ok(TLExpression::Empty)
    }
  })
}

pub fn parse_expression(input: Tokens) -> TLParser<TLExpression> {
  let (i, expr) = alt((
    map_res(
      tuple((
        parse_term,
        empty(parse_expression_hp)
      )),
      |(term, expr)| -> ParserM<TLExpression> { Ok(TLExpression::Expression(vec![term, expr])) },
    ),
    map_res(
      tuple((
        nat_const,
        tag(TLTokenEnum::PLUS),
        parse_expression,
        empty(parse_expression_hp),
      )),
      |(nat, _, expr, sexpr)| -> ParserM<TLExpression> {
        let expr = TLExpression::Expression(vec![TLExpression::Nat(nat), expr, sexpr]);
        return Ok(TLExpression::Operator(TLOperator::Plus, Box::from(expr)));
      },
    ),
  ))(input)?;

  return Ok((i, expr));
}

fn parse_result_type_helper(input: Tokens) -> TLParser<Vec<TLExpression>> {
  let (i, (_, exprs, _)) = tuple((
    tag(TLTokenEnum::LESSTHAN),
    separated_nonempty_list(
      tag(TLTokenEnum::COMA),
      parse_full_expression,
    ),
    tag(TLTokenEnum::GREATERTHAN),
  ))(input)?;

  return Ok((i, exprs));
}

pub fn parse_result_type(input: Tokens) -> TLParser<TLExpression> {
  let (i, (ident, exprs)) = pair(uc_ident,
    alt((parse_result_type_helper, many0(parse_expression)))
  )(input)?;
  let name = TLExpression::Ident(TLTypeIdent::Upper(ident));
  let mut exprs_with_ident = vec![name];
  exprs_with_ident.extend(exprs);
  return Ok((i, TLExpression::Expression(exprs_with_ident)));
}

pub fn parse_type_term_with_bang(input: Tokens) -> TLParser<TLExpression> {
  let (i, (bang, term)) = pair(
    opt(tag(TLTokenEnum::EXCLMARK)),
    parse_term
  )(input)?;
  let expr = match bang {
    Some(_) => TLExpression::Operator(TLOperator::Bang, Box::from(term)),
    None => term
  };
  return Ok((i, expr));
}