use super::{path::find_in_path, CommandError, Runner};
use std::{
    env,
    path::{self, Path},
    process::exit,
    str::FromStr,
};

#[allow(unused)]
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
        Ok(self.args.clone() + "\n")
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
            // TODO: migliorabile
            "echo" => "echo is a shell builtin\n".to_string(),
            "exit" => "exit is a shell builtin\n".to_string(),
            "type" => "type is a shell builtin\n".to_string(),
            "pwd" => "pwd is a shell builtin\n".to_string(),
            "cd" => "cd is a shell builtin\n".to_string(),
            v => {
                if let Some(path) = find_in_path(&self.arg) {
                    format!("{v} is {}\n", path.display())
                } else {
                    format!("{v} not found\n")
                }
            }
        })
    }
}

pub(super) struct Pwd;

impl Runner for Pwd {
    fn run(&self) -> Result<String, CommandError> {
        env::current_dir()
            .map_err(|_| CommandError::Fatal("could not read pwd"))
            .and_then(|p| {
                p.to_str()
                    .ok_or(CommandError::Fatal(":("))
                    .map(str::to_owned)
            })
            .map(|mut pwd| {
                pwd.push('\n');
                pwd
            })
    }
}

pub(super) struct Cd<'a> {
    dir: &'a str,
}
impl<'a> Cd<'a> {
    pub(super) fn new(dir: &'a str) -> Self {
        Self { dir }
    }
}

impl<'a> Runner for Cd<'a> {
    fn run(&self) -> Result<String, CommandError> {
        let path_to_set = if self.dir.starts_with("/") {
            let path =
                path::PathBuf::from_str(&self.dir).map_err(|_| CommandError::Fatal("non so"))?;
            path
        } else if self.dir.starts_with("./") {
            let current = Pwd.run()? + &self.dir[2..];
            let current = path::Path::new(&current);
            current.to_path_buf()
        } else {
            let current = Pwd.run()?;
            // NOTE:  a lot of allocations going on round here
            let mut current =
                path::PathBuf::from_str(&current).map_err(|_| CommandError::Fatal("non so"))?;
            let p_split = self.dir.split("/");
            for d in p_split {
                if d == ".." {
                    current = current
                        .parent()
                        .map(Path::to_path_buf)
                        .ok_or_else(|| CommandError::PathNotFound(self.dir.to_owned()))?;
                } else {
                    current = current.join(d)
                }
            }
            current.to_path_buf()
        };
        if let Ok(true) = path_to_set.try_exists() {
            env::set_current_dir(&path_to_set)
                .map_err(|_| CommandError::PathNotFound(self.dir.to_owned()))?;
            Ok(String::new())
        } else {
            Err(CommandError::PathNotFound(self.dir.to_owned()))
        }
    }
}
