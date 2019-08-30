use std::convert::TryFrom;

use nom::{
    AsChar,
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{char, multispace0},
    combinator::map,
    IResult,
    multi::separated_list,
    number::complete::recognize_float, sequence::{delimited, preceded, tuple},
};

use super::Value;

pub fn parse_bool(input: &str) -> IResult<&str, Value> {
    alt((
        map(tag("True"), |_| Value::Bool(true)),
        map(tag("False"), |_| Value::Bool(false)),
    ))(input)
}

pub fn parse_str(input: &str) -> IResult<&str, Value> {
    let single_quoted = delimited(char('\''), is_not("'"), char('\''));
    let double_quoted = delimited(char('"'), is_not("\""), char('"'));
    map(alt((single_quoted, double_quoted)), |s: &str| {
        Value::Str(s.into())
    })(input)
}

pub fn parse_int_or_float(input: &str) -> IResult<&str, Value> {
    let (rest, float_like) = recognize_float(input)?;
    if float_like.contains('.') {
        let f = float_like
            .parse::<f64>()
            .expect(&format!("input '{}' must parse as a float", float_like));
        Ok((rest, Value::Float(f)))
    } else {
        let i = float_like
            .parse::<i64>()
            .expect(&format!("input '{}' must parse as an int", float_like));
        Ok((rest, Value::Int(i)))
    }
}

fn parse_seq<'a>(
    open: char,
    f: impl Fn(Vec<Value>) -> Value,
    close: char,
) -> impl Fn(&'a str) -> IResult<&'a str, Value> {
    move |input: &'a str| -> IResult<&'a str, Value> {
        map(
            delimited(
                char(open),
                separated_list(comma_space, parse_value),
                char(close),
            ),
            &f,
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
    map(preceded(char(':'), multispace0), |_| ())(input)
}

fn parse_dict_key_value(input: &str) -> IResult<&str, (Value, Value)> {
    tuple((parse_value, preceded(colon_space, parse_value)))(input)
}

fn comma_space(input: &str) -> IResult<&str, ()> {
    map(preceded(char(','), multispace0), |_| ())(input)
}

pub fn parse_dict(input: &str) -> IResult<&str, Value> {
    map(
        delimited(
            char('{'),
            separated_list(comma_space, parse_dict_key_value),
            char('}'),
        ),
        |v| Value::Dict(v),
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
                    (&input[idx + 1..], input[0..idx + 1].into()) // TODO: remove .into() here
                })
                .or_else(|| Some((&input[1..], input[..1].into())))
        })
        .ok_or_else(|| nom::Err::Error((input, nom::error::ErrorKind::Char)))
}

pub fn parse_symbol(input: &str) -> IResult<&str, Value> {
    map(identifier, Value::Symbol)(input)
}

fn parse_kwarg(input: &str) -> IResult<&str, (String, Value)> {
    tuple((identifier, preceded(char('='), parse_value)))(input)
}

pub fn parse_constructor(input: &str) -> IResult<&str, Value> {
    map(
        tuple((
            identifier,
            delimited(
                char('('),
                separated_list(comma_space, parse_kwarg),
                char(')'),
            ),
        )),
        |(name, kwargs)| Value::Constructor(name, kwargs),
    )(input)
}

pub fn parse_value(input: &str) -> IResult<&str, Value> {
    alt((
        parse_int_or_float,
        //        parse_int,
        //        parse_float,
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
