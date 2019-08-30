use std::convert::TryFrom;

use nom::error::ErrorKind;

use m_o::value::{
    parse::{
        parse_bool, parse_constructor, parse_dict, parse_int_or_float, parse_list, parse_set,
        parse_str, parse_symbol, parse_tuple,
    },
    Value,
};

type ParseResult<T> = Result<T, nom::Err<(&'static str, ErrorKind)>>;

#[test]
fn test_int() -> ParseResult<()> {
    let (_rest, i) = parse_int_or_float("1234")?;
    assert_eq!(i, Value::Int(1234));

    let (_rest, i) = parse_int_or_float("-1234")?;
    assert_eq!(i, Value::Int(-1234));

    Ok(())
}

#[test]
fn test_float() -> ParseResult<()> {
    let (_rest, f) = parse_int_or_float("123.456")?;
    assert_eq!(f, Value::Float(123.456));

    let (_rest, f) = parse_int_or_float("-1.234")?;
    assert_eq!(f, Value::Float(-1.234));

    Ok(())
}

#[test]
fn test_symbol() -> ParseResult<()> {
    let (_rest, sym) = parse_symbol("_")?;
    assert_eq!(sym, Value::Symbol("_".into()));

    let (_rest, sym) = parse_symbol("_123")?;
    assert_eq!(sym, Value::Symbol("_123".into()));

    let (_rest, sym) = parse_symbol("x86_64")?;
    assert_eq!(sym, Value::Symbol("x86_64".into()));

    assert!(parse_symbol("3d_movie").is_err());

    Ok(())
}

#[test]
fn test_str() -> ParseResult<()> {
    let (_rest, s) = parse_str("\"double quoted\"")?;
    assert_eq!(s, Value::Str("double quoted".into()));

    let (_rest, s) = parse_str("'single quoted'")?;
    assert_eq!(s, Value::Str("single quoted".into()));

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
            (Value::Str("a".into()), Value::Int(3)),
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
            "MyType".into(),
            vec![
                ("arg1".into(), Value::Int(12)),
                ("arg2".into(), Value::Int(34)),
            ]
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
            "A".into(),
            vec![(
                "qwerty".into(),
                Value::Set(vec![
                    Value::Int(1),
                    Value::Int(2),
                    Value::List(vec![
                        Value::Bool(true),
                        Value::List(vec![Value::Bool(false), Value::Str("abc".into())])
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
