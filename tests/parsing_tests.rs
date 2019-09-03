use std::convert::TryFrom;

use nom::error::ErrorKind;

use m_o::value::{
    parse::{
        parse_bool, parse_constructor, parse_dict, parse_list, parse_set, parse_str, parse_symbol,
        parse_tuple,
    },
    Value,
};

type ParseResult<T> = Result<T, nom::Err<(&'static str, ErrorKind)>>;

#[test]
fn test_int() -> ParseResult<()> {
    let i = Value::try_from("1234")?;
    assert_eq!(i, Value::Int(1234));

    let i = Value::try_from("-1234")?;
    assert_eq!(i, Value::Int(-1234));

    Ok(())
}

#[test]
fn test_float() -> ParseResult<()> {
    let f = Value::try_from("123.456")?;
    assert_eq!(f, Value::Float(123.456));

    let f = Value::try_from("-1.234")?;
    assert_eq!(f, Value::Float(-1.234));

    Ok(())
}

#[test]
fn test_symbol() -> ParseResult<()> {
    let (_rest, sym) = parse_symbol("_")?;
    assert_eq!(sym, Value::Symbol("_"));

    let (_rest, sym) = parse_symbol("_123")?;
    assert_eq!(sym, Value::Symbol("_123"));

    let (_rest, sym) = parse_symbol("x86_64")?;
    assert_eq!(sym, Value::Symbol("x86_64"));

    assert!(parse_symbol("3d_movie").is_err());

    Ok(())
}

#[test]
fn test_str() -> ParseResult<()> {
    let txt = r#""double quoted""#;
    let (_rest, s) = parse_str(txt)?;
    assert_eq!(s, Value::Str(txt));

    let txt = r#"'single quoted'"#;
    let (_rest, s) = parse_str(txt)?;
    assert_eq!(s, Value::Str(txt));

    Ok(())
}

#[test]
fn test_str_escaping() -> ParseResult<()> {
    let txt = r#""escaped quote character (\") in a sentence.""#;
    let s = Value::try_from(txt)?;
    assert_eq!(s, Value::Str(txt));

    let txt = r#"'escaped quote character (\') in a sentence.'"#;
    let s = Value::try_from(txt)?;
    assert_eq!(s, Value::Str(txt));

    let txt = r#""what's up?""#;
    let s = Value::try_from(txt)?;
    assert_eq!(s, Value::Str(txt));

    let txt = r#"'they told me "keep it down"'"#;
    let s = Value::try_from(txt)?;
    assert_eq!(s, Value::Str(txt));

    let txt = r#"'I hadn\'t seen it coming.\n"who are you?" they said.'"#;
    let s = Value::try_from(txt)?;
    assert_eq!(s, Value::Str(txt));

    let single_quote = r#"'\\ \' \a \b \f \n \N{name} \r \t \u1234 \U12341234 \v \012 \123 \234 \345 \456 \567 \670 \701 \x12'"#;
    let s = Value::try_from(single_quote)?;
    assert_eq!(s, Value::Str(single_quote));

    let double_quote = r#""\\ \" \a \b \f \n \N{name} \r \t \u1234 \U12341234 \v \012 \123 \234 \345 \456 \567 \670 \701 \x12""#;
    let s = Value::try_from(double_quote)?;
    assert_eq!(s, Value::Str(double_quote));

    Ok(())
}

#[test]
fn test_quotes_inside_quotes() -> ParseResult<()> {
    let quote = r#"'"'"#;
    let s = Value::try_from(quote)?;
    assert_eq!(s, Value::Str(quote));

    let quote = r#""'""#;
    let s = Value::try_from(quote)?;
    assert_eq!(s, Value::Str(quote));

    Ok(())
}

#[test]
fn test_bool() -> ParseResult<()> {
    let (_rest, b) = parse_bool("True")?;
    assert_eq!(b, Value::Bool(true));

    let (_rest, b) = parse_bool("False")?;
    assert_eq!(b, Value::Bool(false));

    Ok(())
}

#[test]
fn test_list() -> ParseResult<()> {
    let (_rest, list) = parse_list("[1, 2, 3]")?;
    assert_eq!(
        list,
        Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)])
    );

    Ok(())
}

#[test]
fn test_tuple() -> ParseResult<()> {
    let (_rest, list) = parse_tuple("(1, 2, 3)")?;
    assert_eq!(
        list,
        Value::Tuple(vec![Value::Int(1), Value::Int(2), Value::Int(3)])
    );

    Ok(())
}

#[test]
fn test_set() -> ParseResult<()> {
    let (_rest, list) = parse_set("{1, 2, 3}")?;
    assert_eq!(
        list,
        Value::Set(vec![Value::Int(1), Value::Int(2), Value::Int(3)])
    );

    Ok(())
}

#[test]
fn test_empty_set() -> ParseResult<()> {
    let (_rest, set) = parse_set("{}")?;
    assert_eq!(set, Value::Set(vec![]));

    Ok(())
}

#[test]
fn test_dict() -> ParseResult<()> {
    let (_rest, dict) = parse_dict("{1: 2, 'a': 3}")?;
    assert_eq!(
        dict,
        Value::Dict(vec![
            (Value::Int(1), Value::Int(2)),
            (Value::Str("'a'"), Value::Int(3)),
        ])
    );

    Ok(())
}

#[test]
fn test_empty_dict_value() -> ParseResult<()> {
    let dict = Value::try_from("{}")?;
    assert_eq!(dict, Value::Dict(vec![]));
    Ok(())
}

#[test]
fn test_constructor() -> ParseResult<()> {
    let (_rest, cons) = parse_constructor("MyType(arg1=12, arg2=34)")?;
    assert_eq!(
        cons,
        Value::Constructor(
            "MyType",
            vec![("arg1", Value::Int(12)), ("arg2", Value::Int(34)),]
        )
    );
    Ok(())
}

#[test]
fn test_big_value() -> ParseResult<()> {
    let value = Value::try_from("A(qwerty={1, 2, [True, [False, 'abc']]})")?;
    assert_eq!(
        value,
        Value::Constructor(
            "A",
            vec![(
                "qwerty",
                Value::Set(vec![
                    Value::Int(1),
                    Value::Int(2),
                    Value::List(vec![
                        Value::Bool(true),
                        Value::List(vec![Value::Bool(false), Value::Str("'abc'")])
                    ])
                ])
            )]
        )
    );
    Ok(())
}

#[test]
fn test_float_vs_int_precedence() -> ParseResult<()> {
    let value = Value::try_from("123.456")?;
    assert_eq!(value, Value::Float(123.456));

    let value = Value::try_from("-123.456")?;
    assert_eq!(value, Value::Float(-123.456));

    let value = Value::try_from("123")?;
    assert_eq!(value, Value::Int(123));

    let value = Value::try_from("-123")?;
    assert_eq!(value, Value::Int(-123));

    let value = Value::try_from("123.")?;
    assert_eq!(value, Value::Float(123.0));

    Ok(())
}
