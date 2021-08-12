use std::collections::HashMap;
use std::{fmt, fmt::Write};
#[derive(Clone, Debug, Default)]
pub struct Data {
    data: HashMap<u32, Vec<String>>,
}
impl Data {
    pub fn new() -> Data {
        Data {
            data: Default::default(),
        }
    }
    pub fn get_mut(&mut self) -> &mut HashMap<u32, Vec<String>> {
        &mut self.data
    }
}
impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = format!("");
        for (pid, msgs) in &self.data {
            write!(s, "\nClient PID {}:", pid)?;
            for msg in msgs {
                write!(s, " '{}'", msg)?;
            }
        }
        write!(f, "{}", s)
    }
}
