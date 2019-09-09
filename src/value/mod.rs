pub mod parse;
pub mod print;

#[derive(Debug, Clone, PartialEq)]
pub enum Value<'a> {
    Bool(bool),
    Str(&'a str),
    Int(i64),
    Float(f64),
    Tuple(Vec<Value<'a>>),
    List(Vec<Value<'a>>),
    Set(Vec<Value<'a>>),
    Dict(Vec<(Value<'a>, Value<'a>)>),
    Constructor(&'a str, Vec<Arg<'a>>),
    Symbol(&'a str),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Arg<'a> {
    Arg(Value<'a>),
    Kwarg(&'a str, Value<'a>),
}
