use std::fmt;

use super::Value;
use pretty::{BoxDoc, Doc};

static INDENT: usize = 4;

impl<'value> Value<'value> {
    fn seq_to_doc<'iter, I>(open: &'static str, xs: I, close: &'static str) -> Doc<'value, BoxDoc<'value, ()>>
    where
        I: Iterator<Item = Doc<'value, BoxDoc<'value, ()>>>,
        'value: 'iter,
    {
        Doc::text(open)
            .append(Doc::newline().flat_alt(Doc::nil()))
            .nest(INDENT)
            .append(
                Doc::intersperse(xs, Doc::text(",").append(Doc::space()))
                    .nest(INDENT)
            )
            .append(Doc::newline().flat_alt(Doc::nil()))
            .append(Doc::text(close))
            .group()
    }

    fn dictionary_to_doc<'tmp>(pairs: &'tmp Vec<(Value<'value>, Value<'value>)>) -> Doc<'value, BoxDoc<'value, ()>>
    where
        'value: 'tmp,
    {
        Self::seq_to_doc(
            "{",
            pairs.iter().map(|(key, value)| {
                key.to_doc()
                    .append(Doc::text(": "))
                    .append(value.to_doc())
            }),
            "}",
        )
        .group()
    }

    fn constructor_to_doc<'tmp>(
        name: &'value str,
        kwargs: &'tmp Vec<(&'value str, Value<'value>)>,
    ) -> Doc<'value, BoxDoc<'value, ()>>
    where
        'value: 'tmp,
    {
        Doc::text(name)
            .append(Self::seq_to_doc(
                "(",
                kwargs.iter().map(|(key, value)| {
                    Doc::text(*key)
                        .append(Doc::text("="))
                        .append(value.to_doc())
                }),
                ")",
            ))
            .group()
    }
}

impl<'value> Value<'value> {
    pub fn to_doc(&self) -> Doc<'value, BoxDoc<'value, ()>> {
        match *self {
            Value::Int(x) => Doc::text(x.to_string()),
            Value::Float(x) => Doc::text(x.to_string()),
            Value::Bool(x) => Doc::text(if x { "True" } else { "False" }),
            Value::Symbol(x) => Doc::text(x),
            Value::Str(x) => Doc::text(x),
            Value::List(ref xs) => Self::seq_to_doc("[", xs.iter().map(Self::to_doc), "]"),
            Value::Tuple(ref xs) => Self::seq_to_doc("(", xs.iter().map(Self::to_doc), ")"),
            Value::Set(ref xs) => Self::seq_to_doc("{", xs.iter().map(Self::to_doc), "}"),
            Value::Dict(ref pairs) => Self::dictionary_to_doc(pairs),
            Value::Constructor(name, ref kwargs) => Self::constructor_to_doc(name, kwargs),
        }
    }
}

impl<'a> fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.to_doc().render_fmt(80, f)
    }
}
