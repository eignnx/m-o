use std::fmt;

use super::Value;
use sparkly::{Colour, Doc, Style};

static INDENT: usize = 4;
static WIDTH: usize = 50;

impl<'value> Value<'value> {
    fn seq_to_doc<'iter, I>(xs: I) -> Doc
    where
        I: Iterator<Item = &'iter Value<'value>>,
        'value: 'iter,
    {
        Doc::text("[", Style::default())
            .append(Doc::split_point())
            .nest(INDENT)
            .append(
                Doc::text(",", Style::default())
                    .append(Doc::space())
                    .join(xs.map(|value| value.to_doc()))
                    .nest(INDENT),
            )
            .append(Doc::split_point())
            .append(Doc::text("]", Style::default()))
            .group()
    }

    pub fn to_doc(&self) -> Doc {
        match *self {
            Value::Int(x) => Doc::text(x, Style::from(Colour::Green)),
            Value::Float(x) => Doc::text(x, Style::from(Colour::Purple)),
            Value::List(ref xs) => Self::seq_to_doc(xs.iter()),
            _ => unimplemented!(),
        }
    }
}

impl<'a> fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.to_doc().display_opts(WIDTH, true).fmt(f)
    }
}
