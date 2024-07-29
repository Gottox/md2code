mod gen;
mod language;
mod templates;

use crate::{gen::Gen, language::LANGS};
use clap::Parser;
use std::{fs::File, io::Write};

pub type Error = Box<dyn std::error::Error>;

#[derive(Debug, Parser)]
struct Opts {
    /// Adds custom include/imports to the generated code
    #[clap(short, long)]
    include: Vec<String>,

    /// Language of the code blocks to extract. If not provided, the language
    /// will be inferred from the output file extension.
    #[clap(short, long)]
    lang: Option<String>,

    /// Input markdown file
    input: String,

    /// Output file. If not provided, the generated code will be printed to stdout.
    #[clap(default_value = "-")]
    output: String,
}

fn main() -> Result<(), Error> {
    let opts = Opts::parse();

    let input = std::fs::read_to_string(&opts.input)?;

    let lang = if let Some(lang) = opts.lang {
        LANGS
            .find_by_name(&lang)
            .ok_or(format!("Language `{lang}` not supported"))?
    } else {
        LANGS.find_by_extension(&opts.output).ok_or(format!(
            "Output file extension `{}` not supported",
            opts.output
        ))?
    };

    let content = Gen::new(&lang)?.generate(&input, opts.include)?;

    let mut out: Box<dyn Write> = if opts.output == "-" {
        Box::new(std::io::stdout())
    } else {
        Box::new(File::create(opts.output)?)
    };

    out.write_all(content.as_bytes())?;

    Ok(())
}
