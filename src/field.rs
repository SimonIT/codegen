use crate::r#type::Type;

/// Defines a struct field.
#[derive(Debug, Clone)]
pub struct Field {
    /// Field name
    pub name: String,

    /// Field type
    pub ty: Type,

    /// Field documentation
    pub documentation: String,

    /// Field annotation
    pub annotation: Vec<String>,

    /// Field value
    pub value: String,

    /// The visibility of the field
    pub visibility: Option<String>,
}

impl Field {
    /// Return a field definition with the provided name and type
    pub fn new<T>(name: &str, ty: T) -> Self
    where
        T: Into<Type>,
    {
        Field {
            name: name.into(),
            ty: ty.into(),
            documentation: String::new(),
            annotation: Vec::new(),
            value: String::new(),
            visibility: None,
        }
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

    /// Set the visibility of the field
    pub fn vis(&mut self, visibility: impl Into<String>) -> &mut Self {
        self.visibility = Some(visibility.into());
        self
    }
}
