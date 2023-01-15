/// Defines an import (`use` statement).
#[derive(Debug, Clone)]
pub struct Import {
    line: String,

    /// Function visibility
    pub vis: Option<String>,

    /// Alias using the `as` keyword
    pub alias: Option<String>,
}

impl Import {
    /// Return a new import.
    pub fn new(path: impl ToString, ty: impl ToString, alias: Option<&str>) -> Self {
        let base_line = format!("{}::{}", path.to_string(), ty.to_string());
        Import {
            line: match alias {
                None => base_line,
                Some(str) => format!("{} as {}", base_line, str),
            },
            vis: None,
            alias: alias.map(ToOwned::to_owned),
        }
    }

    /// Set the import visibility.
    pub fn vis(&mut self, vis: impl ToString) -> &mut Self {
        self.vis = Some(vis.to_string());
        self
    }

    /// Set the import visibility.
    pub fn alias(&mut self, alias: Option<&str>) -> &mut Self {
        self.alias = alias.map(ToOwned::to_owned);
        self
    }
}
