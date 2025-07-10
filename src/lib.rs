//! # Prisma Rust Schema
//!
//! A re-export of the Prisma TypeScript types in Rust.

use annotation::{EnumAnnotation, EnumValueAnnotation, ModelAnnotation, TypeAnnotation};
use code::{extract_docs, handle_derive, handle_fields};
use psl::{
    parse_schema,
    schema_ast::ast::{Top, WithDocumentation, WithName},
};
use quote::{ToTokens, format_ident, quote};
use serde::Deserialize;
use serde_tokenstream::{ParseWrapper, from_tokenstream};
use syn::{ItemStruct, LitStr};
use transform::to_pascal_case;

mod annotation;
mod code;
mod transform;

#[proc_macro]
pub fn import_types(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match handle_import(item) {
        Ok(o) => o,
        Err(e) => e.to_compile_error().into(),
    }
}

#[derive(Deserialize, Debug)]
struct ImportOptions {
    schema_path: String,
    derive: Option<Vec<ParseWrapper<syn::Path>>>,
    include: Option<Vec<String>>,
    prefix: Option<String>,
    patch: Option<Vec<ParseWrapper<ItemStruct>>>,
}

fn handle_import(item: proc_macro::TokenStream) -> syn::Result<proc_macro::TokenStream> {
    // Parse the input as a string literal `import_types("path.prisma")` or `import_types({ derive: [Path], include: [String]`
    let import_options: ImportOptions =
        from_tokenstream(&proc_macro2::TokenStream::from(item.clone())).unwrap_or_else(|_e| {
            let schema_path = syn::parse::<LitStr>(item)
                .expect("schema path to be provided")
                .value();
            ImportOptions {
                schema_path,
                // TODO: Consider defaulting to SERDE
                derive: None,
                include: None,
                prefix: None,
                patch: None,
            }
        });

    let dir = std::env::var("CARGO_MANIFEST_DIR").map_or_else(
        |_| std::env::current_dir().expect("current dir to be determined"),
        |s| std::path::Path::new(&s).to_path_buf(),
    );

    // println!("{:#?}", import_options.patch);

    let schema_path = import_options.schema_path.clone();

    let path = dir.join(&schema_path);
    if !path.exists() {
        return Err(syn::Error::new_spanned(
            &schema_path,
            format!("Schema file not found: {}", path.display()),
        ));
    }

    let schema = std::fs::read_to_string(&path)
        .map_err(|e| syn::Error::new_spanned(&schema_path, e.to_string()))?;

    let validated_schema =
        parse_schema(&schema).map_err(|e| syn::Error::new_spanned(&schema_path, e.to_string()))?;

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

    for top in &tops {
        match top {
            Top::CompositeType(composite_type) => {
                let name = composite_type.name();
                if import_options.include.is_some() {
                    let include = import_options.include.as_ref().expect("UNREACHABLE");
                    if !include.contains(&name.to_string()) {
                        continue;
                    }
                }

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

                let derive = derive.or(import_options.derive.as_ref().map(|d| {
                    d.into_iter()
                        .map(|i| i.to_token_stream().to_string())
                        .collect()
                }));

                if type_.is_some() {
                    eprintln!(
                        "`@prs.type` annotation is not supported for type name declarations."
                    );
                    eprintln!("Please use `@prs.rename` instead.");
                    eprintln!("Skipping type name declaration for `{}`", name);
                    continue;
                }

                let name = match rename {
                    Some(name) => name,
                    None => to_pascal_case(name),
                };
                let name = if let Some(prefix) = &import_options.prefix {
                    format!("{}{}", prefix, name)
                } else {
                    name.to_string()
                };
                let struct_name = format_ident!("{}", name);

                let documentation = extract_docs(composite_type.documentation().clone());
                let fields = composite_type
                    .iter_fields()
                    .filter_map(|(_field_id, field)| handle_fields(&tops, &import_options, field));

                let derive = handle_derive(derive);

                let s = quote! {
                    #documentation
                    #derive
                    #visibility struct #struct_name {
                        #(#fields)*
                    }
                };

                let s = if let Some(Some(patch)) = import_options
                    .patch
                    .as_ref()
                    .map(|patches| patches.iter().find(|p| p.ident == struct_name))
                {
                    // Keep non-skipped fields, overwrite explicitly mentioned fields
                    let item_struct = syn::parse2::<syn::ItemStruct>(s).expect("item struct");
                    let patch_fields = &patch.fields;

                    let fields = item_struct.fields.iter().map(|field| {
                        if let Some(f) = patch_fields.iter().find(|f| f.ident == field.ident) {
                            let f = quote! {
                                #f,
                            };
                            f
                        } else {
                            let f = quote! {
                                #field,
                            };
                            f
                        }
                    });

                    quote! {
                        #documentation
                        #derive
                        #visibility struct #struct_name {
                            #(#fields)*
                        }
                    }
                } else {
                    s
                };

                output_tokens.extend(s);
            }
            Top::Enum(enum_type) => {
                let name = enum_type.name();
                if import_options.include.is_some() {
                    let include = import_options.include.as_ref().expect("UNREACHABLE");
                    if !include.contains(&name.to_string()) {
                        continue;
                    }
                }

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

                let derive = derive.or(import_options.derive.as_ref().map(|d| {
                    d.into_iter()
                        .map(|i| i.to_token_stream().to_string())
                        .collect()
                }));

                let name = match rename {
                    Some(name) => name,
                    None => name.to_string(),
                };
                let name = if let Some(prefix) = &import_options.prefix {
                    format!("{}{}", prefix, name)
                } else {
                    name.to_string()
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
                let name = model.name();
                if import_options.include.is_some() {
                    let include = import_options.include.as_ref().expect("UNREACHABLE");
                    if !include.contains(&name.to_string()) {
                        continue;
                    }
                }

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

                let derive = derive.or(import_options.derive.as_ref().map(|d| {
                    d.into_iter()
                        .map(|i| i.to_token_stream().to_string())
                        .collect()
                }));

                let name = match rename {
                    Some(name) => name,
                    None => to_pascal_case(name),
                };
                let name = if let Some(prefix) = &import_options.prefix {
                    format!("{}{}", prefix, name)
                } else {
                    name.to_string()
                };
                let struct_name = format_ident!("{}", name);
                let documentation = extract_docs(model.documentation().clone());
                let fields = model
                    .iter_fields()
                    .filter_map(|(_field_id, field)| handle_fields(&tops, &import_options, field));
                let derive = handle_derive(derive);

                let s = quote! {
                    #documentation
                    #derive
                    #visibility struct #struct_name {
                        #(#fields)*
                    }
                };

                let s = if let Some(Some(patch)) = import_options
                    .patch
                    .as_ref()
                    .map(|patches| patches.iter().find(|p| p.ident == struct_name))
                {
                    // Keep non-skipped fields, overwrite explicitly mentioned fields
                    let item_struct = syn::parse2::<syn::ItemStruct>(s).expect("item struct");
                    let patch_fields = &patch.fields;

                    let fields = item_struct.fields.iter().map(|field| {
                        if let Some(f) = patch_fields.iter().find(|f| f.ident == field.ident) {
                            let f = quote! {
                                #f,
                            };
                            f
                        } else {
                            let f = quote! {
                                #field,
                            };
                            f
                        }
                    });

                    quote! {
                        #documentation
                        #derive
                        #visibility struct #struct_name {
                            #(#fields)*
                        }
                    }
                } else {
                    s
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
