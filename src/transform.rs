use psl::schema_ast::ast::{Field, WithName};

use crate::ImportOptions;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Case {
    Snake,
    Camel,
    Pascal,
    Kebab,
    ScreamingSnake,
    Unknown,
}

pub(crate) fn identify_case(input: &str) -> Case {
    if input.is_empty() {
        return Case::Snake;
    }

    let chars = input.chars().collect::<Vec<_>>();

    if chars.iter().all(|c| c.is_uppercase() || c.is_numeric()) {
        return Case::ScreamingSnake;
    }

    if chars.contains(&'_') {
        if chars
            .iter()
            .all(|c| c.is_lowercase() || c.is_numeric() || *c == '_')
        {
            return Case::Snake;
        }

        if chars
            .iter()
            .all(|c| c.is_uppercase() || c.is_numeric() || *c == '_')
        {
            return Case::ScreamingSnake;
        }

        return Case::Unknown;
    }

    if chars.contains(&'-') {
        if chars
            .iter()
            .all(|c| c.is_lowercase() || c.is_numeric() || *c == '-')
        {
            return Case::Kebab;
        }

        return Case::Unknown;
    }

    if chars.iter().any(|c| c.is_uppercase()) && !chars.contains(&'_') {
        if chars[0].is_uppercase() {
            return Case::Pascal;
        }
        return Case::Camel;
    }

    if chars.iter().all(|c| c.is_lowercase() || c.is_numeric()) {
        return Case::Snake;
    }

    Case::Unknown
}

// Split by uppercase (keep), underscore (discard), hyphen (discard)
// Lowercase all characters
// Join with underscore
pub(crate) fn to_snake_case(input: &str) -> String {
    let case = identify_case(input);

    match case {
        Case::Snake => input.to_string(),
        Case::Camel => {
            let mut result = String::new();
            let mut capitalize_next = false;

            for c in input.chars() {
                if c.is_uppercase() {
                    if !result.is_empty() && !capitalize_next {
                        result.push('_');
                    }
                    result.push(c.to_ascii_lowercase());
                    capitalize_next = false;
                } else {
                    result.push(c);
                }
            }

            result
        }
        Case::Pascal => {
            let mut result = String::new();
            let mut capitalize_next = true;

            for c in input.chars() {
                if c.is_uppercase() {
                    if !result.is_empty() && !capitalize_next {
                        result.push('_');
                    }
                    result.push(c.to_ascii_lowercase());
                    capitalize_next = false;
                } else {
                    result.push(c);
                }
            }

            result
        }
        Case::Kebab => input.replace('-', "_").to_ascii_lowercase(),
        Case::ScreamingSnake => input.to_ascii_lowercase(),
        Case::Unknown => {
            let mut result = String::new();
            let mut capitalize_next = false;

            for c in input.chars() {
                if c.is_uppercase() {
                    if !result.is_empty() && !capitalize_next {
                        result.push('_');
                    }
                    result.push(c.to_ascii_lowercase());
                    capitalize_next = false;
                } else {
                    result.push(c);
                }
            }

            result
        }
    }
}

pub(crate) fn to_pascal_case(input: &str) -> String {
    let case = identify_case(input);

    match case {
        Case::Snake => {
            let mut result = String::new();
            let mut capitalize_next = true;

            for c in input.chars() {
                if c == '_' {
                    capitalize_next = true;
                } else if capitalize_next {
                    result.push(c.to_ascii_uppercase());
                    capitalize_next = false;
                } else {
                    result.push(c);
                }
            }

            result
        }
        Case::Camel => {
            let mut result = input.get(0..1).unwrap_or_default().to_ascii_uppercase();
            let mut capitalize_next = true;

            for c in input.chars().skip(1) {
                if c.is_uppercase() {
                    if !result.is_empty() && !capitalize_next {
                        result.push('_');
                    }
                    result.push(c.to_ascii_uppercase());
                    capitalize_next = false;
                } else {
                    result.push(c);
                }
            }

            result
        }
        Case::Pascal => input.to_string(),
        Case::Kebab => {
            let mut result = String::new();
            let mut capitalize_next = true;

            for c in input.chars() {
                if c == '-' {
                    capitalize_next = true;
                } else if capitalize_next {
                    result.push(c.to_ascii_uppercase());
                    capitalize_next = false;
                } else {
                    result.push(c);
                }
            }

            result
        }
        Case::ScreamingSnake => {
            // `HELLO_WORLD` -> `HelloWorld`
            let mut result = String::new();
            let mut capitalize_next = true;
            for c in input.chars() {
                if c == '_' {
                    capitalize_next = true;
                } else if capitalize_next {
                    result.push(c.to_ascii_uppercase());
                    capitalize_next = false;
                } else {
                    result.push(c.to_ascii_lowercase());
                }
            }
            result
        }
        Case::Unknown => input.to_string(),
    }
}

pub(crate) fn convert_field_to_type(field: &Field, import_options: &ImportOptions) -> String {
    let mut field_type_name = field.field_type.name().to_string();

    // If attribute contains `@db.ObjectId`, convert field_type_name to `ObjectId`
    if field
        .attributes
        .iter()
        .any(|attr| attr.name() == "db.ObjectId")
    {
        field_type_name = "bson::oid::ObjectId".to_string();
    }

    let scalar = match field_type_name.as_str() {
        "Boolean" => "bool".to_string(),
        "Int" => "i64".to_string(),
        "Float" => "f64".to_string(),
        "String" => "String".to_string(),
        "Json" => "serde_json::Value".to_string(),
        "DateTime" => "chrono::DateTime<chrono::Utc>".to_string(),
        "bson::oid::ObjectId" => "bson::oid::ObjectId".to_string(),
        _ => {
            let field_type_name = if let Some(prefix) = &import_options.prefix {
                format!("{}{}", prefix, field_type_name)
            } else {
                field_type_name.to_string()
            };
            to_pascal_case(&field_type_name)
        }
    };

    let maybe_list = if field.arity.is_list() {
        format!("Vec<{}>", scalar)
    } else {
        scalar
    };

    let maybe_option = if field.arity.is_optional() {
        format!("Option<{}>", maybe_list)
    } else {
        maybe_list
    };

    maybe_option
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identify_case() {
        assert_eq!(identify_case("hello_world"), Case::Snake);
        assert_eq!(identify_case("HelloWorld"), Case::Pascal);
        assert_eq!(identify_case("helloWorld"), Case::Camel);
        assert_eq!(identify_case("hello-world"), Case::Kebab);
        assert_eq!(identify_case("HELLO_WORLD"), Case::ScreamingSnake);
        assert_eq!(identify_case("helloworld"), Case::Snake);
        assert_eq!(identify_case("hello_world_123"), Case::Snake);
        assert_eq!(identify_case("Hello_World"), Case::Unknown);
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(to_snake_case("helloWorld"), "hello_world");
        assert_eq!(to_snake_case("hello_world"), "hello_world");
        assert_eq!(to_snake_case("HELLO_WORLD"), "hello_world");
        assert_eq!(to_snake_case("helloworld"), "helloworld");
    }

    // #[test]
    // fn test_to_camel_case() {
    //     assert_eq!(to_camel_case("hello_world"), "helloWorld");
    //     assert_eq!(to_camel_case("helloWorld"), "helloWorld");
    //     assert_eq!(to_camel_case("hello-world"), "helloWorld");
    //     assert_eq!(to_camel_case("HELLO_WORLD"), "helloWorld");
    // }

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
        assert_eq!(to_pascal_case("helloWorld"), "HelloWorld");
        assert_eq!(to_pascal_case("hello-world"), "HelloWorld");
        assert_eq!(to_pascal_case("HELLO_WORLD"), "HelloWorld");
        assert_eq!(to_pascal_case("helloworld"), "Helloworld");
        assert_eq!(to_pascal_case("HelloWorld"), "HelloWorld");
    }

    // #[test]
    // fn test_to_kebab_case() {
    //     assert_eq!(to_kebab_case("HelloWorld"), "hello-world");
    //     assert_eq!(to_kebab_case("helloWorld"), "hello-world");
    //     assert_eq!(to_kebab_case("hello-world"), "hello-world");
    //     assert_eq!(to_kebab_case("Hello-World"), "hello-world");
    //     assert_eq!(to_kebab_case("HELLO_WORLD"), "hello-world");
    //     assert_eq!(to_kebab_case("helloworld"), "helloworld");
    // }

    // #[test]
    // fn test_to_screaming_snake_case() {
    //     assert_eq!(to_screaming_snake_case("HelloWorld"), "HELLO_WORLD");
    //     assert_eq!(to_screaming_snake_case("helloWorld"), "HELLO_WORLD");
    //     assert_eq!(to_screaming_snake_case("hello_world"), "HELLO_WORLD");
    //     assert_eq!(to_screaming_snake_case("Hello_World"), "HELLO_WORLD");
    //     assert_eq!(to_screaming_snake_case("HELLO_WORLD"), "HELLO_WORLD");
    //     assert_eq!(to_screaming_snake_case("helloworld"), "HELLOWORLD");
    //     assert_eq!(to_screaming_snake_case("hello-world"), "HELLO_WORLD");
    // }
}
