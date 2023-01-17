use std::collections::BTreeMap;
use std::fmt::{self, Debug, Display, Write};

use indexmap::IndexMap;

use crate::docs::Docs;
use crate::formatter::Formatter;
use crate::function::Function;
use crate::import::Import;
use crate::item::Item;
use crate::module::Module;

use crate::r#enum::Enum;
use crate::r#impl::Impl;
use crate::r#struct::Struct;
use crate::r#trait::Trait;
use crate::type_alias::TypeAlias;

/// Defines a scope.
///
/// A scope contains modules, types, etc...
#[derive(Debug, Clone)]
pub struct Scope {
    /// Scope documentation
    docs: Option<Docs>,

    /// Imports
    imports: IndexMap<String, IndexMap<String, Import>>,

    /// Contents of the documentation,
    items: Vec<Item>,
}

impl Scope {
    /// Returns a new scope
    pub fn new() -> Self {
        Scope {
            docs: None,
            imports: IndexMap::new(),
            items: vec![],
        }
    }

    /// Import a type into the scope.
    ///
    /// This results in a new `use` statement being added to the beginning of
    /// the scope.
    pub fn new_import(&mut self, path: impl ToString, ty: impl ToString, alias: Option<&str>) -> &mut Import {
        // handle cases where the caller wants to refer to a type namespaced
        // within the containing namespace, like "a::B".
        let ty = ty.to_string();
        let ty = ty.split("::").next().unwrap_or_else(|| ty.as_str());
        self.imports
            .entry(path.to_string())
            .or_insert(IndexMap::new())
            .entry(ty.to_string())
            .or_insert_with(|| Import::new(path, ty, alias))
    }

    /// Push a new import (`use` statement) ad the beginning of the scope
    pub fn push_import(&mut self, path: impl ToString, ty: impl ToString, alias: Option<&str>) -> &mut Self {
        self.new_import(path, ty, alias);
        self
    }

    /// Push a new module definition, returning a mutable reference to it.
    ///
    /// # Panics
    ///
    /// Since a module's name must uniquely identify it within the scope in
    /// which it is defined, pushing a module whose name is already defined
    /// in this scope will cause this function to panic.
    ///
    /// In many cases, the [`get_or_new_module`] function is preferrable, as it
    /// will return the existing definition instead.
    ///
    /// [`get_or_new_module`]: #method.get_or_new_module
    pub fn new_module(&mut self, name: impl ToString) -> &mut Module {
        self.push_module(Module::new(name));

        match *self.items.last_mut().unwrap() {
            Item::Module(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Returns a mutable reference to a module if it is exists in this scope.
    pub fn get_module_mut<Q: ?Sized>(&mut self, name: &Q) -> Option<&mut Module>
    where
        String: PartialEq<Q>,
    {
        self.items
            .iter_mut()
            .filter_map(|item| match item {
                &mut Item::Module(ref mut module) if module.name == *name => Some(module),
                _ => None,
            })
            .next()
    }

    /// Returns a mutable reference to a module if it is exists in this scope.
    pub fn get_module<Q: ?Sized>(&self, name: &Q) -> Option<&Module>
    where
        String: PartialEq<Q>,
    {
        self.items
            .iter()
            .filter_map(|item| match item {
                &Item::Module(ref module) if module.name == *name => Some(module),
                _ => None,
            })
            .next()
    }

    /// Returns a mutable reference to a module, creating it if it does
    /// not exist.
    pub fn get_or_new_module<Q: ?Sized + Display>(&mut self, name: &Q) -> &mut Module
    where
        String: PartialEq<Q>,
    {
        if self.get_module(name).is_some() {
            self.get_module_mut(name).unwrap()
        } else {
            self.new_module(name)
        }
    }

    /// Push a module definition.
    ///
    /// # Panics
    ///
    /// Since a module's name must uniquely identify it within the scope in
    /// which it is defined, pushing a module whose name is already defined
    /// in this scope will cause this function to panic.
    ///
    /// In many cases, the [`get_or_new_module`] function is preferrable, as it will
    /// return the existing definition instead.
    ///
    /// [`get_or_new_module`]: #method.get_or_new_module
    pub fn push_module(&mut self, item: Module) -> &mut Self {
        assert!(self.get_module(&item.name).is_none());
        self.items.push(Item::Module(item));
        self
    }

    /// Push a new struct definition, returning a mutable reference to it.
    pub fn new_struct(&mut self, name: impl ToString) -> &mut Struct {
        self.push_struct(Struct::new(name));

        match *self.items.last_mut().unwrap() {
            Item::Struct(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a struct definition
    pub fn push_struct(&mut self, item: Struct) -> &mut Self {
        self.items.push(Item::Struct(item));
        self
    }

    /// Push a new function definition, returning a mutable reference to it.
    pub fn new_fn(&mut self, name: impl ToString) -> &mut Function {
        self.push_fn(Function::new(name));

        match *self.items.last_mut().unwrap() {
            Item::Function(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a function definition
    pub fn push_fn(&mut self, item: Function) -> &mut Self {
        self.items.push(Item::Function(item));
        self
    }

    /// Push a new trait definition, returning a mutable reference to it.
    pub fn new_trait(&mut self, name: impl ToString) -> &mut Trait {
        self.push_trait(Trait::new(name));

        match *self.items.last_mut().unwrap() {
            Item::Trait(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a trait definition
    pub fn push_trait(&mut self, item: Trait) -> &mut Self {
        self.items.push(Item::Trait(item));
        self
    }

    /// Push a new struct definition, returning a mutable reference to it.
    pub fn new_enum(&mut self, name: impl ToString) -> &mut Enum {
        self.push_enum(Enum::new(name));

        match *self.items.last_mut().unwrap() {
            Item::Enum(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a structure definition
    pub fn push_enum(&mut self, item: Enum) -> &mut Self {
        self.items.push(Item::Enum(item));
        self
    }

    /// Push a new `impl` block, returning a mutable reference to it.
    pub fn new_impl(&mut self, target: impl ToString) -> &mut Impl {
        self.push_impl(Impl::new(target));

        match *self.items.last_mut().unwrap() {
            Item::Impl(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Push an `impl` block.
    pub fn push_impl(&mut self, item: Impl) -> &mut Self {
        self.items.push(Item::Impl(item));
        self
    }

    /// Push a raw string to the scope.
    ///
    /// This string will be included verbatim in the formatted string.
    pub fn raw(&mut self, val: impl ToString) -> &mut Self {
        self.items.push(Item::Raw(val.to_string()));
        self
    }

    /// Push a new `TypeAlias`, returning a mutable reference to it.
    pub fn new_type_alias(&mut self, name: impl ToString, target: impl ToString) -> &mut TypeAlias {
        self.push_type_alias(TypeAlias::new(name, target));

        match *self.items.last_mut().unwrap() {
            Item::TypeAlias(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Push an `TypeAlias`.
    pub fn push_type_alias(&mut self, item: TypeAlias) -> &mut Self {
        self.items.push(Item::TypeAlias(item));
        self
    }

    /// Return a string representation of the scope.
    pub fn to_string(&self) -> String {
        let mut ret = String::new();

        self.fmt(&mut Formatter::new(&mut ret)).unwrap();

        // Remove the trailing newline
        if ret.as_bytes().last() == Some(&b'\n') {
            ret.pop();
        }

        ret
    }

    /// Formats the scope using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        {
            let mut has_raw = false;
            for item in self.items.iter() {
                if let Item::Raw(ref v) = *item {
                    writeln!(fmt, "{}", v)?;
                    has_raw = true;
                }
            }
            if has_raw {
                writeln!(fmt)?;
            }
        }

        {
            self.fmt_imports(fmt)?;
            if !self.imports.is_empty() {
                writeln!(fmt)?;
            }
        }

        let mut sorted_items = BTreeMap::<String, Vec<&Item>>::new();
        for item in self.items.iter() {
            match *item {
                Item::Module(ref v) => sorted_items.entry(format!("{}-module", v.name)).or_default().push(item),
                // note: purposely use `astruct` instead of `struct` to make sure the struct always comes first in alphabetical order
                Item::Struct(ref v) => sorted_items.entry(format!("{}-astruct", v.ty().key_for_sorting())).or_default().push(item),
                Item::Function(ref v) => sorted_items.entry(format!("{}-function", v.name())).or_default().push(item),
                Item::Trait(ref v) => sorted_items.entry(format!("{}-trait", v.ty().key_for_sorting())).or_default().push(item),
                Item::Enum(ref v) => sorted_items.entry(format!("{}-enum", v.ty().key_for_sorting())).or_default().push(item),
                Item::Impl(ref v) => sorted_items.entry(format!("{}-impl", v.key_for_sorting().key_for_sorting())).or_default().push(item),
                Item::TypeAlias(ref v) => sorted_items.entry(format!("{}-alias", v.type_def().key_for_sorting())).or_default().push(item),
                _ => {},
            }
        }

        {
            let mut has_item = false;
            for key_vals in sorted_items.iter() {
                for item in key_vals.1.iter() {
                    match *item {
                        Item::Raw(_) => {}
                        _ => {
                            if has_item {
                                writeln!(fmt)?;
                            } else {
                                has_item = true;
                            }
                        },
                    }

                    match *item {
                        Item::Module(ref v) => v.fmt(fmt)?,
                        Item::Struct(ref v) => v.fmt(fmt)?,
                        Item::Function(ref v) => v.fmt(false, fmt)?,
                        Item::Trait(ref v) => v.fmt(fmt)?,
                        Item::Enum(ref v) => v.fmt(fmt)?,
                        Item::Impl(ref v) => v.fmt(fmt)?,
                        Item::TypeAlias(ref v) => v.fmt(fmt)?,
                        _ => {}, // already printed earlier
                    }
                    
                }
            }
        }

        Ok(())
    }

    fn fmt_imports(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        // First, collect all visibilities
        let mut visibilities = vec![];

        for (_, imports) in &self.imports {
            for (_, import) in imports {
                if !visibilities.contains(&import.vis) {
                    visibilities.push(import.vis.clone());
                }
            }
        }

        let mut alias_tys = vec![];
        let mut simple_tys = vec![];

        // Loop over all visibilities and format the associated imports
        for vis in &visibilities {
            for (path, imports) in &self.imports {
                alias_tys.clear();
                simple_tys.clear();

                for (ty, import) in imports {
                    if *vis == import.vis {
                        match import.alias.as_ref() {
                            None => { simple_tys.push(ty); }
                            Some(alias) => { alias_tys.push(format!("{} as {}", ty, alias)); }
                        }
                    }
                }

                for ty in alias_tys.iter() {
                    if let Some(ref vis) = *vis {
                        write!(fmt, "{} ", vis)?;
                    }

                    write!(fmt, "use {}::{};\n", path, ty)?;
                }
                if !simple_tys.is_empty() {
                    if let Some(ref vis) = *vis {
                        write!(fmt, "{} ", vis)?;
                    }

                    write!(fmt, "use {}::", path)?;

                    if simple_tys.len() > 1 {
                        write!(fmt, "{{")?;

                        for (i, ty) in simple_tys.iter().enumerate() {
                            if i != 0 {
                                write!(fmt, ", ")?;
                            }
                            write!(fmt, "{}", ty)?;
                        }

                        write!(fmt, "}};\n")?;
                    } else if simple_tys.len() == 1 {
                        write!(fmt, "{};\n", simple_tys[0])?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Merge two scopes together
    pub fn append(&mut self, other: &Self) -> &Self {
        self.docs = match (self.docs.as_ref(), other.docs.as_ref()) {
            (Some(doc_a), Some(doc_b)) => Some(Docs::new("").append(doc_a.to_str()).append(doc_b.to_str()).clone()),
            (Some(doc_a), None) => Some(doc_a.clone()),
            (None, Some(doc_b)) => Some(doc_b.clone()),
            (None, None) => None,
        };
        for (key, value) in other.imports.iter() {
            self.imports
                .entry(key.to_string())
                .or_insert(IndexMap::new())
                .extend(value.iter().map(|(a,b)| (a.clone(), b.clone())));
        }

        self.items.extend(other.items.iter().cloned());
        self
    }
}

