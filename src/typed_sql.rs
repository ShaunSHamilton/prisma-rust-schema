use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SqlQueryOutput {
    pub name: String,
    pub source: String,
    pub documentation: Option<String>,
    pub parameters: Vec<SqlQueryParameterOutput>,
    pub result_columns: Vec<SqlQueryColumnOutput>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SqlQueryParameterOutput {
    pub name: String,
    pub typ: QueryIntrospectionType,
    pub documentation: Option<String>,
    pub nullable: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SqlQueryColumnOutput {
    pub name: String,
    pub typ: QueryIntrospectionType,
    pub nullable: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum QueryIntrospectionType {
    Builtin(QueryIntrospectionBuiltinType),
    UserDefined(Cow<'static, str>),
}

// This must remain in sync with the `quaint::ColumnType` enum in the QueryEngine.
// https://github.com/prisma/prisma-engines/blob/main/quaint/src/connector/column_type.rs#L12
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum QueryIntrospectionBuiltinType {
    Int,
    Bigint,
    Float,
    Double,
    String,
    Enum,
    Bytes,
    Bool,
    Char,
    Decimal,
    Json,
    Xml,
    Uuid,
    Datetime,
    Date,
    Time,
    IntArray,
    BigintArray,
    FloatArray,
    DoubleArray,
    StringArray,
    CharArray,
    BytesArray,
    BoolArray,
    DecimalArray,
    JsonArray,
    XmlArray,
    UuidArray,
    DatetimeArray,
    DateArray,
    TimeArray,
    Null,
    Unknown,
}

impl From<&'static str> for QueryIntrospectionType {
    fn from(s: &'static str) -> Self {
        match s {
            "int" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Int),
            "bigint" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Bigint),
            "float" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Float),
            "double" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Double),
            "string" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::String),
            "enum" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Enum),
            "bytes" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Bytes),
            "bool" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Bool),
            "char" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Char),
            "decimal" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Decimal),
            "json" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Json),
            "xml" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Xml),
            "uuid" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Uuid),
            "datetime" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Datetime),
            "date" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Date),
            "time" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Time),
            "int-array" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::IntArray),
            "bigint-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::BigintArray)
            }
            "float-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::FloatArray)
            }
            "double-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::DoubleArray)
            }
            "string-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::StringArray)
            }
            "char-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::CharArray)
            }
            "bytes-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::BytesArray)
            }
            "bool-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::BoolArray)
            }
            "decimal-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::DecimalArray)
            }
            "json-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::JsonArray)
            }
            "xml-array" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::XmlArray),
            "uuid-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::UuidArray)
            }
            "datetime-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::DatetimeArray)
            }
            "date-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::DateArray)
            }
            "time-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::TimeArray)
            }
            "null" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Null),
            "unknown" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Unknown),
            other => QueryIntrospectionType::UserDefined(other.into()),
        }
    }
}

impl From<String> for QueryIntrospectionType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "int" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Int),
            "bigint" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Bigint),
            "float" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Float),
            "double" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Double),
            "string" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::String),
            "enum" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Enum),
            "bytes" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Bytes),
            "bool" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Bool),
            "char" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Char),
            "decimal" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Decimal),
            "json" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Json),
            "xml" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Xml),
            "uuid" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Uuid),
            "datetime" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Datetime),
            "date" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Date),
            "time" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Time),
            "int-array" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::IntArray),
            "bigint-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::BigintArray)
            }
            "float-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::FloatArray)
            }
            "double-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::DoubleArray)
            }
            "string-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::StringArray)
            }
            "char-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::CharArray)
            }
            "bytes-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::BytesArray)
            }
            "bool-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::BoolArray)
            }
            "decimal-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::DecimalArray)
            }
            "json-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::JsonArray)
            }
            "xml-array" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::XmlArray),
            "uuid-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::UuidArray)
            }
            "datetime-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::DatetimeArray)
            }
            "date-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::DateArray)
            }
            "time-array" => {
                QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::TimeArray)
            }
            "null" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Null),
            "unknown" => QueryIntrospectionType::Builtin(QueryIntrospectionBuiltinType::Unknown),
            other => QueryIntrospectionType::UserDefined(other.to_string().into()),
        }
    }
}
