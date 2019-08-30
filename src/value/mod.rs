pub mod parse;
pub mod print;

#[derive(Debug, Clone)]
pub enum Value<'a> {
    Bool(bool),
    Str(&'a str),
    Int(i64),
    Float(f64),
    Tuple(Vec<Value<'a>>),
    List(Vec<Value<'a>>),
    Set(Vec<Value<'a>>),
    Dict(Vec<(Value<'a>, Value<'a>)>),
    Constructor(&'a str, Vec<(&'a str, Value<'a>)>),
    Symbol(&'a str),
}

impl<'a> PartialEq for Value<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Bool(x), Value::Bool(y)) => x == y,
            (Value::Str(x), Value::Str(y)) => x == y,
            (Value::Int(x), Value::Int(y)) => x == y,
            (Value::Float(x), Value::Float(y)) => (x - y).abs() <= std::f64::EPSILON,
            (Value::Tuple(x), Value::Tuple(y)) => x == y,
            (Value::List(x), Value::List(y)) => x == y,
            (Value::Set(x), Value::Set(y)) => x == y,
            (Value::Dict(x), Value::Dict(y)) => x == y,
            (Value::Constructor(a, x), Value::Constructor(b, y)) => a == b && x == y,
            (Value::Symbol(x), Value::Symbol(y)) => x == y,
            _ => false,
        }
    }
}

impl<'a> Eq for Value<'a> {}
