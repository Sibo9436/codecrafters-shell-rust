use thiserror::Error;

mod builtin;

#[derive(Error, Debug)]
pub(crate) enum CommandError {
    #[error("{0}: command not found")]
    CommandNotFound(String),
}

pub(crate) trait Runner {
    fn run(&self) -> Result<String, CommandError>;
}

#[allow(dead_code)]
struct CommandRunner<'a> {
    cmd: &'a str,
    args: &'a [&'a str],
}
impl<'a> Runner for CommandRunner<'a> {
    fn run(&self) -> Result<String, CommandError> {
        // TODO: here we should look in the path directories :)
        Err(CommandError::CommandNotFound(self.cmd.to_string()))
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
    let runner = init_runner(&toks);
    runner.and_then(|r| r.run())
}

fn init_runner<'a>(toks: &'a [&'a str]) -> Result<impl Runner + 'a, CommandError> {
    Ok(CommandRunner {
        cmd: toks[0],
        args: &toks[1..],
    })
}
