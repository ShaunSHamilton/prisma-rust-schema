use serde::{Deserialize, Serialize};
use serde_json::Value;

// /// Provided by Prisma CLI to generators
// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct EngineDMMF {
//     pub generator: Generator,
//     pub schema_path: String,
//     pub datamodel: String,
//     pub dmmf: DMMF,
//     pub datasources: Vec<Datasource>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct DMMF {
//     // pub datamodel: Value,
//     pub datamodel: Datamodel,
// }

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Datamodel {
//     pub enums: Vec<Enum>,
//     pub indexes: Vec<Index>,
//     pub models: Vec<Model>,
//     pub types: Vec<Type>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Type {
//     pub db_name: Value,
//     pub fields: Vec<TypeField>,
//     pub name: String,
//     pub primary_key: Value,
//     pub schema: Value,
//     pub unique_fields: Vec<Value>,
//     pub unique_indexes: Vec<Value>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct TypeField {
//     pub has_default_value: bool,
//     pub is_id: bool,
//     pub is_list: bool,
//     pub is_read_only: bool,
//     pub is_required: bool,
//     pub is_unique: bool,
//     pub kind: String,
//     pub name: String,
//     pub native_type: Value,
//     #[serde(rename = "type")]
//     pub _type: String,
// }

// impl From<&TypeField> for String {
//     fn from(field: &TypeField) -> Self {
//         // camelCase to snake_case
//         let field_name = field
//             .name
//             .clone()
//             .chars()
//             .fold(String::new(), |mut acc, c| {
//                 if c.is_uppercase() {
//                     if !acc.is_empty() {
//                         acc.push('_');
//                     }
//                     acc.push(c.to_ascii_lowercase());
//                 } else {
//                     acc.push(c);
//                 }
//                 acc
//             });
//         let mut field_type = String::new();
//         let t = match field._type.as_str() {
//             "String" => "String".to_string(),
//             "Int" => "i32".to_string(),
//             "Float" => "f64".to_string(),
//             "Boolean" => "bool".to_string(),
//             "DateTime" => "chrono::NaiveDateTime".to_string(),
//             _ => field._type.clone(),
//         };

//         if !field.is_required {
//             field_type.push_str("Option<");
//         }
//         if field.is_list {
//             field_type.push_str("Vec<");
//         }

//         field_type.push_str(&t);

//         if field.is_list {
//             field_type.push_str(">");
//         }
//         if !field.is_required {
//             field_type.push_str(">");
//         }

//         format!("pub {}: {},\n", field_name, field_type)
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Model {
//     pub db_name: Value,
//     pub fields: Vec<ModelField>,
//     pub is_generated: bool,
//     pub name: String,
//     pub primary_key: Value,
//     pub schema: Value,
//     pub unique_fields: Vec<Value>,
//     pub unique_indexes: Vec<Value>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct ModelField {
//     pub db_name: Option<Value>,
//     pub default: Option<ModelFieldDefault>,
//     pub has_default_value: bool,
//     pub is_generated: bool,
//     pub is_id: bool,
//     pub is_list: bool,
//     pub is_read_only: bool,
//     pub is_required: bool,
//     pub is_unique: bool,
//     pub is_updated_at: bool,
//     pub kind: String,
//     pub name: String,
//     pub native_type: Value,
//     pub relation_from_fields: Option<Vec<String>>,
//     pub relation_name: Option<Value>,
//     pub relation_to_fields: Option<Vec<String>>,
//     #[serde(rename = "type")]
//     pub _type: String,
// }

// impl From<&ModelField> for String {
//     fn from(field: &ModelField) -> Self {
//         let field_name = field
//             .name
//             .clone()
//             .chars()
//             .fold(String::new(), |mut acc, c| {
//                 if c.is_uppercase() {
//                     if !acc.is_empty() {
//                         acc.push('_');
//                     }
//                     acc.push(c.to_ascii_lowercase());
//                 } else {
//                     acc.push(c);
//                 }
//                 acc
//             });
//         let mut field_type = String::new();
//         let t = match field._type.as_str() {
//             "String" => "String".to_string(),
//             "Int" => "i32".to_string(),
//             "Float" => "f64".to_string(),
//             "Boolean" => "bool".to_string(),
//             "DateTime" => "chrono::NaiveDateTime".to_string(),
//             _ => field._type.clone(),
//         };

//         if !field.is_required {
//             field_type.push_str("Option<");
//         }
//         if field.is_list {
//             field_type.push_str("Vec<");
//         }

//         field_type.push_str(&t);

//         if field.is_list {
//             field_type.push_str(">");
//         }
//         if !field.is_required {
//             field_type.push_str(">");
//         }

//         format!("pub {}: {},\n", field_name, field_type)
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct ModelFieldDefault {
//     pub args: Vec<Value>,
//     pub name: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Index {
//     pub fields: Vec<IndexField>,
//     pub is_defined_on_field: bool,
//     pub model: String,
//     #[serde(rename = "type")]
//     pub _type: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct IndexField {
//     pub name: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Enum {
//     pub db_name: Value,
//     pub name: String,
//     pub values: Vec<EnumValue>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct EnumValue {
//     pub db_name: Value,
//     pub name: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Generator {
//     pub provider: EnvValue,
//     pub output: EnvValue,
//     pub name: String,
//     #[serde(default)]
//     pub is_custom_output: bool,
//     pub preview_features: Vec<String>,
//     pub config: Map<String, serde_json::Value>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct Datasource {
//     pub name: String,
//     pub provider: String,
//     pub url: EnvValue,
// }

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct EnvValue {
//     pub from_env_var: Option<String>,
//     pub value: Option<String>,
// }

// impl EnvValue {
//     pub fn get_value(&self) -> String {
//         self.from_env_var
//             .as_ref()
//             .and_then(|o| match o.as_str() {
//                 // dmmf is cringe apparently?
//                 "null" => None,
//                 env_var => {
//                     Some(std::env::var(env_var).expect(&format!("env var {env_var} not found")))
//                 }
//             })
//             .unwrap_or_else(|| self.value.clone().expect("value not found"))
//     }
// }

/// -------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Document {
    pub datamodel: Datamodel,
    // pub schema: Schema,
    // pub mappings: Mappings,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Mappings {
    #[serde(rename = "modelOperations")]
    pub model_operations: Vec<ModelMapping>,
    #[serde(rename = "otherOperations")]
    pub other_operations: OtherOperationMappings,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OtherOperationMappings {
    pub read: Vec<String>,
    pub write: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatamodelEnum {
    pub name: String,
    pub values: Vec<EnumValue>,
    #[serde(rename = "dbName")]
    pub db_name: Option<String>,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SchemaEnum {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnumValue {
    pub name: String,
    #[serde(rename = "dbName")]
    pub db_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Datamodel {
    pub models: Vec<Model>,
    pub enums: Vec<DatamodelEnum>,
    #[serde(rename = "types")]
    pub type_models: Vec<Model>,
    pub indexes: Vec<Index>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UniqueIndex {
    pub name: String,
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrimaryKey {
    pub name: Option<String>,
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Model {
    pub name: String,
    #[serde(rename = "dbName")]
    pub db_name: Option<String>,
    pub schema: Option<String>,
    pub fields: Vec<Field>,
    #[serde(rename = "uniqueFields")]
    pub unique_fields: Vec<Vec<String>>,
    #[serde(rename = "uniqueIndexes")]
    pub unique_indexes: Vec<UniqueIndex>,
    pub documentation: Option<String>,
    #[serde(rename = "primaryKey")]
    pub primary_key: Option<PrimaryKey>,
    #[serde(rename = "isGenerated")]
    pub is_generated: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum FieldKind {
    Scalar,
    Object,
    Enum,
    Unsupported,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum FieldNamespace {
    Model,
    Prisma,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum FieldLocation {
    Scalar,
    InputObjectTypes,
    OutputObjectTypes,
    EnumTypes,
    FieldRefTypes,
}

impl AsRef<str> for FieldLocation {
    fn as_ref(&self) -> &str {
        match self {
            FieldLocation::Scalar => "scalar",
            FieldLocation::InputObjectTypes => "inputObjectTypes",
            FieldLocation::OutputObjectTypes => "outputObjectTypes",
            FieldLocation::EnumTypes => "enumTypes",
            FieldLocation::FieldRefTypes => "fieldRefTypes",
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Field {
    pub kind: FieldKind,
    pub name: String,
    #[serde(rename = "isRequired")]
    pub is_required: bool,
    #[serde(rename = "isList")]
    pub is_list: bool,
    #[serde(rename = "isUnique")]
    pub is_unique: bool,
    #[serde(rename = "isId")]
    pub is_id: bool,
    #[serde(rename = "isReadOnly")]
    pub is_read_only: bool,
    #[serde(rename = "isGenerated")]
    pub is_generated: Option<bool>,
    #[serde(rename = "isUpdatedAt")]
    pub is_updated_at: Option<bool>,
    #[serde(rename = "type")]
    pub field_type: String,
    #[serde(rename = "nativeType")]
    pub native_type: Option<Vec<Value>>, // [string, string[]]
    #[serde(rename = "dbName")]
    pub db_name: Option<String>,
    #[serde(rename = "hasDefaultValue")]
    pub has_default_value: bool,
    #[serde(rename = "default")]
    pub default_value: Option<FieldDefaultValue>,
    #[serde(rename = "relationFromFields")]
    pub relation_from_fields: Option<Vec<String>>,
    #[serde(rename = "relationToFields")]
    pub relation_to_fields: Option<Vec<String>>,
    #[serde(rename = "relationOnDelete")]
    pub relation_on_delete: Option<String>,
    #[serde(rename = "relationOnUpdate")]
    pub relation_on_update: Option<String>,
    #[serde(rename = "relationName")]
    pub relation_name: Option<String>,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FieldDefaultValue {
    Object(FieldDefault),
    Scalar(FieldDefaultScalar),
    ScalarList(Vec<FieldDefaultScalar>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FieldDefault {
    pub name: String,
    pub args: Vec<Value>, // Keeping it flexible as string or number
}

pub type FieldDefaultScalar = Value; // Can be string, boolean, or number

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Index {
    pub model: String,
    #[serde(rename = "type")]
    pub index_type: IndexType,
    #[serde(rename = "isDefinedOnField")]
    pub is_defined_on_field: bool,
    pub name: Option<String>,
    #[serde(rename = "dbName")]
    pub db_name: Option<String>,
    pub algorithm: Option<String>,
    pub clustered: Option<bool>,
    pub fields: Vec<IndexField>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum IndexType {
    Id,
    Normal,
    Unique,
    Fulltext,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndexField {
    pub name: String,
    #[serde(rename = "sortOrder")]
    pub sort_order: Option<SortOrder>,
    pub length: Option<u32>,
    #[serde(rename = "operatorClass")]
    pub operator_class: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Schema {
    #[serde(rename = "rootQueryType")]
    pub root_query_type: Option<String>,
    #[serde(rename = "rootMutationType")]
    pub root_mutation_type: Option<String>,
    #[serde(rename = "inputObjectTypes")]
    pub input_object_types: InputObjectTypes,
    #[serde(rename = "outputObjectTypes")]
    pub output_object_types: OutputObjectTypes,
    #[serde(rename = "enumTypes")]
    pub enum_types: EnumTypes,
    #[serde(rename = "fieldRefTypes")]
    pub field_ref_types: FieldRefTypes,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InputObjectTypes {
    pub model: Option<Vec<InputType>>,
    pub prisma: Vec<InputType>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OutputObjectTypes {
    pub model: Vec<OutputType>,
    pub prisma: Vec<OutputType>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnumTypes {
    pub model: Option<Vec<SchemaEnum>>,
    pub prisma: Vec<SchemaEnum>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FieldRefTypes {
    pub prisma: Option<Vec<FieldRefType>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Query {
    pub name: String,
    pub args: Vec<SchemaArg>,
    pub output: QueryOutput,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueryOutput {
    pub name: String,
    #[serde(rename = "isRequired")]
    pub is_required: bool,
    #[serde(rename = "isList")]
    pub is_list: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TypeRef<T: AsRef<str>> {
    #[serde(rename = "isList")]
    pub is_list: bool,
    #[serde(rename = "type")]
    pub type_name: String,
    pub location: T,
    pub namespace: Option<FieldNamespace>,
}

pub type InputTypeRef = TypeRef<FieldLocation>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SchemaArg {
    pub name: String,
    pub comment: Option<String>,
    #[serde(rename = "isNullable")]
    pub is_nullable: bool,
    #[serde(rename = "isRequired")]
    pub is_required: bool,
    #[serde(rename = "inputTypes")]
    pub input_types: Vec<InputTypeRef>,
    pub deprecation: Option<Deprecation>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OutputType {
    pub name: String,
    pub fields: Vec<SchemaField>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SchemaField {
    pub name: String,
    #[serde(rename = "isNullable")]
    pub is_nullable: Option<bool>,
    #[serde(rename = "outputType")]
    pub output_type: OutputTypeRef,
    pub args: Vec<SchemaArg>,
    pub deprecation: Option<Deprecation>,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OutputTypeRef {
    #[serde(rename = "isList")]
    pub is_list: bool,
    #[serde(rename = "type")]
    pub type_name: String,
    pub location: FieldLocation,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Deprecation {
    #[serde(rename = "sinceVersion")]
    pub since_version: String,
    pub reason: String,
    #[serde(rename = "plannedRemovalVersion")]
    pub planned_removal_version: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InputType {
    pub name: String,
    pub constraints: InputTypeConstraints,
    pub meta: Option<InputTypeMeta>,
    pub fields: Vec<SchemaArg>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InputTypeConstraints {
    #[serde(rename = "maxNumFields")]
    pub max_num_fields: Option<u32>,
    #[serde(rename = "minNumFields")]
    pub min_num_fields: Option<u32>,
    pub fields: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InputTypeMeta {
    pub source: Option<String>,
    pub grouping: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FieldRefType {
    pub name: String,
    #[serde(rename = "allowTypes")]
    pub allow_types: Vec<TypeRef<FieldLocation>>,
    pub fields: Vec<SchemaArg>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelMapping {
    pub model: String,
    pub plural: Option<String>, // This is not optional in @prisma/dmmf
    #[serde(rename = "findUnique")]
    pub find_unique: Option<String>,
    #[serde(rename = "findUniqueOrThrow")]
    pub find_unique_or_throw: Option<String>,
    #[serde(rename = "findFirst")]
    pub find_first: Option<String>,
    #[serde(rename = "findFirstOrThrow")]
    pub find_first_or_throw: Option<String>,
    #[serde(rename = "findMany")]
    pub find_many: Option<String>,
    pub create: Option<String>,
    #[serde(rename = "createMany")]
    pub create_many: Option<String>,
    #[serde(rename = "createManyAndReturn")]
    pub create_many_and_return: Option<String>,
    pub update: Option<String>,
    #[serde(rename = "updateMany")]
    pub update_many: Option<String>,
    #[serde(rename = "updateManyAndReturn")]
    pub update_many_and_return: Option<String>,
    pub upsert: Option<String>,
    pub delete: Option<String>,
    #[serde(rename = "deleteMany")]
    pub delete_many: Option<String>,
    pub aggregate: Option<String>,
    #[serde(rename = "groupBy")]
    pub group_by: Option<String>,
    pub count: Option<String>,
    #[serde(rename = "findRaw")]
    pub find_raw: Option<String>,
    #[serde(rename = "aggregateRaw")]
    pub aggregate_raw: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ModelAction {
    FindUnique,
    FindUniqueOrThrow,
    FindFirst,
    FindFirstOrThrow,
    FindMany,
    Create,
    CreateMany,
    CreateManyAndReturn,
    Update,
    UpdateMany,
    UpdateManyAndReturn,
    Upsert,
    Delete,
    DeleteMany,
    GroupBy,
    Count, // TODO: count does not actually exist in DMMF
    Aggregate,
    FindRaw,
    AggregateRaw,
}

impl From<&Field> for String {
    fn from(field: &Field) -> Self {
        // camelCase to snake_case
        let field_name = field
            .name
            .clone()
            .chars()
            .fold(String::new(), |mut acc, c| {
                if c.is_uppercase() {
                    if !acc.is_empty() {
                        acc.push('_');
                    }
                    acc.push(c.to_ascii_lowercase());
                } else {
                    acc.push(c);
                }
                acc
            });
        let mut field_type = String::new();
        let t = match field.field_type.as_str() {
            "String" => "String".to_string(),
            "Int" => "i32".to_string(),
            "Float" => "f64".to_string(),
            "Boolean" => "bool".to_string(),
            "DateTime" => "chrono::NaiveDateTime".to_string(),
            _ => field.field_type.clone(),
        };

        if !field.is_required {
            field_type.push_str("Option<");
        }
        if field.is_list {
            field_type.push_str("Vec<");
        }

        field_type.push_str(&t);

        if field.is_list {
            field_type.push('>');
        }
        if !field.is_required {
            field_type.push('>');
        }

        format!("{:>4}pub {}: {},\n", "", field_name, field_type)
    }
}
