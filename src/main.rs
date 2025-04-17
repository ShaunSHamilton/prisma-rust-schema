//! # Prisma Rust Schema
//!
//! This CLI tool generates Rust types from a Prisma schema file.
//!
//!

use std::io::{BufRead, BufReader, Write, stderr, stdin};

mod config;
mod dmmf;
mod jsonrpc;
mod typed_sql;

use config::GeneratorOptions;

fn main() {
    loop {
        let mut content = String::new();
        BufReader::new(stdin())
            .read_line(&mut content)
            .expect("stdin to be written to buffer");

        let input: jsonrpc::Request = serde_json::from_str(&content)
            .expect("stdin from prisma to be serializable into jsonrpc");

        let data = match input.method.as_str() {
            "getManifest" => jsonrpc::ResponseData::Result(
                serde_json::to_value(jsonrpc::ManifestResponse {
                    manifest: jsonrpc::Manifest {
                        default_output: "../src/typ.rs".to_string(),
                        pretty_name: "Prisma Rust Schema".to_string(),
                        version: Some("0.1.0".to_string()),
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
    let mut output_file = String::new();
    output_file.push_str("use serde::{Deserialize, Serialize};\n");

    for model in datamodel.models.iter() {
        let struct_name = model.name.clone();
        let fields = model.fields.iter().filter_map(|field| {
            // If relation from fields exists and is non-empty, skip
            if let Some(relation) = &field.relation_from_fields {
                if !relation.is_empty() {
                    return None;
                }
            }
            Some(String::from(field))
        });
        let s = format!(
            "
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = \"camelCase\")]
pub struct {} {{
{}
}}
",
            struct_name,
            fields.collect::<String>()
        );
        output_file.push_str(&s);
    }

    for enum_ in datamodel.enums.iter() {
        let enum_name = enum_.name.clone();
        let variants = enum_.values.iter().map(|variant| {
            let variant_name = variant.name.clone();
            format!("{:>4}{},\n", "", variant_name)
        });
        let s = format!(
            "
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = \"camelCase\")]
pub enum {} {{
{}
}}
",
            enum_name,
            variants.collect::<String>()
        );
        output_file.push_str(&s);
    }

    for type_ in datamodel.type_models.iter() {
        let type_name = type_.name.clone();
        let fields = type_.fields.iter().map(String::from);
        let s = format!(
            "
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = \"camelCase\")]
pub struct {} {{
{}
}}
",
            type_name,
            fields.collect::<String>()
        );
        output_file.push_str(&s);
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
    file.write_all(output_file.as_bytes())
        .expect("output file to be written");
}
