pub type Nat = u32;

#[derive(Debug, Clone)]
pub enum TLLowerName {
  Name(String),
  NameNs { ns: String, name: String },
}

#[derive(Debug, Clone)]
pub enum TLUpperName {
  Name(String),
  NameNs { ns: String, name: String },
}

#[derive(Debug, Clone)]
pub enum TLTypeIdent {
  Lower(TLLowerName),
  Upper(TLUpperName),
}

#[derive(Debug)]
pub enum TLCName {
  Name(TLLowerName),
  FullName(TLLowerName, Nat),
  EmptyName,
}

#[derive(Debug, Clone)]
pub enum TLVarName {
  Name(String),
}

#[derive(Debug, Clone)]
pub enum TLOperator {
  Plus,
  Bang,
  Bare,
}

#[derive(Debug, Clone)]
pub enum TLExpression {
  Nat(Nat),
  Hash,
  Empty,
  Operator(TLOperator, Box<TLExpression>),
  Expression(Vec<TLExpression>),
  Ident(TLTypeIdent),
}

#[derive(Debug)]
pub enum TLCondition {
  Condition(TLVarName, Nat),
}

#[derive(Debug)]
pub enum TLArg {
  Arg(Option<TLVarName>, TLExpression),
  OptArg(TLVarName, TLExpression),
  ConditionalArg(Option<TLVarName>, TLCondition, TLExpression),
  MultiplicityArg(Option<TLVarName>, Option<TLExpression>, Vec<TLArg>),
}

#[derive(Debug)]
pub struct TLCombinator {
  pub identifier: TLCName,
  pub args: Vec<TLArg>,
  pub result_type: TLExpression,
}

#[derive(Debug)]
pub enum TLFinal {
  New(TLUpperName),
  Final(TLUpperName),
  Empty(TLUpperName)
}

#[derive(Debug)]
pub enum TLDeclaration {
  Final(TLFinal),
  BuiltIn(TLCombinator),
  Combinator(TLCombinator),
}

#[derive(Debug)]
pub enum TLDeclarationBlock {
  Types(Vec<TLDeclaration>),
  Functions(Vec<TLDeclaration>)
}

#[derive(Debug)]
pub struct TLProgram {
  pub blocks: Vec<TLDeclarationBlock>
}