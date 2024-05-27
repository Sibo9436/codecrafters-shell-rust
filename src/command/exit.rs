use std::process::exit;

use super::{CommandError, Runner};

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
