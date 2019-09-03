use std::convert::TryFrom;

use nom::{
    branch::alt,
    bytes::complete::{escaped, is_not, tag},
    character::complete::{char, digit1, multispace0, one_of},
    combinator::{map, not, opt, recognize},
    multi::separated_list,
    number::complete::double,
    sequence::{delimited, preceded, terminated, tuple},
    AsChar, IResult,
};

use super::Value;

pub fn parse_bool(input: &str) -> IResult<&str, Value> {
    alt((
        map(tag("True"), |_| Value::Bool(true)),
        map(tag("False"), |_| Value::Bool(false)),
    ))(input)
}

pub fn parse_str(input: &str) -> IResult<&str, Value> {
    // See: https://python-reference.readthedocs.io/en/latest/docs/str/escapes.html
    //
    // \a           ASCII bell
    // \b           ASCII backspace
    // \f           ASCII formfeed
    // \n           ASCII linefeed
    // \N{name}     character named NAME in the Unicode database
    // \r           ASCII carriage return
    // \t           ASCII horizontal tab
    // \uxxxx       character with 16-bit hex value XXXX
    // \Uxxxxxxxx   character with 32-bit hex value XXXXXXXX
    // \v           ASCII vertical tab
    // \ooo         character with octal value OOO
    // \hxx         Character with hex value XX
    let single_quoted_str_escape = r#"\'abfnNrtuUvx01234567"#;
    let double_quoted_str_escape = r#"\"abfnNrtuUvx01234567"#;

    let single_quoted = recognize(delimited(
        char('\''),
        escaped(is_not(r#"'\"#), '\\', one_of(single_quoted_str_escape)),
        char('\''),
    ));
    let double_quoted = recognize(delimited(
        char('"'),
        escaped(is_not(r#""\"#), '\\', one_of(double_quoted_str_escape)),
        char('"'),
    ));
    map(alt((single_quoted, double_quoted)), Value::Str)(input)
}

pub fn parse_int(input: &str) -> IResult<&str, Value> {
    map(
        tuple((opt(tag("-")), terminated(digit1, not(tag("."))))),
        |(sign, s): (Option<&str>, &str)| {
            let sign = if sign.is_some() { -1 } else { 1 };
            let i = s
                .parse::<i64>()
                .expect("sequence of digits can parse to int");
            Value::Int(sign * i)
        },
    )(input)
}

pub fn parse_float(input: &str) -> IResult<&str, Value> {
    map(double, Value::Float)(input)
}

fn parse_seq<'a>(
    open: char,
    f: impl Fn(Vec<Value<'a>>) -> Value<'a>,
    close: char,
) -> impl Fn(&'a str) -> IResult<&'a str, Value<'a>> {
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
        Value::Dict,
    )(input)
}

fn identifier(input: &str) -> IResult<&str, &str> {
    let mut chars = input.chars().enumerate();
    let first_char = chars.next();
    first_char
        .filter(|(_, ch)| (*ch == '_' || ch.is_alpha()))
        .and_then(|_| {
            chars
                .take_while(|(_i, ch)| (*ch == '_' || ch.is_alphanumeric()))
                .map(|(i, _ch)| i)
                .last()
                .map(|idx| (&input[idx + 1..], &input[0..=idx]))
                .or_else(|| Some((&input[1..], &input[..1])))
        })
        .ok_or_else(|| nom::Err::Error((input, nom::error::ErrorKind::Char)))
}

pub fn parse_symbol(input: &str) -> IResult<&str, Value> {
    map(identifier, Value::Symbol)(input)
}

fn parse_kwarg(input: &str) -> IResult<&str, (&str, Value)> {
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
        parse_int,
        parse_float, // Appears after int parser because f64 is superset of i64
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

impl<'a> TryFrom<&'a str> for Value<'a> {
    type Error = nom::Err<(&'a str, nom::error::ErrorKind)>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        match parse_value(input) {
            Ok((_rest, value)) => Ok(value),
            Err(err) => Err(err),
        }
    }
}
