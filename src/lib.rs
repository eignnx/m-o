use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{alphanumeric1, char, digit1, multispace0};
use nom::combinator::{map, map_res};
use nom::IResult;
use nom::multi::separated_list;
use nom::number::complete::double;
use nom::sequence::{delimited, preceded, tuple};

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Str(String),
    Int(i64),
    Float(f64),
    Tuple(Vec<Value>),
    List(Vec<Value>),
    Set(Vec<Value>),
    Dict(Vec<(Value, Value)>),
    Constructor(String, Vec<(String, Value)>),
}

impl PartialEq for Value {
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
            _ => false,
        }
    }
}

impl Eq for Value {}

pub fn parse_bool(input: &str) -> IResult<&str, Value> {
    alt((
        map(tag("True"), |_| Value::Bool(true)),
        map(tag("False"), |_| Value::Bool(false)),
    ))(input)
}

pub fn parse_str(input: &str) -> IResult<&str, Value> {
    let single_quoted = delimited(
        char('\''),
        is_not("'"),
        char('\'')
    );
    let double_quoted = delimited(
        char('"'),
        is_not("\""),
        char('"')
    );
    map(
        alt((single_quoted, double_quoted)),
        |s: &str| Value::Str(s.into()),
    )(input)
}

pub fn parse_float(input: &str) -> IResult<&str, Value> {
    map(
        double,
        Value::Float,
    )(input)
}

pub fn parse_int(input: &str) -> IResult<&str, Value> {
    map(
        map_res(digit1, |s: &str| s.parse::<i64>()), // TODO: handle negative signs
        Value::Int,
    )(input)
}

fn parse_seq<'a>(open: char, f: impl Fn(Vec<Value>) -> Value, close: char) -> impl Fn(&'a str) -> IResult<&'a str, Value> {
    move |input: &'a str| -> IResult<&'a str, Value> {
        map(
            delimited(
                char(open),
                separated_list(
                    comma_space,
                    parse_value,
                ),
                char(close)
            ),
            &f
        )(input)
    }
}

pub fn parse_list(input: &str) -> IResult<&str, Value> {
    parse_seq('[', Value::List, ']')(input)
}

pub fn parse_tuple(input: &str) -> IResult<&str, Value> {
    parse_seq('(', Value::Tuple, ')')(input)
}

pub fn parse_set(input: &str) -> IResult<&str, Value> {
    parse_seq('{', Value::Set, '}')(input)
}

fn colon_space(input: &str) -> IResult<&str, ()> {
    map(
        preceded(char(':'), multispace0),
        |_| ()
    )(input)
}

fn parse_dict_key_value(input: &str) -> IResult<&str, (Value, Value)> {
    tuple((
        parse_value,
        preceded(
            colon_space,
            parse_value,
        ),
    ))(input)
}

fn comma_space(input: &str) -> IResult<&str, ()> {
    map(
        preceded(char(','), multispace0),
        |_| ()
    )(input)
}

pub fn parse_dict(input: &str) -> IResult<&str, Value> {
    map(
        delimited(
            char('{'),
            separated_list(
                comma_space,
                parse_dict_key_value,
            ),
            char('}'),
        ),
        |v| Value::Dict(v)
    )(input)
}

fn identifier(input: &str) -> IResult<&str, String> {
    map(
        alphanumeric1, // TODO: define better identifier parser
        String::from
    )(input)
}

fn parse_kwarg(input: &str) -> IResult<&str, (String, Value)> {
    tuple((
        identifier,
        preceded(
            char('='),
            parse_value,
        )
    ))(input)
}

pub fn parse_constructor(input: &str) -> IResult<&str, Value> {
    map(
        tuple((
            identifier,
            delimited(
                char('('),
                separated_list(
                    comma_space,
                    parse_kwarg
                ),
                char(')'),
            )
        )),
        |(name, kwargs)| Value::Constructor(name, kwargs)
    )(input)
}

pub fn parse_value(input: &str) -> IResult<&str, Value> {
    alt((
        parse_int,
        parse_float,
        parse_bool,
        parse_str,
        parse_list,
        parse_tuple,
        parse_dict,
        parse_set, // Appears after dict parser because `{}` is a dict, not a set.
        parse_constructor,
    ))(input)
}

