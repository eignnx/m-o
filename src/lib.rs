use std::convert::TryFrom;
use std::fmt;
use std::fmt::Write;

use nom::{AsChar, IResult};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{char, digit1, multispace0};
use nom::combinator::{map, map_res, opt};
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
    Symbol(String),
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
            (Value::Symbol(x), Value::Symbol(y)) => x == y,
            _ => false,
        }
    }
}

impl Eq for Value {}

fn write_comma_seq(
    f: &mut fmt::Formatter,
    xs: &[Value],
    open: char,
    close: char,
    level: usize
) -> Result<(), fmt::Error> {
    f.write_char(open)?;
    if xs.len() > 0 {
        f.write_char('\n')?;
        for x in xs {
            indent(f, level + 1)?;
            display_value(f, x, level + 1)?;
            f.write_str(",\n")?;
        }
    }
    indent(f, level)?;
    f.write_char(close)?;
    Ok(())
}

fn write_kwarg(
    f: &mut fmt::Formatter,
    (key, value): &(String, Value),
    level: usize
) -> Result<(), fmt::Error> {
    f.write_str(key)?;
    f.write_char('=')?;
    display_value(f, value, level)?;
    Ok(())
}

fn write_constructor(
    f: &mut fmt::Formatter,
    name: &str,
    kwargs: &[(String, Value)],
    level: usize,
) -> Result<(), fmt::Error> {
    f.write_str(name)?;
    f.write_char('(')?;
    f.write_char('\n')?;
    for kwarg in kwargs {
        indent(f, level + 1)?;
        write_kwarg(f, kwarg, level + 1)?;
        f.write_str(",\n")?;
    }
    indent(f, level)?;
    f.write_char(')')?;
    Ok(())
}

fn write_dict_key_value(
    f: &mut fmt::Formatter,
    (key, value): &(Value, Value),
    level: usize,
) -> Result<(), fmt::Error> {
    display_value(f, key, level)?;
    f.write_str(": ")?;
    display_value(f, value, level)?;
    Ok(())
}

fn write_dict(
    f: &mut fmt::Formatter,
    pairs: &[(Value, Value)],
    level: usize,
) -> Result<(), fmt::Error> {
    f.write_char('{')?;
    f.write_char('\n')?;
    for pair in pairs {
        indent(f, level + 1)?;
        write_dict_key_value(f, pair, level + 1)?;
        f.write_str(",\n")?;
    }
    indent(f, level)?;
    f.write_char('}')?;
    Ok(())
}

fn indent(f: &mut fmt::Formatter, level: usize) -> Result<(), fmt::Error> {
    for _ in 0..level {
        f.write_str("    ")?;
    }
    Ok(())
}

fn display_value(f: &mut fmt::Formatter, value: &Value, level: usize) -> Result<(), fmt::Error> {
    match value {
        Value::Bool(b) => f.write_str(if *b { "True" } else { "False" }),
        Value::Str(s) => write!(f, "\"{}\"", *s),
        Value::Int(i) => write!(f, "{}", *i),
        Value::Float(float) => write!(f, "{}", *float),
        Value::Tuple(xs) => write_comma_seq(f, &xs, '(', ')', level),
        Value::List(xs) => write_comma_seq(f, &xs, '[', ']', level),
        Value::Set(xs) => write_comma_seq(f, &xs, '{', '}', level),
        Value::Dict(pairs) => write_dict(f, pairs, level),
        Value::Constructor(name, kwargs) => write_constructor(f, name, kwargs, level),
        Value::Symbol(s) => f.write_str(s),
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        display_value(f, self, 0)
    }
}

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
        tuple((
            map(
                opt(tag("-")),
                |sign| if sign.is_some() { -1 } else { 1 }
            ),
            map_res(digit1, |s: &str| s.parse::<i64>()),
        )),
        |(sign, i)| Value::Int(sign * i),
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
    let mut chars = input.chars().enumerate();
    let first_char = chars.next();
    first_char
        .filter(|(_, ch)| (*ch == '_' || ch.is_alpha()))
        .and_then(|_| {
            chars
                .take_while(|(_i, ch)| (*ch == '_' || ch.is_alphanumeric()))
                .map(|(i, _ch)| i)
                .last()
                .map(|idx| {
                    (&input[idx+1..], input[0..idx+1].into()) // TODO: remove .into() here
                })
                .or_else(|| Some((&input[1..], input[..1].into())))
        })
        .ok_or_else(|| nom::Err::Error((input, nom::error::ErrorKind::Char)))
}

pub fn parse_symbol(input: &str) -> IResult<&str, Value> {
    map(
        identifier,
        Value::Symbol,
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
        parse_symbol,
    ))(input)
}

impl<'a> TryFrom<&'a str> for Value {
    type Error = nom::Err<(&'a str, nom::error::ErrorKind)>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        match parse_value(input) {
            Ok((_rest, value)) => Ok(value),
            Err(err) => Err(err),
        }
    }
}
