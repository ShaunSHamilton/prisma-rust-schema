use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::ToTokens;

#[derive(Debug, Default)]
pub struct FieldAnnotation {
    /// What to rename the field to
    pub rename: Option<String>,
    /// Whether to skip this field
    pub skip: bool,
    /// The Rust type of the field to overwrite to
    pub type_: Option<String>,
    /// The visibility of the field
    pub visibility: Visibility,
}

#[derive(Debug, Default)]
pub struct ModelAnnotation {
    pub derive: Option<Vec<String>>,
    pub rename: Option<String>,
    pub skip: bool,
    pub visibility: Visibility,
}

#[derive(Debug, Default)]
pub struct EnumAnnotation {
    pub derive: Option<Vec<String>>,
    pub rename: Option<String>,
    pub skip: bool,
    pub visibility: Visibility,
}

#[derive(Debug, Default)]
pub struct EnumValueAnnotation {
    pub rename: Option<String>,
    pub skip: bool,
}

#[derive(Debug, Default)]
pub struct TypeAnnotation {
    pub derive: Option<Vec<String>>,
    pub rename: Option<String>,
    pub skip: bool,
    /// The Rust type of the field to overwrite to
    pub type_: Option<String>,
    pub visibility: Visibility,
}

#[derive(Debug, Clone, Copy)]
pub enum Visibility {
    /// Visible to all
    /// i.e. `pub`
    Public,
    /// Not visible
    Private,
    /// Visible to the current module and its descendants
    /// (i.e. `pub(crate)`)
    Protected,
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Public
    }
}

// TODO: Find way to not need `proc_macro2::TokenStream` for this. This is the only reason for the `proc_macro2` dependency.
impl ToTokens for Visibility {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Visibility::Public => quote::quote! { pub }.to_tokens(tokens),
            Visibility::Private => quote::quote! {}.to_tokens(tokens),
            Visibility::Protected => quote::quote! { pub(crate) }.to_tokens(tokens),
        }
    }
}

impl FromStr for FieldAnnotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut skip = false;
        let mut rename = None;
        let mut visibility = Visibility::default();
        let mut type_ = None;

        for line in s.lines() {
            if line.trim().starts_with("@prs.") {
                let func = line.trim_start_matches("@prs.");
                let (op, val) = func.split_once('=').unwrap_or((func, "true"));

                match op.trim() {
                    "skip" => {
                        skip = val.trim() != "false";
                    }
                    "type" => {
                        type_ = Some(val.trim().to_string());
                    }
                    "rename" => {
                        rename = Some(val.trim().to_string());
                    }
                    "visibility" => {
                        visibility = match val.trim() {
                            "private" => Visibility::Private,
                            "protected" => Visibility::Protected,
                            "public" => Visibility::Public,
                            _ => {
                                return Err(format!(
                                    "Unknown visibility: {}
Available options: private, protected (pub(crate)), public (pub)",
                                    val
                                ));
                            }
                        };
                    }
                    _ => {
                        return Err(format!("Unknown field annotation: {}", func));
                    }
                }
            }
        }

        Ok(FieldAnnotation {
            rename,
            skip,
            type_,
            visibility,
        })
    }
}

impl From<String> for FieldAnnotation {
    fn from(s: String) -> Self {
        FieldAnnotation::from_str(&s).unwrap_or_default()
    }
}
impl From<&String> for FieldAnnotation {
    fn from(s: &String) -> Self {
        FieldAnnotation::from_str(s).unwrap_or_default()
    }
}
impl From<&str> for FieldAnnotation {
    fn from(s: &str) -> Self {
        FieldAnnotation::from_str(s).unwrap_or_default()
    }
}

impl FromStr for ModelAnnotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut skip = false;
        let mut rename = None;
        let mut visibility = Visibility::default();
        let mut derive = None;

        for line in s.lines() {
            if line.trim().starts_with("@prs.") {
                let func = line.trim_start_matches("@prs.");
                let (op, val) = func.split_once('=').unwrap_or((func, "true"));

                match op.trim() {
                    "skip" => {
                        skip = val.trim() != "false";
                    }
                    "rename" => {
                        rename = Some(val.trim().to_string());
                    }
                    "visibility" => {
                        visibility = match val.trim() {
                            "private" => Visibility::Private,
                            "protected" => Visibility::Protected,
                            "public" => Visibility::Public,
                            _ => {
                                return Err(format!(
                                    "Unknown visibility: {}
Available options: private, protected (pub(crate)), public (pub)",
                                    val
                                ));
                            }
                        };
                    }
                    "derive" => {
                        let derive_str = val.trim();
                        if derive_str.is_empty() {
                            return Err("Derive cannot be empty".to_string());
                        }
                        derive = Some(
                            derive_str
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .collect(),
                        );
                    }
                    _ => {
                        return Err(format!("Unknown field annotation: {}", func));
                    }
                }
            }
        }

        Ok(ModelAnnotation {
            skip,
            rename,
            visibility,
            derive,
        })
    }
}

impl From<String> for ModelAnnotation {
    fn from(s: String) -> Self {
        ModelAnnotation::from_str(&s).unwrap_or_default()
    }
}
impl From<&String> for ModelAnnotation {
    fn from(s: &String) -> Self {
        ModelAnnotation::from_str(s).unwrap_or_default()
    }
}
impl From<&str> for ModelAnnotation {
    fn from(s: &str) -> Self {
        ModelAnnotation::from_str(s).unwrap_or_default()
    }
}

impl FromStr for EnumAnnotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut skip = false;
        let mut rename = None;
        let mut visibility = Visibility::default();
        let mut derive = None;

        for line in s.lines() {
            if line.trim().starts_with("@prs.") {
                let func = line.trim_start_matches("@prs.");
                let (op, val) = func.split_once('=').unwrap_or((func, "true"));

                match op.trim() {
                    "skip" => {
                        skip = val.trim() != "false";
                    }
                    "rename" => {
                        rename = Some(val.trim().to_string());
                    }
                    "visibility" => {
                        visibility = match val.trim() {
                            "private" => Visibility::Private,
                            "protected" => Visibility::Protected,
                            "public" => Visibility::Public,
                            _ => {
                                return Err(format!(
                                    "Unknown visibility: {}
Available options: private, protected (pub(crate)), public (pub)",
                                    val
                                ));
                            }
                        };
                    }
                    "derive" => {
                        let derive_str = val.trim();
                        if derive_str.is_empty() {
                            return Err("Derive cannot be empty".to_string());
                        }
                        derive = Some(
                            derive_str
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .collect(),
                        );
                    }
                    _ => {
                        return Err(format!("Unknown field annotation: {}", func));
                    }
                }
            }
        }

        Ok(EnumAnnotation {
            skip,
            rename,
            visibility,
            derive,
        })
    }
}

impl From<String> for EnumAnnotation {
    fn from(s: String) -> Self {
        EnumAnnotation::from_str(&s).unwrap_or_default()
    }
}
impl From<&String> for EnumAnnotation {
    fn from(s: &String) -> Self {
        EnumAnnotation::from_str(s).unwrap_or_default()
    }
}
impl From<&str> for EnumAnnotation {
    fn from(s: &str) -> Self {
        EnumAnnotation::from_str(s).unwrap_or_default()
    }
}

impl FromStr for TypeAnnotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut skip = false;
        let mut rename = None;
        let mut visibility = Visibility::default();
        let mut derive = None;
        let mut type_ = None;

        for line in s.lines() {
            if line.trim().starts_with("@prs.") {
                let func = line.trim_start_matches("@prs.");
                let (op, val) = func.split_once('=').unwrap_or((func, "true"));

                match op.trim() {
                    "skip" => {
                        skip = val.trim() != "false";
                    }
                    "type" => {
                        type_ = Some(val.trim().to_string());
                    }
                    "rename" => {
                        rename = Some(val.trim().to_string());
                    }
                    "visibility" => {
                        visibility = match val.trim() {
                            "private" => Visibility::Private,
                            "protected" => Visibility::Protected,
                            "public" => Visibility::Public,
                            _ => {
                                return Err(format!(
                                    "Unknown visibility: {}
Available options: private, protected (pub(crate)), public (pub)",
                                    val
                                ));
                            }
                        };
                    }
                    "derive" => {
                        let derive_str = val.trim();
                        if derive_str.is_empty() {
                            return Err("Derive cannot be empty".to_string());
                        }
                        derive = Some(
                            derive_str
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .collect(),
                        );
                    }
                    _ => {
                        return Err(format!("Unknown field annotation: {}", func));
                    }
                }
            }
        }

        Ok(TypeAnnotation {
            derive,
            rename,
            skip,
            type_,
            visibility,
        })
    }
}

impl From<String> for TypeAnnotation {
    fn from(s: String) -> Self {
        TypeAnnotation::from_str(&s).unwrap_or_default()
    }
}
impl From<&String> for TypeAnnotation {
    fn from(s: &String) -> Self {
        TypeAnnotation::from_str(s).unwrap_or_default()
    }
}
impl From<&str> for TypeAnnotation {
    fn from(s: &str) -> Self {
        TypeAnnotation::from_str(s).unwrap_or_default()
    }
}

impl FromStr for EnumValueAnnotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut skip = false;
        let mut rename = None;

        for line in s.lines() {
            if line.trim().starts_with("@prs.") {
                let func = line.trim_start_matches("@prs.");
                let (op, val) = func.split_once('=').unwrap_or((func, "true"));

                match op.trim() {
                    "skip" => {
                        skip = val.trim() != "false";
                    }
                    "rename" => {
                        rename = Some(val.trim().to_string());
                    }
                    _ => {
                        return Err(format!("Unknown field annotation: {}", func));
                    }
                }
            }
        }

        Ok(EnumValueAnnotation { skip, rename })
    }
}

impl From<String> for EnumValueAnnotation {
    fn from(s: String) -> Self {
        EnumValueAnnotation::from_str(&s).unwrap_or_default()
    }
}
impl From<&String> for EnumValueAnnotation {
    fn from(s: &String) -> Self {
        EnumValueAnnotation::from_str(s).unwrap_or_default()
    }
}
impl From<&str> for EnumValueAnnotation {
    fn from(s: &str) -> Self {
        EnumValueAnnotation::from_str(s).unwrap_or_default()
    }
}
