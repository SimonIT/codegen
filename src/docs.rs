use std::fmt::{self, Write};

use crate::formatter::Formatter;

#[derive(Debug, Clone)]
pub struct Docs {
    docs: String,
}

impl Docs {
    pub fn new(docs: impl ToString) -> Self {
        Docs {
            docs: docs.to_string(),
        }
    }

    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for line in self.docs.lines() {
            write!(fmt, "///")?;
            if !line.is_empty() {
                write!(fmt, " {}", line)?;
            }
            writeln!(fmt)?;
        }

        Ok(())
    }

    pub fn append(&mut self, other: impl ToString) -> &mut Self {
        self.docs.push_str(&other.to_string());
        self
    }

    pub fn to_str(&self) -> &str {
        &self.docs
    }
}
