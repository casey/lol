use {
  self::{arguments::Arguments, error::Error, prompt::Prompt},
  camino::{Utf8Path, Utf8PathBuf},
  clap::{
    Parser,
    builder::{
      Styles,
      styling::{AnsiColor, Effects},
    },
  },
  snafu::{ErrorCompat, IntoError, OptionExt, ResultExt, Snafu},
  std::{
    env,
    fmt::{self, Display, Formatter},
    fs::{self},
    io::{self, IsTerminal},
    os::unix::process::CommandExt,
    path::PathBuf,
    process::{self, Command},
  },
  tempfile::NamedTempFile,
};

mod arguments;
mod error;
mod prompt;

fn main() {
  if let Err(err) = run() {
    eprintln!("error: {err}");

    let causes = err.iter_chain().skip(1).count();

    for (i, err) in err.iter_chain().skip(1).enumerate() {
      eprintln!("       {}─ {err}", if i < causes - 1 { '├' } else { '└' });
    }

    process::exit(1);
  }
}

fn run() -> Result<(), Error> {
  let arguments = Arguments::parse();

  let script = fs::read_to_string(&arguments.script).context(error::Script {
    path: &arguments.script,
  })?;

  let script = if script.starts_with("#!") {
    script.lines().skip(1).collect::<Vec<&str>>().join("\n")
  } else {
    script
  };

  let script = script.trim().to_string();

  let stdin = if io::stdin().is_terminal() {
    None
  } else {
    let mut file = NamedTempFile::new().context(error::Tempfile)?;
    let path = Utf8Path::from_path(file.path())
      .context(error::CurrentDirUnicode { path: file.path() })?
      .to_owned();
    io::copy(&mut io::stdin(), &mut file).context(error::Stdin)?;
    Some((file, path))
  };

  let prompt = Prompt {
    arguments: arguments.arguments,
    current_directory: Utf8PathBuf::from_path_buf(env::current_dir().context(error::CurrentDir)?)
      .map_err(|path| error::CurrentDirUnicode { path }.build())?,
    program: arguments.script.to_string(),
    script: if script.is_empty() {
      None
    } else {
      Some(script)
    },
    stdin: stdin.as_ref().map(|(_file, path)| path.into()),
  };

  let source = Command::new("claude")
    .arg("--dangerously-skip-permissions")
    .arg("--print")
    .arg(prompt.to_string())
    .exec();

  Err(error::Exec.into_error(source))
}
