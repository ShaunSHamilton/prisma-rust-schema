//! # Prisma Rust Schema
//!
//! A re-export of the Prisma TypeScript types in Rust.

use annotation::{
    EnumAnnotation, EnumValueAnnotation, FieldAnnotation, ModelAnnotation, TypeAnnotation,
};
use psl::{
    parse_schema,
    schema_ast::ast::{Field, Top, WithDocumentation, WithName},
};
use quote::{ToTokens, format_ident, quote};
use syn::{Ident, LitStr, Type, parse_str};
use transform::{convert_field_to_type, to_pascal_case, to_snake_case};

mod annotation;
mod transform;

#[proc_macro]
pub fn import_types(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match handle_import(item) {
        Ok(o) => o,
        Err(e) => e.to_compile_error().into(),
    }
}

fn handle_import(item: proc_macro::TokenStream) -> syn::Result<proc_macro::TokenStream> {
    // Parse the input as a string literal
    let schema_path = syn::parse::<LitStr>(item)?;

    let dir = std::env::var("CARGO_MANIFEST_DIR").map_or_else(
        |_| std::env::current_dir().unwrap(),
        |s| std::path::Path::new(&s).to_path_buf(),
    );

    let path = dir.join(schema_path.value());
    if !path.exists() {
        return Err(syn::Error::new_spanned(
            &schema_path,
            format!("Schema file not found: {}", path.display()),
        ));
    }

    let schema = std::fs::read_to_string(&path)
        .map_err(|e| syn::Error::new_spanned(&schema_path, e.to_string()))?;

    let validated_schema =
        parse_schema(schema).map_err(|e| syn::Error::new_spanned(&schema_path, e.to_string()))?;

    let db = validated_schema.db;

    assert_eq!(
        db.files_count(),
        1,
        "This macro only supports single-file schemas"
    );

    let ast = db.into_iter_asts().next().expect("Expected a single AST");
    let tops = ast.tops;

    // let mut output_token_stream = TokenStream::new();
    let mut output_tokens = quote! {};

    for top in tops {
        match top {
            Top::CompositeType(composite_type) => {
                let TypeAnnotation {
                    skip,
                    rename,
                    visibility,
                    derive,
                    type_,
                } = match composite_type.documentation() {
                    Some(d) => d.into(),
                    None => TypeAnnotation::default(),
                };
                if skip {
                    continue;
                }

                if type_.is_some() {
                    eprintln!(
                        "`@prs.type` annotation is not supported for type name declarations."
                    );
                    eprintln!("Please use `@prs.rename` instead.");
                    eprintln!(
                        "Skipping type name declaration for `{}`",
                        composite_type.name()
                    );
                    continue;
                }

                let name: String = match rename {
                    Some(name) => name,
                    None => to_pascal_case(&composite_type.name()),
                };
                let struct_name = format_ident!("{}", name);
                let documentation = extract_docs(composite_type.documentation().clone());
                let fields = composite_type
                    .iter_fields()
                    .filter_map(|(_field_id, field)| {
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

                        let serde_rename = if let Some(db_name) =
                            &field.attributes.iter().find_map(|a| {
                                if a.name() == "map" {
                                    let (val, _) =
                                        a.arguments.arguments[0].value.as_string_value().unwrap();
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
                                let t: Type =
                                    parse_str(&converted_type).expect("type to be parseable");

                                quote! { #t }
                            }
                        };

                        let documentation = extract_docs(field.documentation().clone());

                        return Some(quote! {
                            #documentation
                            #serde_rename
                            #visibility #name: #type_name,
                        });
                    });

                let derive = handle_derive(derive);

                let s = quote! {
                    #documentation
                    #derive
                    #visibility struct #struct_name {
                        #(#fields)*
                    }
                };
                output_tokens.extend(s);
            }
            Top::Enum(enum_type) => {
                let EnumAnnotation {
                    skip,
                    rename,
                    visibility,
                    derive,
                } = match enum_type.documentation() {
                    Some(d) => d.into(),
                    None => EnumAnnotation::default(),
                };
                if skip {
                    continue;
                }

                let name = match rename {
                    Some(name) => name,
                    None => enum_type.name().to_string(),
                };
                let enum_name = format_ident!("{}", name);
                let documentation = extract_docs(enum_type.documentation().clone());
                let enum_values = enum_type.values.iter().filter_map(|enum_value| {
                    let EnumValueAnnotation { skip, rename } = match enum_value.documentation() {
                        Some(d) => d.into(),
                        None => EnumValueAnnotation::default(),
                    };

                    if skip {
                        return None;
                    }

                    let serde_rename = if let Some(_changed_name) = &rename {
                        let original_name = enum_value.name();
                        let s = quote! {
                            #[serde(rename = #original_name)]
                        };
                        Some(s)
                    } else {
                        None
                    };

                    let name = match rename {
                        Some(name) => name,
                        None => enum_value.name().to_string(),
                    };
                    let name = format_ident!("{}", name);

                    let documentation = extract_docs(enum_value.documentation().clone());

                    return Some(quote! {
                        #documentation
                        #serde_rename
                        #name,
                    });
                });

                let derive = handle_derive(derive);

                let s = quote! {
                    #documentation
                    #derive
                    #visibility enum #enum_name {
                        #(#enum_values)*
                    }
                };
                output_tokens.extend(s);
            }
            Top::Model(model) => {
                let ModelAnnotation {
                    skip,
                    rename,
                    visibility,
                    derive,
                } = match model.documentation() {
                    Some(d) => d.into(),
                    None => ModelAnnotation::default(),
                };
                if skip {
                    continue;
                }

                let name = match rename {
                    Some(name) => name,
                    None => to_pascal_case(&model.name()),
                };
                let struct_name = format_ident!("{}", name);
                let documentation = extract_docs(model.documentation().clone());
                let fields = model.iter_fields().filter_map(|(_field_id, field)| {
                    // If field is a relation, skip
                    if is_relation(&field) {
                        return None;
                    }

                    let FieldAnnotation {
                        rename,
                        skip,
                        type_,
                        visibility,
                    } = match field.documentation() {
                        Some(d) => d.into(),
                        None => FieldAnnotation::default(),
                    };

                    if skip {
                        return None;
                    }

                    let name = match &rename {
                        Some(name) => name,
                        None => &to_snake_case(&field.name()),
                    };

                    // let serde_rename: Option<TokenStream> = None;
                    let serde_rename = if let Some(db_name) =
                        &field.attributes.iter().find_map(|a| {
                            if a.name() == "map" {
                                let (val, _) =
                                    a.arguments.arguments[0].value.as_string_value().unwrap();
                                Some(val)
                            } else {
                                None
                            }
                        }) {
                        let s = quote! {
                            #[serde(rename = #db_name)]
                        };
                        Some(s)
                    } else if let Some(_changed_name) = &rename {
                        // If field is renamed in Rust, the actual name should be used
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

                    let name = format_ident!("{}", name);

                    let type_name = match type_ {
                        Some(type_) => {
                            let ident = format_ident!("{}", type_);
                            quote! { #ident }
                        }
                        None => {
                            // Handle type conversions like `Int` to `i32`, and `field.native_type: ObjectId` to `bson::oid::ObjectId`
                            let converted_type = convert_field_to_type(&field);
                            let t: Type = parse_str(&converted_type).expect("type to be parseable");

                            quote! { #t }
                        }
                    };

                    let documentation = extract_docs(field.documentation());

                    return Some(quote! {
                        #documentation
                        #serde_rename
                        #visibility #name: #type_name,
                    });
                });

                let derive = handle_derive(derive);

                let s = quote! {
                    #documentation
                    #derive
                    #visibility struct #struct_name {
                        #(#fields)*
                    }
                };
                output_tokens.extend(s);
            }
            _ => {
                // Skip
                continue;
            }
        }
    }

    Ok(output_tokens.into())
}

fn extract_docs(documentation: Option<&str>) -> impl ToTokens {
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

fn handle_derive(derive: Option<Vec<String>>) -> impl ToTokens {
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

fn is_relation(field: &Field) -> bool {
    field.attributes.iter().any(|a| a.name() == "relation")
}
