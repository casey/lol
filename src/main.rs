use {
  clap::{
    Parser,
    builder::{
      Styles,
      styling::{AnsiColor, Effects},
    },
  },
  std::{
    fs::{self, File},
    io::{self, IsTerminal, Read, Write},
    os::unix::process::CommandExt,
    path::PathBuf,
    process::Command,
  },
};

#[derive(Parser)]
#[command(
  version,
  styles = Styles::styled()
    .header(AnsiColor::Green.on_default() | Effects::BOLD)
    .usage(AnsiColor::Green.on_default() | Effects::BOLD)
    .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
    .placeholder(AnsiColor::Cyan.on_default()))
]
pub(crate) struct Arguments {
  script: PathBuf,
  arguments: Vec<String>,
}

fn main() {
  let arguments = Arguments::parse();

  let content = match fs::read_to_string(&arguments.script) {
    Ok(c) => c,
    Err(e) => {
      eprintln!("Failed to read {}: {}", &arguments.script.display(), e);
      std::process::exit(1);
    }
  };

  let prompt = if content.starts_with("#!") {
    content.lines().skip(1).collect::<Vec<&str>>().join("\n")
  } else {
    content
  };

  let prompt = prompt.trim_start().to_string();

  if prompt.is_empty() {
    eprintln!("Nothing to do. Write a description of what you want done.");
    std::process::exit(1);
  }

  let mut full_prompt = prompt;

  if !arguments.arguments.is_empty() {
    full_prompt.push_str("\n\n---\n\n## Arguments\n\n");
    for (i, arg) in arguments.arguments.iter().enumerate() {
      full_prompt.push_str(&format!("- `${}`: `{}`\n", i + 1, arg));
    }
  }

  if !io::stdin().is_terminal() {
    let mut stdin_content = Vec::new();
    if let Ok(n) = io::stdin().read_to_end(&mut stdin_content) {
      if n > 0 {
        let stdin_path = format!("/tmp/dwim-stdin-{}", std::process::id());
        match File::create(&stdin_path).and_then(|mut f| f.write_all(&stdin_content)) {
          Ok(()) => {
            full_prompt.push_str("\n\n---\n\n## Stdin\n\n");
            full_prompt.push_str(&format!(
              "Input was piped to this script. Contents are available at: `{}`\n",
              stdin_path
            ));
          }
          Err(e) => {
            eprintln!("Warning: failed to write stdin to temp file: {}", e);
          }
        }
      }
    }
  }

  let err = Command::new("claude")
    .arg("--dangerously-skip-permissions")
    .arg("--print")
    .arg(&full_prompt)
    .exec();

  eprintln!("Failed to exec claude: {}", err);
  eprintln!("Is claude CLI installed? Try: npm install -g @anthropic-ai/claude-code");
  std::process::exit(1);
}
