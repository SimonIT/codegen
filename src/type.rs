use std::fmt::{self, Write};
use regex::Regex;

use crate::formatter::Formatter;

/// Defines a type.
#[derive(Debug, Clone)]
pub struct Type {
    name: String,
    generics: Vec<Type>,
}

fn split_name_and_generic(ty: &str) -> Type {
    let re = Regex::new(r"([^<]*)<(.*)>").unwrap();
    if let Some(captures) = re.captures(ty) {
        let type_name = captures.get(1).unwrap().as_str();
        let generic = captures.get(2).unwrap().as_str();

        let mut new_type = Type::new(type_name);

        // TODO: this won't work if the generic contains multiple fields
        // ex: Map<u8, u8>
        // that can't be solved with regex, so I just leave this as a future problem
        new_type.generic(generic);
        new_type
    } else {
        panic!("Malformed type: {}", ty);
    }
}
impl Type {
    /// Return a new type with the given name.
    pub fn new(name: impl ToString) -> Self {
        let name = name.to_string();
        if name.contains('<') {
            split_name_and_generic(&name)
        } else {
            Type {
                name,
                generics: Vec::new(),
            }
        }
    }

    /// Returns the name of the type
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns the name of the type
    pub fn generics(&self) -> &Vec<Type> {
        &self.generics
    }

    /// Returns the key for sorting
    pub fn key_for_sorting(&self) -> &str {
        match self.name.rfind("::") {
            Some(index) => &self.name[index + 2..],
            None => &self.name,
        }
    }

    /// Add a generic to the type.
    pub fn generic<T>(&mut self, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        // Make sure that the name doesn't already include generics
        assert!(
            !self.name.contains("<"),
            "type name already includes generics"
        );

        self.generics.push(ty.into());
        self
    }

    /// Rewrite the `Type` with the provided path
    ///
    /// TODO: Is this needed?
    pub fn path(&self, path: impl ToString) -> Type {
        // TODO: This isn't really correct
        assert!(!self.name.contains("::"));

        let mut name = path.to_string();
        name.push_str("::");
        name.push_str(&self.name);

        Type {
            name,
            generics: self.generics.clone(),
        }
    }

    /// Formats the struct using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.name)?;
        Type::fmt_slice(&self.generics, fmt)
    }

    fn fmt_slice(generics: &[Type], fmt: &mut Formatter<'_>) -> fmt::Result {
        if !generics.is_empty() {
            write!(fmt, "<")?;

            for (i, ty) in generics.iter().enumerate() {
                if i != 0 {
                    write!(fmt, ", ")?
                }
                ty.fmt(fmt)?;
            }

            write!(fmt, ">")?;
        }

        Ok(())
    }
}

impl<S: ToString> From<S> for Type {
    fn from(src: S) -> Self {
        Type {
            name: src.to_string(),
            generics: vec![],
        }
    }
}

impl<'a> From<&'a Type> for Type {
    fn from(src: &'a Type) -> Self {
        src.clone()
    }
}

#[test]
fn parse_type() {
    {
        let ty = Type::new("u8");
        assert_eq!(ty.name, "u8");
        assert!(ty.generics.is_empty());
    }
}

#[test]
fn parse_generic() {
    {
        let ty = Type::new("Vec<u8>");
        assert_eq!(ty.name, "Vec");
        assert_eq!(ty.generics.iter().map(|generic| generic.name().as_str()).collect::<Vec<&str>>().join(""), "u8");
    }
    {
        let ty = Type::new("Vec<Vec<u8>>");
        assert_eq!(ty.name, "Vec");
        assert_eq!(ty.generics.iter().map(|generic| generic.name().as_str()).collect::<Vec<&str>>().join(""), "Vec<u8>");
    }
}