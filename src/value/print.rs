use std::fmt;
use std::fmt::Write;

use super::Value;

fn write_comma_seq(
    f: &mut fmt::Formatter,
    xs: &[Value],
    open: char,
    close: char,
    level: usize,
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
    level: usize,
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
