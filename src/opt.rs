use m_o::value::print::PrintOptions;
use std::num::ParseIntError;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about, rename_all = "kebab-case")]
pub struct Opt {
    /// The number of spaces used for a single indentation in the output.
    #[structopt(short, long, default_value = "4")]
    pub indent: usize,

    /// Specifies the width of the terminal or file that the results will be printed to. If
    /// unspecified, `m-o` will try to use the width of the current terminal window.
    #[structopt(short, long, parse(try_from_str = parse_columns))]
    pub columns: usize,
}

fn terminal_width() -> usize {
    termion::terminal_size()
        .map(|(w, _h)| w as usize)
        .unwrap_or(80)
}

fn parse_columns(input: &str) -> Result<usize, ParseIntError> {
    if input.is_empty() {
        Ok(terminal_width())
    } else {
        input.parse::<usize>()
    }
}

impl Into<PrintOptions> for &Opt {
    fn into(self) -> PrintOptions {
        PrintOptions {
            indent: self.indent,
            columns: self.columns,
        }
    }
}
