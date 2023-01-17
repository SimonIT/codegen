use std::fmt::{self, Write};

use crate::bound::Bound;
use crate::field::Field;
use crate::formatter::{fmt_bounds, fmt_generics, Formatter};
use crate::function::Function;

use crate::r#type::Type;

/// Defines an impl block.
#[derive(Debug, Clone)]
pub struct Impl {
    /// The struct being implemented
    target: Type,

    /// Impl level generics
    generics: Vec<String>,

    /// If implementing a trait
    impl_trait: Option<Type>,

    /// Associated constants
    assoc_csts: Vec<Field>,

    /// Associated types
    assoc_tys: Vec<Field>,

    /// Bounds
    bounds: Vec<Bound>,

    fns: Vec<Function>,

    macros: Vec<String>,
}

impl Impl {
    /// Return a new impl definition
    pub fn new<T>(target: T) -> Self
    where
        T: Into<Type>,
    {
        Impl {
            target: target.into(),
            generics: Vec::new(),
            impl_trait: None,
            assoc_csts: Vec::new(),
            assoc_tys: Vec::new(),
            bounds: Vec::new(),
            fns: Vec::new(),
            macros: Vec::new(),
        }
    }

    /// Returns the target of the impl
    pub fn target(&self) -> &Type {
        &self.target
    }

    /// Returns the key for sorting
    pub fn key_for_sorting(&self) -> &Type {
        if self.target.generics().is_empty() || self.impl_trait.is_none() {
            &self.target
        } else {
            let impl_type = self.impl_trait.as_ref().unwrap();
            if impl_type.name() == "From" {
                impl_type.generics().get(0).unwrap()
            } else {
                impl_type
            }
        }
    }


    /// Add a generic to the impl block.
    ///
    /// This adds the generic for the block (`impl<T>`) and not the target type.
    pub fn generic(&mut self, name: impl ToString) -> &mut Self {
        self.generics.push(name.to_string());
        self
    }

    /// Add a generic to the target type.
    pub fn target_generic<T>(&mut self, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        self.target.generic(ty);
        self
    }

    /// Set the trait that the impl block is implementing.
    pub fn impl_trait<T>(&mut self, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        self.impl_trait = Some(ty.into());
        self
    }

    /// Add a macro to the impl block (e.g. `"#[async_trait]"`)
    pub fn r#macro(&mut self, r#macro: impl ToString) -> &mut Self {
        self.macros.push(r#macro.to_string());
        self
    }

    /// Set an associated constant.
    pub fn associate_const<T>(
        &mut self,
        name: impl ToString,
        ty: T,
        value: impl ToString,
        visibility: impl ToString,
    ) -> &mut Self
    where
        T: Into<Type>,
    {
        self.assoc_csts.push(Field {
            name: name.to_string(),
            ty: ty.into(),
            documentation: String::new(),
            annotation: Vec::new(),
            value: value.to_string(),
            visibility: Some(visibility.to_string()),
        });

        self
    }

    /// Set an associated type.
    pub fn associate_type<T>(&mut self, name: impl ToString, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        self.assoc_tys.push(Field {
            name: name.to_string(),
            ty: ty.into(),
            documentation: String::new(),
            annotation: Vec::new(),
            value: String::new(),
            visibility: None,
        });

        self
    }

    /// Add a `where` bound to the impl block.
    pub fn bound<T>(&mut self, name: impl ToString, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        self.bounds.push(Bound {
            name: name.to_string(),
            bound: vec![ty.into()],
        });
        self
    }

    /// Push a new function definition, returning a mutable reference to it.
    pub fn new_fn(&mut self, name: impl ToString) -> &mut Function {
        self.push_fn(Function::new(name));
        self.fns.last_mut().unwrap()
    }

    /// Push a function definition.
    pub fn push_fn(&mut self, item: Function) -> &mut Self {
        self.fns.push(item);
        self
    }

    /// Formats the impl block using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for m in self.macros.iter() {
            write!(fmt, "{}\n", m)?;
        }
        write!(fmt, "impl")?;
        fmt_generics(&self.generics[..], fmt)?;

        if let Some(ref t) = self.impl_trait {
            write!(fmt, " ")?;
            t.fmt(fmt)?;
            write!(fmt, " for")?;
        }

        write!(fmt, " ")?;
        self.target.fmt(fmt)?;

        fmt_bounds(&self.bounds, fmt)?;

        fmt.block(|fmt| {
            // format associated constants
            if !self.assoc_csts.is_empty() {
                for cst in &self.assoc_csts {
                    if let Some(vis) = &cst.visibility {
                        write!(fmt, "{} ", vis)?;
                    }
                    write!(fmt, "const {}: ", cst.name)?;
                    cst.ty.fmt(fmt)?;
                    write!(fmt, " = {};\n", cst.value)?;
                }
            }

            // format associated types
            if !self.assoc_tys.is_empty() {
                for ty in &self.assoc_tys {
                    write!(fmt, "type {} = ", ty.name)?;
                    ty.ty.fmt(fmt)?;
                    write!(fmt, ";\n")?;
                }
            }

            for (i, func) in self.fns.iter().enumerate() {
                if i != 0 || !self.assoc_tys.is_empty() {
                    write!(fmt, "\n")?;
                }

                func.fmt(false, fmt)?;
            }

            Ok(())
        })
    }
}

#[test]
fn type_alias() {
    {
        let mut generic_type = Type::new("From");
        generic_type.generic("Bar");

        let mut impl_type = Impl::new("Foo");
        impl_type.impl_trait(generic_type);

        let mut str = String::new();
        impl_type.fmt(&mut Formatter::new(&mut str)).unwrap();
        assert_eq!(impl_type.key_for_sorting().name(), "Foo");
    }
    {
        let mut generic_type = Type::new("From");
        generic_type.generic("Bar");

        let mut impl_type = Impl::new("Vec");
        impl_type.target_generic("Foo");
        impl_type.impl_trait(generic_type);

        let mut str = String::new();
        impl_type.fmt(&mut Formatter::new(&mut str)).unwrap();
        println!("{:?}", str);
        println!("{:?}", impl_type.key_for_sorting());
        assert_eq!(impl_type.key_for_sorting().name(), "Bar");
    }
}
