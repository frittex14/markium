use anyhow::{anyhow, Result};
use markdown::Options;
use maud::{html, PreEscaped, DOCTYPE};
use minify_html::{minify, Cfg};
use regex::{Captures, Regex};

use crate::Args;

const CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/style.css"));

pub fn to_html(markdown: &str, args: &Args) -> Result<String> {
  // Try to get title from output path, otherwise fallback to "Markdown Preview"
  let title = if !args.input.path().is_std() {
    (|| Some(args.input.path().file_name()?.to_str()?.to_string()))()
      .unwrap_or(String::from("Markdown Preview"))
  } else {
    String::from("Markdown Preview")
  };

  // Construct parser and compiler options
  let mut options = Options::gfm();
  options.compile.allow_dangerous_html = true;

  // Convert markdown to raw html
  let html = markdown::to_html_with_options(markdown, &options).map_err(|err| anyhow!(err))?;

  // Regex to find <pre><code> blocks
  let regex = Regex::new(r"<pre><code([\s\S]*?)>([\s\S]*?)</code></pre>")?;

  // Trim end newlines from contents of <code> blocks
  let html = regex
    .replace_all(&html, |snippet: &Captures| {
      format!(
        "<pre><code{}>{}</code></pre>",
        &snippet[1],
        &snippet[2].trim_end()
      )
    })
    .to_string();

  // Build the html document
  let document = html! {
    (DOCTYPE)
    html {
      head {
        meta charset="UTF-8";
        meta name="viewport" content="width=device-width, initial-scale=1";
        @if !args.clean {
          style type="text/css" { (PreEscaped(CSS)) }
        }
        title { (title) }
      }
      body {
        @if !args.clean {
          input type="checkbox" id="theme-switch";
          input type="checkbox" id="preview-size";
          div id="container" {
            div {
              div id="header" {
                div {
                  span {
                    (PreEscaped("<svg aria-hidden='true' focusable='false' role='img' class='octicon octicon-book' viewBox='0 0 16 16' width='16' height='16' fill='currentColor' style='display:inline-block;user-select:none;vertical-align:text-bottom;overflow:visible'><path d='M0 1.75A.75.75 0 0 1 .75 1h4.253c1.227 0 2.317.59 3 1.501A3.743 3.743 0 0 1 11.006 1h4.245a.75.75 0 0 1 .75.75v10.5a.75.75 0 0 1-.75.75h-4.507a2.25 2.25 0 0 0-1.591.659l-.622.621a.75.75 0 0 1-1.06 0l-.622-.621A2.25 2.25 0 0 0 5.258 13H.75a.75.75 0 0 1-.75-.75Zm7.251 10.324.004-5.073-.002-2.253A2.25 2.25 0 0 0 5.003 2.5H1.5v9h3.757a3.75 3.75 0 0 1 1.994.574ZM8.755 4.75l-.004 7.322a3.752 3.752 0 0 1 1.992-.572H14.5v-9h-3.495a2.25 2.25 0 0 0-2.25 2.25Z'></path></svg>"))
                    (title.strip_suffix(".md").unwrap_or(&title))
                  }
                  div {
                    label for="theme-switch" {}
                    label for="preview-size" {}
                  }
                }
              }
              div id="content" {
                div {
                  (PreEscaped(html.trim()))
                }
              }
            }
          }
        } @else {
          div {
            (PreEscaped(html.trim()))
          }
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
