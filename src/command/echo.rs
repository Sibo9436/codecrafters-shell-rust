use super::Runner;

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
