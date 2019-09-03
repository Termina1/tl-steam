#[derive(Debug)]
pub enum TLLowerName {
  Name(String),
  NameNs { ns: String, name: String },
}

#[derive(Debug)]
pub enum TLUpperName {
  Name(String),
  NameNs { ns: String, name: String },
}

#[derive(Debug)]
pub enum TLTypeIdent {
  Lower(TLLowerName),
  Upper(TLUpperName)
}

#[derive(Debug)]
pub enum TLCName {
  Name(TLLowerName),
  FullName(TLLowerName, u32),
  EmptyName,
}


#[derive(Debug)]
pub enum TLVarNameOptional {
  Name(String),
}

#[derive(Debug)]
pub enum TLVarName {
  Name(String),
  Empty,
}

#[derive(Debug)]
pub enum TLOperator {
  Plus,
  Bang,
  Bare,
}

#[derive(Debug)]
pub enum TLExpression {
  Nat(u32),
  Hash,
  Empty,
  Operator(TLOperator, Box<TLExpression>),
  Expression(Vec<TLExpression>),
  Ident(TLTypeIdent)
}
