use std::io::Write;

use anyhow::{bail, Context, Result};
use clap::Parser;
use clio::{Input, Output};
use tracing::info;

mod parser;

#[derive(Parser, Debug)]
struct Args {
  /// Input file ('-' for stdout)
  #[arg(value_parser, default_value = "-")]
  input: Input,

  /// Output file ('-' for stdout)
  #[arg(long, short, value_parser, default_value = "-", value_name = "PATH")]
  output: Output,

  /// Silence debug output
  #[arg(long, short, default_value = "false")]
  silent: bool,

  /// Ignore input size constraints
  #[arg(long, short, default_value = "false")]
  force: bool,

  /// Return clean HTML, without any JS or CSS
  #[arg(long, short, default_value = "false")]
  clean: bool,
}

fn main() -> Result<()> {
  let mut args = Args::parse();

  // Silence `tracing` output if requested
  if !args.silent {
    tracing_subscriber::fmt::init();
  }

  info!("Reading input...");
  let input = std::io::read_to_string(&mut args.input)?;

  if input.len() > 1024 * 500 && !args.force {
    bail!("Only files up to 500KiB are allowed. Use '--force' if you know what you're doing.")
  }

  info!("Parsing input to html...");
  let html = parser::to_html(&input, &args).context("Couldn't parse input to html")?;

  info!("Writing to output...");
  args
    .output
    .write_all(html.as_bytes())
    .context("Couldn't write to output")?;

  Ok(())
}
