#[allow(unused_imports)]
use log::error;
use std::io::{self, Write};
use std::process::{Command, ExitStatus};

use super::strip_trailing_newline;

#[derive(Debug)]
// https://doc.rust-lang.org/std/process/struct.Output.html
pub struct ExecuteCommandOutcome {
  /// The status (exit code) of the process.
  pub status: ExitStatus,
  /// The data that the process wrote to stdout.
  pub stdout: Vec<u8>,
  /// The data that the process wrote to stderr.
  pub stderr: Vec<u8>,
  /// success is defined as a zero exit status
  pub success: bool,
  /// error code
  pub error_code: i32,
  /// The data that the process wrote to stdout converted from bytes to string
  pub stdout_string: String,
  /// The data that the process wrote to stderr converted from bytes to string
  pub stderr_string: String,
}

pub fn execute_command(args: &[String], hide_stdout: bool) -> ExecuteCommandOutcome {
  let output = if cfg!(target_os = "windows") {
    Command::new("cmd").args(args).output().expect("failed to execute command")
  } else {
    Command::new("bash").args(args).output().expect("failed to execute command")
  };

  if !hide_stdout {
    io::stdout().write_all(&output.stdout).unwrap();
  }
  io::stderr().write_all(&output.stderr).unwrap();

  // Returns the exit code of the process, if any.
  let error_code = match output.status.code() {
    Some(code) => code,
    None => -1,
  };
  // assert!(output.status.success());
  ExecuteCommandOutcome {
    status: output.status,
    stdout: output.stdout.clone(),
    stderr: output.stderr.clone(),
    success: output.status.success(),
    error_code,
    stdout_string: String::from_utf8(output.stdout).expect("invalid UTF-8"),
    stderr_string: String::from_utf8(output.stderr).expect("invalid UTF-8"),
  }
}

/// a quick helper to execute commands and pass only command string,
/// this version is the one that is used with `Result<String, String>`
pub fn execute_command_shortcut(command: &str) -> Result<String, String> {
  // get Version
  let command_args = &[String::from("-c"), String::from(command)];
  // debug!("{:?}", command_args);
  let command_outcome: ExecuteCommandOutcome = execute_command(command_args, true);
  if command_outcome.error_code != 0 {
    error!("error_code: {}", command_outcome.error_code);
    // Err(format!("error code '{}'", command_outcome.error_code));
  }
  match command_outcome.error_code {
    0 => Ok(strip_trailing_newline(&command_outcome.stdout_string).to_string()),
    _ => Err(format!("error code '{}'", command_outcome.error_code)),
  }
}

pub fn execute_syslog_start_stop(action: &str) -> Result<String, String> {
  // use sudo to prevent intereactivity in debug
  let command = format!("sudo systemctl {} syslog.socket rsyslog.service", action);
  // let propagate the error
  execute_command_shortcut(&command)
}