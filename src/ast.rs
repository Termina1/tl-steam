#[derive(Debug)]
pub enum TLName {
  Name(String),
  NameNs { ns: String, name: String },
}

#[derive(Debug)]
pub enum TLCName {
  Name(TLName),
  FullName(TLName, u32),
  EmptyName,
}

pub enum TLVarNameOptional {
  Name(String),
}

pub enum TLVarName {
  Name(String),
  Empty,
}
