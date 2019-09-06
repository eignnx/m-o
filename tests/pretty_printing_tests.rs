use m_o::value::print::PrintOptions;
use m_o::value::Value;

fn value_to_string(value: &Value, columns: usize) -> String {
    let options = PrintOptions {
        columns,
        ..Default::default()
    };
    value.to_doc(&options).pretty(options.columns).to_string()
}

#[test]
fn test_simple_value_to_string() {
    let list = Value::List(vec![
        Value::Int(123),
        Value::Str("\"abc\""),
        Value::Bool(true),
        Value::Constructor(
            "Dog",
            vec![("name", Value::Str("'Pip'")), ("age", Value::Int(7))],
        ),
    ]);

    // Skinny tty
    assert_eq!(
        value_to_string(&list, 20),
        r#"[
    123,
    "abc",
    True,
    Dog(
        name='Pip',
        age=7
    )
]"#
    );

    // Wide tty
    assert_eq!(
        value_to_string(&list, 1000),
        r#"[123, "abc", True, Dog(name='Pip', age=7)]"#
    );
}

#[test]
fn test_deep_nesting_to_string() {
    let value = Value::List(vec![Value::List(vec![
        Value::Int(123),
        Value::Int(234),
        Value::Int(345),
        Value::Set(vec![
            Value::Int(456),
            Value::Int(567),
            Value::Int(678),
            Value::Constructor(
                "Dog",
                vec![("name", Value::Str("\"Pip\"")), ("age", Value::Int(7))],
            ),
        ]),
    ])]);

    let expected = r#"[
    [
        123,
        234,
        345,
        {
            456,
            567,
            678,
            Dog(
                name="Pip",
                age=7
            )
        }
    ]
]"#;

    assert_eq!(value_to_string(&value, 16), expected);
}

#[test]
fn test_nested_dicts_to_string() {
    let dict = Value::Dict(vec![
        (
            Value::Str("\"abc\""),
            Value::Dict(vec![(
                Value::Int(123),
                Value::Dict(vec![
                    (Value::Bool(true), Value::Float(3.14)),
                    (Value::Bool(true), Value::Bool(false)),
                    (Value::Bool(true), Value::Bool(false)),
                    (Value::Bool(true), Value::Bool(false)),
                ]),
            )]),
        ),
        (Value::Bool(true), Value::Bool(false)),
        (Value::Bool(true), Value::Symbol("qwertyuiopasdfghjkl")),
        (
            Value::Bool(true),
            Value::Tuple(vec![Value::Int(1), Value::Int(2), Value::Int(3)]),
        ),
    ]);

    let expected = r#"{
    "abc": {
        123: {
            True: 3.14,
            True: False,
            True: False,
            True: False
        }
    },
    True: False,
    True: qwertyuiopasdfghjkl,
    True: (1, 2, 3)
}"#;

    assert_eq!(value_to_string(&dict, 20), expected);
}
