use std::convert::TryFrom;

use m_o::{
    parse_bool,
    parse_constructor,
    parse_dict,
    parse_float,
    parse_int,
    parse_list,
    parse_set,
    parse_str,
    parse_symbol,
    parse_tuple,
    Value,
};

#[test]
fn test_int() {
    let (_rest, i) = parse_int("1234").unwrap();
    assert_eq!(i, Value::Int(1234));

    let (_rest, i) = parse_int("-1234").unwrap();
    assert_eq!(i, Value::Int(-1234));
}

#[test]
fn test_float() {
    let (_rest, f) = parse_float("123.456").unwrap();
    assert_eq!(f, Value::Float(123.456));

    let (_rest, f) = parse_float("-1.234").unwrap();
    assert_eq!(f, Value::Float(-1.234));
}

#[test]
fn test_symbol() {
    let (_rest, sym) = parse_symbol("_").unwrap();
    assert_eq!(sym, Value::Symbol("_".into()));

    let (_rest, sym) = parse_symbol("_123").unwrap();
    assert_eq!(sym, Value::Symbol("_123".into()));

    let (_rest, sym) = parse_symbol("x86_64").unwrap();
    assert_eq!(sym, Value::Symbol("x86_64".into()));

    assert!(parse_symbol("3d_movie").is_err());
}

#[test]
fn test_str() {
    let (_rest, s) = parse_str("\"double quoted\"").unwrap();
    assert_eq!(s, Value::Str("double quoted".into()));
    let (_rest, s) = parse_str("'single quoted'").unwrap();
    assert_eq!(s, Value::Str("single quoted".into()));
}

#[test]
fn test_bool() {
    let (_rest, b) = parse_bool("True").unwrap();
    assert_eq!(b, Value::Bool(true));
    let (_rest, b) = parse_bool("False").unwrap();
    assert_eq!(b, Value::Bool(false));
}

#[test]
fn test_list() {
    let (_rest, list) = parse_list("[1, 2, 3]").unwrap();
    assert_eq!(list, Value::List(vec![
        Value::Int(1),
        Value::Int(2),
        Value::Int(3),
    ]))
}

#[test]
fn test_tuple() {
    let (_rest, list) = parse_tuple("(1, 2, 3)").unwrap();
    assert_eq!(list, Value::Tuple(vec![
        Value::Int(1),
        Value::Int(2),
        Value::Int(3),
    ]))
}

#[test]
fn test_set() {
    let (_rest, list) = parse_set("{1, 2, 3}").unwrap();
    assert_eq!(list, Value::Set(vec![
        Value::Int(1),
        Value::Int(2),
        Value::Int(3),
    ]))
}

#[test]
fn test_empty_set() {
    let (_rest, set) = parse_set("{}").unwrap();
    assert_eq!(set, Value::Set(vec![]));
}

#[test]
fn test_dict() {
    let (_rest, dict) = parse_dict("{1: 2, 'a': 3}").unwrap();
    assert_eq!(dict, Value::Dict(vec![
        (Value::Int(1), Value::Int(2)),
        (Value::Str("a".into()), Value::Int(3)),
    ]));
}

#[test]
fn test_empty_dict_value() {
    let dict = Value::try_from("{}").unwrap();
    assert_eq!(dict, Value::Dict(vec![]));
}

#[test]
fn test_constructor() {
    let (_rest, cons) = parse_constructor("MyType(arg1=12, arg2=34)").unwrap();
    assert_eq!(cons, Value::Constructor("MyType".into(), vec![
        ("arg1".into(), Value::Int(12)),
        ("arg2".into(), Value::Int(34)),
    ]))
}

#[test]
fn test_big_value() {
    let value = Value::try_from("A(qwerty={1, 2, [True, [False, 'abc']]})").unwrap();
    assert_eq!(value, Value::Constructor("A".into(), vec![
        ("qwerty".into(), Value::Set(vec![
            Value::Int(1),
            Value::Int(2),
            Value::List(vec![
                Value::Bool(true),
                Value::List(vec![
                    Value::Bool(false),
                    Value::Str("abc".into())
                ])
            ])
        ]))
    ]));
}

#[test]
fn test_simple_value_to_string() {
    let list = Value::List(vec![
        Value::Int(123),
        Value::Str("abc".into()),
        Value::Bool(true),
        Value::Constructor("Dog".into(), vec![
            ("name".into(), Value::Str("Pip".into())),
            ("age".into(), Value::Int(7)),
        ]),
    ]);
    assert_eq!(list.to_string(),
r#"[
    123,
    "abc",
    True,
    Dog(
        name="Pip",
        age=7,
    ),
]"#
    );
}

#[test]
fn test_deep_nesting_to_string() {
    let value = Value::List(vec![
        Value::List(vec![
            Value::List(vec![
                Value::Constructor("Dog".into(), vec![
                    ("name".into(), Value::Str("Pip".into())),
                    ("age".into(), Value::Int(7)),
                ])
            ])
        ])
    ]);

    let expected =
r#"[
    [
        [
            Dog(
                name="Pip",
                age=7,
            ),
        ],
    ],
]"#;

    assert_eq!(value.to_string(), expected);
}

#[test]
fn test_nested_dicts_to_string() {
    let dict = Value::Dict(vec![
        (Value::Str("abc".into()), Value::Dict(vec![
            (Value::Int(123), Value::Dict(vec![
                (Value::Bool(true), Value::Bool(false))
            ]))
        ]))
    ]);

    let expected =
r#"{
    "abc": {
        123: {
            True: False,
        },
    },
}"#;

    assert_eq!(dict.to_string(), expected);
}
