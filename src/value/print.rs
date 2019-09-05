use std::fmt;

use super::Value;
use sparkly::{Colour, Doc, Sparkly, Style};

static INDENT: usize = 4;

impl<'value> Value<'value> {
    fn seq_to_doc<'iter, I, S>(open: &'static str, xs: I, close: &'static str) -> Doc
    where
        I: Iterator<Item = S>,
        S: Sparkly,
        'value: 'iter,
    {
        Doc::text(open, Style::default())
            .append(Doc::split_point())
            .nest(INDENT)
            .append(
                Doc::text(",", Style::default())
                    .append(Doc::space())
                    .join(xs)
                    .nest(INDENT),
            )
            .append(Doc::split_point())
            .append(Doc::text(close, Style::default()))
            .group()
    }

    fn dictionary_to_doc<'tmp>(pairs: &'tmp Vec<(Value<'value>, Value<'value>)>) -> Doc
    where
        'value: 'tmp,
    {
        Self::seq_to_doc(
            "{",
            pairs.iter().map(|(key, value)| {
                key.to_doc()
                    .append(Doc::text(": ", Style::default()))
                    .append(value.to_doc())
            }),
            "}",
        )
        .group()
    }

    fn constructor_to_doc<'tmp>(
        name: &'value str,
        kwargs: &'tmp Vec<(&'value str, Value<'value>)>,
    ) -> Doc
    where
        'value: 'tmp,
    {
        Doc::text(name, Style::from(Colour::Blue))
            .append(Self::seq_to_doc(
                "(",
                kwargs.iter().map(|(key, value)| {
                    Doc::text(*key, Style::from(Colour::Blue))
                        .append(Doc::text("=", Style::default()))
                        .append(value.to_doc())
                }),
                ")",
            ))
            .group()
    }
}

impl<'value> Sparkly for Value<'value> {
    fn to_doc(&self) -> Doc {
        match *self {
            Value::Int(x) => Doc::text(x, Style::from(Colour::Purple)),
            Value::Float(x) => Doc::text(x, Style::from(Colour::Purple)),
            Value::Bool(x) => {
                Doc::text(if x { "True" } else { "False" }, Style::from(Colour::Green))
            }
            Value::Symbol(x) => Doc::text(x, Style::from(Colour::Blue)),
            Value::Str(x) => Doc::text(x, Style::from(Colour::Cyan)),
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
        self.to_doc().display().fmt(f)
    }
}
