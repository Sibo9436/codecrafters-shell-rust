use std::process;

use thiserror::Error;

use self::{
    builtin::{Echo, Pwd, Type},
    path::find_in_path,
};

mod builtin;
mod path;

#[derive(Error, Debug)]
pub(crate) enum CommandError {
    #[error("{0}: command not found")]
    CommandNotFound(String),
    #[error("not enough arguments for command")]
    NotEnoughArguments {},
    #[error("fatal error: {0}")]
    Fatal(&'static str),
}

// NOTE: should I also define a runner definition for this?
pub(crate) trait Runner {
    fn run(&self) -> Result<String, CommandError>;
}

#[allow(dead_code)]
struct CommandRunner {
    cmd: String,
    args: Vec<String>,
}
impl CommandRunner {
    fn new(toks: &[&str]) -> Self {
        CommandRunner {
            cmd: toks[0].to_owned(),
            args: toks[1..].iter().map(|s| s.to_string()).collect(),
        }
    }
}
impl Runner for CommandRunner {
    fn run(&self) -> Result<String, CommandError> {
        // TODO: here we should look in the path directories :)
        if let Some(path) = find_in_path(&self.cmd) {
            //tecnicamente process::Command cerca già da solo nel path ma who cares
            let _status = process::Command::new(path).args(&self.args).status();
            Ok("".to_string())
        } else if self.cmd.starts_with("/") {
            let _status = process::Command::new(&self.cmd).args(&self.args).status();
            Ok("".to_string())
        } else {
            Err(CommandError::CommandNotFound(self.cmd.to_string()))
        }
    }
}

pub(crate) fn parse_command(input: &str) -> Result<String, CommandError> {
    let toks: Vec<&str> = input
        .split(' ')
        .filter(|&s| !s.is_empty())
        .map(|s| s.trim())
        .collect();
    if toks.is_empty() {
        return Ok(String::from(""));
    }
    let runner = init_runner(&toks)?;
    runner.run()
}

fn init_runner(toks: &[&str]) -> Result<Box<dyn Runner + 'static>, CommandError> {
    match toks[0] {
        "exit" => match builtin::ExitRunner::new(&toks[1..]) {
            Ok(runner) => Ok(Box::new(runner)),
            Err(e) => Err(e),
        },
        "type" => {
            if toks[1..].is_empty() {
                Err(CommandError::NotEnoughArguments {})
            } else {
                Ok(Box::new(Type::new(&toks[1..])))
            }
        }
        "echo" => Ok(Box::new(Echo::new(&toks[1..]))),
        "pwd" => Ok(Box::new(Pwd)),
        _ => Ok(Box::new(CommandRunner::new(toks))),
    }
}
