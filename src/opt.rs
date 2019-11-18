use m_o::value::print::PrintOptions;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about, rename_all = "kebab-case")]
struct HiddenOpt {
    /// The number of spaces used for a single indentation in the output.
    #[structopt(short, long, default_value = "4")]
    indent: usize,

    /// Specifies the width of the terminal or file that the results will be printed to. If
    /// unspecified, `m-o` will try to use the width of the current terminal window. Defaults
    /// to 80 columns.
    #[structopt(short, long)]
    columns: Option<usize>,
}

fn terminal_width() -> usize {
    termion::terminal_size()
        .map(|(w, _h)| w as usize)
        .unwrap_or(80)
}

pub struct Opt {
    pub indent: usize,
    pub columns: usize,
}

impl From<HiddenOpt> for Opt {
    fn from(hidden: HiddenOpt) -> Self {
        Opt {
            indent: hidden.indent,
            columns: hidden.columns.unwrap_or_else(terminal_width),
        }
    }
}

impl Opt {
    pub fn from_args() -> Self {
        HiddenOpt::from_args().into()
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
