#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ObjectKind {
  Char,
  Byte,
  SByte,
  Boolean,
  Short,
  UShort,
  Int32,
  UInt32,
  Int64,
  UInt64,
  Float,
  Double,
  Decimal,
  String,
  Object,
}
