use logos::Logos;
use nom::*;
use std::iter::Enumerate;
use std::ops::{Range, RangeFrom, RangeFull, RangeTo};

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
pub enum TLTokenEnum {
  #[token = "_"]
  UNDERLINE,

  #[token = ":"]
  COLON,

  #[token = ";"]
  SEMICOLON,

  #[token = "("]
  OPBR,

  #[token = ")"]
  CLBR,

  #[token = "["]
  OPSBR,

  #[token = "]"]
  CLSBR,

  #[token = "{"]
  OPCBR,

  #[token = "}"]
  CLSCBR,

  #[token = "---"]
  SEPARATOR,

  #[token = "="]
  EQ,

  #[token = "#"]
  NUM,

  #[token = "?"]
  QMARK,

  #[token = "%"]
  PERCENT,

  #[token = "+"]
  PLUS,

  #[token = "<"]
  LESSTHAN,

  #[token = ">"]
  GREATERTHAN,

  #[token = ","]
  COMA,

  #[token = "."]
  STOP,

  #[token = "*"]
  MULT,

  #[token = "!"]
  EXCLMARK,

  #[token = "Final"]
  FINAL,

  #[token = "New"]
  NEW,

  #[token = "Empty"]
  EMPTY,

  #[end]
  END,

  #[regex = "//.*"]
  COMMENT,

  #[regex = "[0-9]+"]
  NUMBER,

  #[regex = "[0-9a-fA-F]+"]
  HEXNUMBER,

  #[regex = "[a-z][a-zA-Z0-9_]+"]
  LCIDENT,

  #[regex = "[A-Z][a-zA-Z0-9_]+"]
  UCIDENT,

  #[error]
  ERROR,
}

#[derive(PartialEq, Debug)]
pub struct TLToken {
  token_type: TLTokenEnum,
  pub token: String,
}

impl PartialEq<TLToken> for TLTokenEnum {
  fn eq(&self, other: &TLToken) -> bool {
    return other.token_type.eq(self);
  }
}

impl PartialEq<TLTokenEnum> for TLToken {
  fn eq(&self, other: &TLTokenEnum) -> bool {
    return other.eq(&self.token_type);
  }
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(C)]
pub struct Tokens<'a> {
  pub tok: &'a [TLToken],
  pub start: usize,
  pub end: usize,
}

impl<'a> Tokens<'a> {
  pub fn new(vec: &'a Vec<TLToken>) -> Self {
    Tokens {
      tok: vec.as_slice(),
      start: 0,
      end: vec.len(),
    }
  }
}

impl<'a> InputLength for Tokens<'a> {
  #[inline]
  fn input_len(&self) -> usize {
    self.tok.len()
  }
}

impl<'a> InputTake for Tokens<'a> {
  #[inline]
  fn take(&self, count: usize) -> Self {
    Tokens {
      tok: &self.tok[0..count],
      start: 0,
      end: count,
    }
  }

  #[inline]
  fn take_split(&self, count: usize) -> (Self, Self) {
    let (prefix, suffix) = self.tok.split_at(count);
    let first = Tokens {
      tok: prefix,
      start: 0,
      end: prefix.len(),
    };
    let second = Tokens {
      tok: suffix,
      start: 0,
      end: suffix.len(),
    };
    (second, first)
  }
}

impl InputLength for TLToken {
  #[inline]
  fn input_len(&self) -> usize {
    1
  }
}

impl InputLength for TLTokenEnum {
  #[inline]
  fn input_len(&self) -> usize {
    1
  }
}

impl<'a> Slice<Range<usize>> for Tokens<'a> {
  #[inline]
  fn slice(&self, range: Range<usize>) -> Self {
    Tokens {
      tok: self.tok.slice(range.clone()),
      start: self.start + range.start,
      end: self.start + range.end,
    }
  }
}

impl<'a> Slice<RangeTo<usize>> for Tokens<'a> {
  #[inline]
  fn slice(&self, range: RangeTo<usize>) -> Self {
    self.slice(0..range.end)
  }
}

impl<'a> Slice<RangeFrom<usize>> for Tokens<'a> {
  #[inline]
  fn slice(&self, range: RangeFrom<usize>) -> Self {
    self.slice(range.start..self.end - self.start)
  }
}

impl<'a> Slice<RangeFull> for Tokens<'a> {
  #[inline]
  fn slice(&self, _: RangeFull) -> Self {
    Tokens {
      tok: self.tok,
      start: self.start,
      end: self.end,
    }
  }
}

impl<'a> InputIter for Tokens<'a> {
  type Item = &'a TLToken;
  type Iter = Enumerate<::std::slice::Iter<'a, TLToken>>;
  type IterElem = ::std::slice::Iter<'a, TLToken>;

  #[inline]
  fn iter_indices(&self) -> Enumerate<::std::slice::Iter<'a, TLToken>> {
    self.tok.iter().enumerate()
  }
  #[inline]
  fn iter_elements(&self) -> ::std::slice::Iter<'a, TLToken> {
    self.tok.iter()
  }
  #[inline]
  fn position<P>(&self, predicate: P) -> Option<usize>
  where
    P: Fn(Self::Item) -> bool,
  {
    self.tok.iter().position(|b| predicate(&b))
  }
  #[inline]
  fn slice_index(&self, count: usize) -> Option<usize> {
    if self.tok.len() >= count {
      Some(count)
    } else {
      None
    }
  }
}

impl<'a> Compare<TLTokenEnum> for Tokens<'a> {
  fn compare(&self, t: TLTokenEnum) -> CompareResult {
    if self.start < self.tok.len() && self.tok[self.start].eq(&t) {
      return CompareResult::Ok;
    }
    return CompareResult::Error;
  }
  fn compare_no_case(&self, t: TLTokenEnum) -> CompareResult {
    self.compare(t)
  }
}

pub fn lex<'a>(scheme: &str, result: &'a mut Vec<TLToken>) -> Tokens<'a> {
  let mut lexer = TLTokenEnum::lexer(scheme);
  while lexer.token != TLTokenEnum::END {
    if lexer.token != TLTokenEnum::COMMENT {
      result.push(TLToken {
        token_type: lexer.token,
        token: lexer.slice().to_string(),
      });
    }
    lexer.advance();
  }
  return Tokens::new(result);
}
