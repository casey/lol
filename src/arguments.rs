use super::*;

#[derive(Parser)]
#[command(
  version,
  styles = Styles::styled()
    .header(AnsiColor::Green.on_default() | Effects::BOLD)
    .usage(AnsiColor::Green.on_default() | Effects::BOLD)
    .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
    .placeholder(AnsiColor::Cyan.on_default()))
]
#[allow(clippy::arbitrary_source_item_ordering)]
pub(crate) struct Arguments {
  pub(crate) script: Utf8PathBuf,
  #[clap(allow_hyphen_values(true))]
  pub(crate) arguments: Vec<String>,
}
