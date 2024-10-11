pub trait Command: Sized {
    fn stringify(&self) -> String;
    fn parse(text: &str) -> Result<Self, String>;
}

pub fn parse_command<T: Command>(text: &str) -> Result<T, String> {
    Command::parse(text)
}