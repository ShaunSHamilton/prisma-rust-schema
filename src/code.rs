use crate::annotation::FieldAnnotation;
use crate::transform::{convert_field_to_type, to_snake_case};
use psl::schema_ast::ast::FieldId;
use psl::schema_ast::ast::{Field, WithDocumentation, WithName};
use quote::{ToTokens, format_ident, quote};
use syn::{Ident, Type, parse_str};

pub fn handle_fields((_field_id, field): (FieldId, &Field)) -> Option<proc_macro2::TokenStream> {
    // If field is a relation, skip
    if is_relation(&field) {
        return None;
    }

    let FieldAnnotation {
        skip,
        rename,
        visibility,
        type_,
    } = match field.documentation() {
        Some(d) => d.into(),
        None => FieldAnnotation::default(),
    };

    if skip {
        return None;
    }

    let name = match &rename {
        Some(name) => name,
        None => &to_snake_case(field.name()),
    };
    let name = format_ident!("{}", name);

    let serde_rename = if let Some(db_name) = &field.attributes.iter().find_map(|a| {
        if a.name() == "map" {
            let (val, _) = a.arguments.arguments[0].value.as_string_value().unwrap();
            Some(val)
        } else {
            None
        }
    }) {
        let s = quote! {
            #[serde(rename = #db_name)]
        };
        Some(s)
        // If field is renamed in Rust, the actual name should be used
    } else if let Some(_changed_name) = &rename {
        let original_name = field.name();
        let s = quote! {
            #[serde(rename = #original_name)]
        };
        Some(s)
    } else if name != &field.name() {
        let original_name = field.name();
        let s = quote! {
            #[serde(rename = #original_name)]
        };
        Some(s)
    } else {
        None
    };

    let type_name = match type_ {
        Some(type_) => {
            let ident = format_ident!("{}", type_);
            quote! { #ident }
        }
        None => {
            // Handle type conversions like `Int` to `i32`, and `field.native_type: ObjectId` to `bson::oid::ObjectId`
            let converted_type = convert_field_to_type(field);
            let t: Type = parse_str(&converted_type).expect("type to be parseable");

            quote! { #t }
        }
    };

    let documentation = extract_docs(field.documentation().clone());

    return Some(quote! {
        #documentation
        #serde_rename
        #visibility #name: #type_name,
    });
}

pub fn extract_docs(documentation: Option<&str>) -> impl ToTokens {
    if let Some(doc) = documentation {
        let docs = doc.lines().filter_map(|line| {
            let line = line.trim();
            if line.starts_with("@prs.") {
                return None;
            }
            Some(quote! {#[doc = #line]})
        });

        let doc = quote! {
            #(#docs)*
        };
        doc
    } else {
        // An empty ident
        let ident: Option<Ident> = None;
        quote! { #ident }
    }
}

pub fn handle_derive(derive: Option<Vec<String>>) -> impl ToTokens {
    if let Some(derive) = derive {
        let derive = derive
            .iter()
            .map(|d| {
                let derive_type: Type = parse_str(d.trim()).expect("derive input to be parseable");
                quote! { #derive_type }
            })
            .collect::<Vec<_>>();
        let derive = quote! {
            #[derive(#(#derive),*)]
        };
        derive
    } else {
        let ident: Option<Ident> = None;
        quote! { #ident }
    }
}

pub fn is_relation(field: &Field) -> bool {
    field.attributes.iter().any(|a| a.name() == "relation")
}
