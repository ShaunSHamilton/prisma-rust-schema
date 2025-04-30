//! # Prisma Rust Schema
//!
//! This CLI tool generates Rust types from a Prisma schema file.

use std::io::{BufRead, BufReader, Write, stderr, stdin};

use prisma_rust_schema::{
    annotation::{
        EnumAnnotation, EnumValueAnnotation, FieldAnnotation, ModelAnnotation, TypeAnnotation,
    },
    config::GeneratorOptions,
    jsonrpc,
    transform::{convert_field_to_type, to_pascal_case, to_snake_case},
};
use quote::{ToTokens, format_ident, quote};
use syn::{Ident, Type, parse_str};

fn main() {
    loop {
        let mut content = String::new();
        BufReader::new(stdin())
            .read_line(&mut content)
            .expect("stdin to be written to buffer");

        let input: jsonrpc::Request = serde_json::from_str(&content)
            .expect("stdin from prisma to be serializable into jsonrpc");

        let version = env!("CARGO_PKG_VERSION").to_string();

        let data = match input.method.as_str() {
            "getManifest" => jsonrpc::ResponseData::Result(
                serde_json::to_value(jsonrpc::ManifestResponse {
                    manifest: jsonrpc::Manifest {
                        default_output: "../src/prisma-rust-schema.rs".to_string(),
                        pretty_name: "Prisma Rust Schema".to_string(),
                        version: Some(version),
                        ..Default::default()
                    },
                })
                .expect("manifest to be serializable into json"),
            ),
            "generate" => {
                let params_str = input.params.to_string();

                let deserializer = &mut serde_json::Deserializer::from_str(&params_str);
                let generator_options: GeneratorOptions =
                    serde_path_to_error::deserialize(deserializer)
                        .map_err(|e| format!("{}\n{}", e, e.path()))
                        .expect("generator options to be deserializable from prisma");
                generate_rust_types(&generator_options);
                jsonrpc::ResponseData::Result(serde_json::Value::Null)
            }
            method => jsonrpc::ResponseData::Error {
                code: 0,
                message: format!("Cannot handle method {}", method),
            },
        };

        let response = jsonrpc::Response {
            jsonrpc: "2.0".to_string(),
            id: input.id,
            data,
        };

        let mut bytes =
            serde_json::to_vec(&response).expect("response to be serializable into json");

        bytes.push(b'\n');

        stderr()
            .by_ref()
            .write_all(bytes.as_ref())
            .expect("response to be written to stderr");

        if input.method.as_str() == "generate" {
            break;
        }
    }
}

fn generate_rust_types(generator_options: &GeneratorOptions) {
    let datamodel = &generator_options.dmmf.datamodel;

    let mut output_tokens = quote! {};

    for model in datamodel.models.iter() {
        let ModelAnnotation {
            skip,
            rename,
            visibility,
            derive,
        } = match &model.documentation {
            Some(d) => d.into(),
            None => ModelAnnotation::default(),
        };
        if skip {
            continue;
        }

        let name = match rename {
            Some(name) => name,
            None => to_pascal_case(&model.name),
        };
        let struct_name = format_ident!("{}", name);
        let documentation = extract_docs(model.documentation.clone());
        let fields = model.fields.iter().filter_map(|field| {
            // If field is a relation, skip
            if let Some(relation) = &field.relation_from_fields {
                if !relation.is_empty() {
                    return None;
                }
            }

            let FieldAnnotation {
                rename,
                skip,
                type_,
                visibility,
            } = match &field.documentation {
                Some(d) => d.into(),
                None => FieldAnnotation::default(),
            };

            if skip {
                return None;
            }

            let name = match &rename {
                Some(name) => name,
                None => &to_snake_case(&field.name),
            };

            let serde_rename = if let Some(db_name) = &field.db_name {
                let s = quote! {
                    #[serde(rename = #db_name)]
                };
                Some(s)
            } else if let Some(_changed_name) = &rename {
                // If field is renamed in Rust, the actual name should be used
                let original_name = field.name.clone();
                let s = quote! {
                    #[serde(rename = #original_name)]
                };
                Some(s)
            } else if name != &field.name {
                let original_name = field.name.clone();
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

            let documentation = extract_docs(field.documentation.clone());

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
    for model in datamodel.type_models.iter() {
        let TypeAnnotation {
            skip,
            rename,
            visibility,
            derive,
            type_,
        } = match &model.documentation {
            Some(d) => d.into(),
            None => TypeAnnotation::default(),
        };
        if skip {
            continue;
        }

        if type_.is_some() {
            eprintln!("`@prs.type` annotation is not supported for type name declarations.");
            eprintln!("Please use `@prs.rename` instead.");
            eprintln!("Skipping type name declaration for `{}`", model.name);
            continue;
        }

        let name: String = match rename {
            Some(name) => name,
            None => to_pascal_case(&model.name),
        };
        let struct_name = format_ident!("{}", name);
        let documentation = extract_docs(model.documentation.clone());
        let fields = model.fields.iter().filter_map(|field| {
            // If field is a relation, skip
            if let Some(relation) = &field.relation_from_fields {
                if !relation.is_empty() {
                    return None;
                }
            }

            let FieldAnnotation {
                skip,
                rename,
                visibility,
                type_,
            } = match &field.documentation {
                Some(d) => d.into(),
                None => FieldAnnotation::default(),
            };

            if skip {
                return None;
            }

            let name = match &rename {
                Some(name) => name,
                None => &to_snake_case(&field.name),
            };
            let name = format_ident!("{}", name);

            let serde_rename = if let Some(db_name) = &field.db_name {
                let s = quote! {
                    #[serde(rename = #db_name)]
                };
                Some(s)
                // If field is renamed in Rust, the actual name should be used
            } else if let Some(_changed_name) = &rename {
                let original_name = field.name.clone();
                let s = quote! {
                    #[serde(rename = #original_name)]
                };
                Some(s)
            } else if name != &field.name {
                let original_name = field.name.clone();
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
                    let converted_type = convert_field_to_type(&field);
                    let t: Type = parse_str(&converted_type).expect("type to be parseable");

                    quote! { #t }
                }
            };

            let documentation = extract_docs(field.documentation.clone());

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

    for enu in datamodel.enums.iter() {
        let EnumAnnotation {
            skip,
            rename,
            visibility,
            derive,
        } = match &enu.documentation {
            Some(d) => d.into(),
            None => EnumAnnotation::default(),
        };
        if skip {
            continue;
        }

        let name = match rename {
            Some(name) => name,
            None => enu.name.clone(),
        };
        let enum_name = format_ident!("{}", name);
        let documentation = extract_docs(enu.documentation.clone());
        let enum_values = enu.values.iter().filter_map(|enum_value| {
            let EnumValueAnnotation { skip, rename } = match &enum_value.documentation {
                Some(d) => d.into(),
                None => EnumValueAnnotation::default(),
            };

            if skip {
                return None;
            }

            let serde_rename = if let Some(db_name) = &enum_value.db_name {
                let s = quote! {
                    #[serde(rename = #db_name)]
                };
                Some(s)
                // If field is renamed in Rust, the actual name should be used
            } else if let Some(_changed_name) = &rename {
                let original_name = enum_value.name.clone();
                let s = quote! {
                    #[serde(rename = #original_name)]
                };
                Some(s)
            } else {
                None
            };

            let name = match rename {
                Some(name) => name,
                None => enum_value.name.clone(),
            };
            let name = format_ident!("{}", name);

            let documentation = extract_docs(enum_value.documentation.clone());

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

    let output_path = generator_options
        .generator
        .output
        .clone()
        .unwrap()
        .value
        .unwrap_or("./src/prisma_types.rs".to_string());
    // Create output file and folders if they do not exist. Overwrite file.
    std::fs::create_dir_all(std::path::Path::new(&output_path).parent().unwrap())
        .expect("output directory to be created");
    let mut file = std::fs::File::create(output_path).expect("output file to be created");
    let output_str = format!("{}", output_tokens);
    file.write_all(output_str.as_bytes())
        .expect("output file to be written");
}

fn extract_docs(documentation: Option<String>) -> impl ToTokens {
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
