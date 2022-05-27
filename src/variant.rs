use std::fmt::{self, Write};

use crate::fields::Fields;
use crate::formatter::Formatter;

use crate::r#type::Type;

/// Defines an enum variant.
#[derive(Debug, Clone)]
pub struct Variant {
    name: String,

    fields: Fields,

    /// Variant attributes, e.g., `#[serde(rename = "variant")]`.
    attributes: Vec<String>,
}

impl Variant {
    /// Return a new enum variant with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Variant {
            name: name.into(),
            fields: Fields::Empty,
            attributes: Vec::new(),
        }
    }

    /// Add a named field to the variant.
    pub fn named<T>(&mut self, name: &str, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        self.fields.named(name, ty);
        self
    }

    /// Add a tuple field to the variant.
    pub fn tuple(&mut self, ty: &str) -> &mut Self {
        self.fields.tuple(ty);
        self
    }

    /// Add an attribute to the variant.
    pub fn attr(&mut self, attr: impl Into<String>) -> &mut Self {
        self.attributes.push(attr.into());
        self
    }

    /// Formats the variant using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for attr in &self.attributes {
            write!(fmt, "#[{}]\n", attr)?;
        }
        write!(fmt, "{}", self.name)?;
        self.fields.fmt(fmt)?;
        write!(fmt, ",\n")?;

        Ok(())
    }
}
