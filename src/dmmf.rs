use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Document {
    pub datamodel: Datamodel,
    pub schema: Schema,
    pub mappings: Mappings,
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
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub name: String,
    pub db_name: Option<String>,
    pub schema: Option<String>,
    pub fields: Vec<Field>,
    pub unique_fields: Vec<Vec<String>>,
    pub unique_indexes: Vec<UniqueIndex>,
    pub documentation: Option<String>,
    pub primary_key: Option<PrimaryKey>,
    pub is_generated: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FieldKind {
    Scalar,
    Object,
    Enum,
    Unsupported,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
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
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub kind: FieldKind,
    pub name: String,
    pub is_required: bool,
    pub is_list: bool,
    pub is_unique: bool,
    pub is_id: bool,
    pub is_read_only: bool,
    pub is_generated: Option<bool>,
    pub is_updated_at: Option<bool>,
    #[serde(rename = "type")]
    pub field_type: String,
    /// [string | string[]]
    pub native_type: Option<Vec<Value>>,
    pub db_name: Option<String>,
    pub has_default_value: bool,
    #[serde(rename = "default")]
    pub default_value: Option<FieldDefaultValue>,
    pub relation_from_fields: Option<Vec<String>>,
    pub relation_to_fields: Option<Vec<String>>,
    pub relation_on_delete: Option<String>,
    pub relation_on_update: Option<String>,
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
    /// Can be a string or number
    pub args: Vec<Value>,
}

/// String, bool, or number
pub type FieldDefaultScalar = Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    pub model: String,
    #[serde(rename = "type")]
    pub index_type: IndexType,
    pub is_defined_on_field: bool,
    pub name: Option<String>,
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
#[serde(rename_all = "camelCase")]
pub struct IndexField {
    pub name: String,
    pub sort_order: Option<SortOrder>,
    pub length: Option<u32>,
    pub operator_class: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub root_query_type: Option<String>,
    pub root_mutation_type: Option<String>,
    pub input_object_types: InputObjectTypes,
    pub output_object_types: OutputObjectTypes,
    pub enum_types: EnumTypes,
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
#[serde(rename_all = "camelCase")]
pub struct QueryOutput {
    pub name: String,
    pub is_required: bool,
    pub is_list: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeRef<T: AsRef<str>> {
    pub is_list: bool,
    #[serde(rename = "type")]
    pub type_name: String,
    pub location: T,
    pub namespace: Option<FieldNamespace>,
}

pub type InputTypeRef = TypeRef<FieldLocation>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaArg {
    pub name: String,
    pub comment: Option<String>,
    pub is_nullable: bool,
    pub is_required: bool,
    pub input_types: Vec<InputTypeRef>,
    pub deprecation: Option<Deprecation>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OutputType {
    pub name: String,
    pub fields: Vec<SchemaField>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaField {
    pub name: String,
    pub is_nullable: Option<bool>,
    pub output_type: OutputTypeRef,
    pub args: Vec<SchemaArg>,
    pub deprecation: Option<Deprecation>,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputTypeRef {
    pub is_list: bool,
    #[serde(rename = "type")]
    pub type_name: String,
    pub location: FieldLocation,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Deprecation {
    pub since_version: String,
    pub reason: String,
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
#[serde(rename_all = "camelCase")]
pub struct InputTypeConstraints {
    pub max_num_fields: Option<u32>,
    pub min_num_fields: Option<u32>,
    pub fields: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InputTypeMeta {
    pub source: Option<String>,
    pub grouping: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldRefType {
    pub name: String,
    pub allow_types: Vec<TypeRef<FieldLocation>>,
    pub fields: Vec<SchemaArg>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelMapping {
    pub model: String,
    /// This is not optional in @prisma/dmmf, but can be None in the Generator DMMF
    pub plural: Option<String>,
    pub find_unique: Option<String>,
    pub find_unique_or_throw: Option<String>,
    pub find_first: Option<String>,
    pub find_first_or_throw: Option<String>,
    pub find_many: Option<String>,
    pub create: Option<String>,
    pub create_many: Option<String>,
    pub create_many_and_return: Option<String>,
    pub update: Option<String>,
    pub update_many: Option<String>,
    pub update_many_and_return: Option<String>,
    pub upsert: Option<String>,
    pub delete: Option<String>,
    pub delete_many: Option<String>,
    pub aggregate: Option<String>,
    pub group_by: Option<String>,
    pub count: Option<String>,
    pub find_raw: Option<String>,
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
    /// Unused
    Count,
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
