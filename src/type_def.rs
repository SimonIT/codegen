use std::fmt::{self, Write};

use crate::bound::Bound;
use crate::docs::Docs;
use crate::formatter::{fmt_bounds, Formatter};

use crate::r#type::Type;

/// Defines a type definition.
#[derive(Debug, Clone)]
pub struct TypeDef {
    pub ty: Type,
    vis: Option<String>,
    docs: Option<Docs>,
    derive: Vec<String>,
    allow: Vec<String>,
    attributes: Vec<String>,
    repr: Option<String>,
    bounds: Vec<Bound>,
    macros: Vec<String>,
    cfg_attrs: Vec<String>,
}

impl TypeDef {
    /// Return a structure definition with the provided name
    pub fn new(name: impl ToString) -> Self {
        TypeDef {
            ty: Type::new(name),
            vis: None,
            docs: None,
            derive: Vec::new(),
            allow: Vec::new(),
            attributes: Vec::new(),
            repr: None,
            bounds: Vec::new(),
            macros: Vec::new(),
            cfg_attrs: Vec::new(),
        }
    }

    pub fn vis(&mut self, vis: impl ToString) {
        self.vis = Some(vis.to_string());
    }

    pub fn bound<T>(&mut self, name: impl ToString, ty: T)
    where
        T: Into<Type>,
    {
        self.bounds.push(Bound {
            name: name.to_string(),
            bound: vec![ty.into()],
        });
    }

    pub fn r#macro(&mut self, r#macro: impl ToString) {
        self.macros.push(r#macro.to_string());
    }

    pub fn attr(&mut self, attr: impl ToString) {
        self.attributes.push(attr.to_string());
    }

    pub fn doc(&mut self, docs: impl ToString) {
        self.docs = Some(Docs::new(docs));
    }

    pub fn derive(&mut self, name: impl ToString) {
        self.derive.push(name.to_string());
    }

    pub fn allow(&mut self, allow: impl ToString) {
        self.allow.push(allow.to_string());
    }

    pub fn repr(&mut self, repr: impl ToString) {
        self.repr = Some(repr.to_string());
    }

    pub fn cfg_attr(&mut self, cfg_attr: impl ToString) {
        self.cfg_attrs.push(cfg_attr.to_string());
    }

    pub fn fmt_head(
        &self,
        keyword: &str,
        parents: &[Type],
        fmt: &mut Formatter<'_>,
    ) -> fmt::Result {
        if let Some(ref docs) = self.docs {
            docs.fmt(fmt)?;
        }

        self.fmt_allow(fmt)?;
        self.fmt_derive(fmt)?;
        self.fmt_repr(fmt)?;
        self.fmt_attributes(fmt)?;
        self.fmt_macros(fmt)?;
        self.fmt_cfg_attrs(fmt)?;

        if let Some(ref vis) = self.vis {
            write!(fmt, "{} ", vis)?;
        }

        write!(fmt, "{} ", keyword)?;
        self.ty.fmt(fmt)?;

        if !parents.is_empty() {
            for (i, ty) in parents.iter().enumerate() {
                if i == 0 {
                    write!(fmt, ": ")?;
                } else {
                    write!(fmt, " + ")?;
                }

                ty.fmt(fmt)?;
            }
        }

        fmt_bounds(&self.bounds, fmt)?;

        Ok(())
    }

    fn fmt_attributes(&self, fmt: &mut Formatter) -> fmt::Result {
        for attr in &self.attributes {
            write!(fmt, "#[{}]\n", attr)?;
        }

        Ok(())
    }

    fn fmt_allow(&self, fmt: &mut Formatter) -> fmt::Result {
        for allow in &self.allow {
            write!(fmt, "#[allow({})]\n", allow)?;
        }

        Ok(())
    }

    fn fmt_repr(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref repr) = self.repr {
            write!(fmt, "#[repr({})]\n", repr)?;
        }

        Ok(())
    }

    fn fmt_derive(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if !self.derive.is_empty() {
            write!(fmt, "#[derive(")?;

            for (i, name) in self.derive.iter().enumerate() {
                if i != 0 {
                    write!(fmt, ", ")?
                }
                write!(fmt, "{}", name)?;
            }

            write!(fmt, ")]\n")?;
        }

        Ok(())
    }

    fn fmt_macros(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for m in self.macros.iter() {
            write!(fmt, "{}\n", m)?;
        }
        Ok(())
    }

    fn fmt_cfg_attrs(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for attr in &self.cfg_attrs {
            write!(fmt, "#[cfg_attr({})]\n", attr)?;
        }

        Ok(())
    }
}
