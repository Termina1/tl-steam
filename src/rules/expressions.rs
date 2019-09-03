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
