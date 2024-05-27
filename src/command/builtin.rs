use super::{CommandError, Runner};
use core::fmt;
use std::process::exit;

pub(super) enum Builtin {
    Echo(Echo),
    Exit(ExitRunner),
    Type(Type),
}

impl Runner for Builtin {
    fn run(&self) -> Result<String, CommandError> {
        match self {
            Builtin::Echo(e) => e.run(),
            Builtin::Exit(e) => e.run(),
            Builtin::Type(e) => e.run(),
        }
    }
}

pub(super) struct Echo {
    args: String,
}

impl Echo {
    pub(super) fn new(args: &[&str]) -> Echo {
        Echo {
            args: args
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(" "),
        }
    }
}
impl Runner for Echo {
    fn run(&self) -> Result<String, super::CommandError> {
        Ok(self.args.clone())
    }
}

pub(crate) struct ExitRunner {
    code: i32,
}

impl ExitRunner {
    pub(crate) fn new(args: &[&str]) -> Result<Self, CommandError> {
        if args.is_empty() {
            return Err(CommandError::NotEnoughArguments {});
        }
        let code = i32::from_str_radix(args[0], 10).unwrap_or(0);
        Ok(Self { code })
    }
}

impl Runner for ExitRunner {
    fn run(&self) -> Result<String, super::CommandError> {
        exit(self.code)
    }
}

pub(super) struct Type {
    arg: String,
}

impl Type {
    pub(super) fn new(args: &[&str]) -> Self {
        let arg = args[0].trim().to_string();
        Self { arg }
    }
}

impl Runner for Type {
    // TODO: I can do better
    fn run(&self) -> Result<String, CommandError> {
        Ok(match self.arg.as_str() {
            "echo" => "echo is a shell builtin".to_string(),
            "exit" => "exit is a shell builtin".to_string(),
            "type" => "type is a shell builtin".to_string(),
            v => format!("{v} not found"),
        })
    }
}
