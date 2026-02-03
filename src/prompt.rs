use super::*;

pub(crate) struct Prompt {
  pub(crate) arguments: Vec<String>,
  pub(crate) current_directory: Utf8PathBuf,
  pub(crate) program: String,
  pub(crate) script: Option<String>,
  pub(crate) stdin: Option<Utf8PathBuf>,
}

impl Display for Prompt {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    if let Some(script) = &self.script {
      writeln!(f, "{script}")?;
    } else {
      writeln!(
        f,
        "You have been invoked from the command line. Please infer what the user wants you to do \
        from the program name, arguments, and current directory. May this prayer guide you on your \
        way. Amen."
      )?;
    }
    writeln!(f, "---")?;
    writeln!(f, "program: {}", self.program)?;
    writeln!(f, "arguments:")?;
    for argument in &self.arguments {
      writeln!(f, "- {argument}")?;
    }
    writeln!(f, "current directory: {}", self.current_directory)?;
    if let Some(stdin) = &self.stdin {
      writeln!(f, "stdin: {stdin}")?;
    }
    Ok(())
  }
}
