use m_o::Value;

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
