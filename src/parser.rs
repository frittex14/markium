use anyhow::{anyhow, Result};
use markdown::Options;
use maud::{html, PreEscaped, DOCTYPE};
use minify_html::{minify, Cfg};

use crate::Args;

pub fn to_html(markdown: &str, args: &Args) -> Result<String> {
  // Try to get title from output path, otherwise fallback to "Markdown Preview"
  let title = if !args.input.path().is_std() {
    (|| Some(args.input.path().file_name()?.to_str()?.to_string()))()
      .unwrap_or(String::from("Markdown Preview"))
  } else {
    String::from("Markdown Preview")
  };

  // Convert markdown to raw html
  let html =
    markdown::to_html_with_options(markdown, &Options::gfm()).map_err(|err| anyhow!(err))?;

  // Build the html document
  let document = html! {
    (DOCTYPE)
    html {
      head {
        meta charset="UTF-8";
        meta name="viewport" content="width=device-width, initial-scale=1";
        title { (title) }
      }
      body {
        div id="content" {
          (PreEscaped(html.trim()))
        }
      }
    }
  }
  .into_string();

  // Minify the html document. If fails, fallback to the original document
  let output =
    String::from_utf8(minify(document.as_bytes(), &Cfg::spec_compliant())).unwrap_or(document);

  Ok(output)
}
