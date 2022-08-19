use crate::r#type::Type;

/// Defines a struct field.
#[derive(Debug, Clone)]
pub struct Field {
    /// Field name
    pub name: String,

    /// The field's visibility
    pub vis: Option<String>,

    /// Field type
    pub ty: Type,

    /// Field documentation
    pub documentation: String,

    /// Field annotation
    pub annotation: Vec<String>,
}

impl Field {
    /// Return a field definition with the provided name and type
    pub fn new<T>(name: &str, ty: T) -> Self
    where
        T: Into<Type>,
    {
        Field {
            name: name.into(),
            vis: None,
            ty: ty.into(),
            documentation: String::new(),
            annotation: Vec::new(),
        }
    }

    /// Set the field's visibility
    pub fn vis(&mut self, vis: impl Into<String>) -> &mut Self {
        self.vis = Some(vis.into());
        self
    }

    /// Set field's documentation.
    pub fn doc(&mut self, documentation: impl Into<String>) -> &mut Self {
        self.documentation = documentation.into();
        self
    }

    /// Set field's annotation.
    pub fn annotation(&mut self, annotation: impl Into<String>) -> &mut Self {
        self.annotation.push(annotation.into());
        self
    }
}
