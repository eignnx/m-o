use std::fmt;

use super::Value;
use pretty::{BoxDoc, Doc};

pub struct PrintOptions {
    pub indent: usize,
    pub columns: usize,
}

impl Default for PrintOptions {
    fn default() -> Self {
        PrintOptions {
            indent: 4,
            columns: 80,
        }
    }
}

impl<'value> Value<'value> {
    fn seq_to_doc<'iter, I>(
        open: &'static str,
        xs: I,
        close: &'static str,
        options: &PrintOptions,
    ) -> Doc<'value, BoxDoc<'value, ()>>
    where
        I: Iterator<Item = Doc<'value, BoxDoc<'value, ()>>>,
        'value: 'iter,
    {
        Doc::text(open)
            .append(Doc::newline().flat_alt(Doc::nil()))
            .nest(options.indent)
            .append(Doc::intersperse(xs, Doc::text(",").append(Doc::space())).nest(options.indent))
            .append(Doc::newline().flat_alt(Doc::nil()))
            .append(Doc::text(close))
            .group()
    }

    fn dictionary_to_doc<'tmp>(
        pairs: &'tmp Vec<(Value<'value>, Value<'value>)>,
        options: &PrintOptions,
    ) -> Doc<'value, BoxDoc<'value, ()>>
    where
        'value: 'tmp,
    {
        Self::seq_to_doc(
            "{",
            pairs.iter().map(|(key, value)| {
                key.to_doc(options)
                    .append(Doc::text(": "))
                    .append(value.to_doc(options))
            }),
            "}",
            options,
        )
        .group()
    }

    fn constructor_to_doc<'tmp>(
        name: &'value str,
        kwargs: &'tmp Vec<(&'value str, Value<'value>)>,
        options: &PrintOptions,
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
                        .append(value.to_doc(options))
                }),
                ")",
                options,
            ))
            .group()
    }
}

impl<'value> Value<'value> {
    pub fn to_doc(&self, options: &PrintOptions) -> Doc<'value, BoxDoc<'value, ()>> {
        match *self {
            Value::Int(x) => Doc::text(x.to_string()),
            Value::Float(x) => Doc::text(x.to_string()),
            Value::Bool(x) => Doc::text(if x { "True" } else { "False" }),
            Value::Symbol(x) => Doc::text(x),
            Value::Str(x) => Doc::text(x),
            Value::List(ref xs) => {
                Self::seq_to_doc("[", xs.iter().map(|x| x.to_doc(options)), "]", options)
            }
            Value::Tuple(ref xs) => {
                Self::seq_to_doc("(", xs.iter().map(|x| x.to_doc(options)), ")", options)
            }
            Value::Set(ref xs) => {
                Self::seq_to_doc("{", xs.iter().map(|x| x.to_doc(options)), "}", options)
            }
            Value::Dict(ref pairs) => Self::dictionary_to_doc(pairs, options),
            Value::Constructor(name, ref kwargs) => Self::constructor_to_doc(name, kwargs, options),
        }
    }
}

impl<'a> fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let options = PrintOptions::default();
        self.to_doc(&options).render_fmt(options.columns, f)
    }
}
