use super::*;

#[derive(Debug, Snafu)]
#[snafu(context(suffix(false)), visibility(pub(crate)))]
pub(crate) enum Error {
  #[snafu(display("failed to get current directory"))]
  CurrentDir { source: io::Error },
  #[snafu(display("current directory is not valid unicode"))]
  CurrentDirUnicode { path: PathBuf },
  #[snafu(display("failed to exec claude"))]
  Exec { source: io::Error },
  #[snafu(display("failed to read `{path}`"))]
  Script {
    path: Utf8PathBuf,
    source: io::Error,
  },
  #[snafu(display("failed to copy stdin to tempfile"))]
  Stdin { source: io::Error },
  #[snafu(display("failed to create tempfile"))]
  Tempfile { source: io::Error },
  #[snafu(display("tempfile path is not valid unicode"))]
  TempfileUnicode { path: PathBuf },
}
