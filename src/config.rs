use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{dmmf::Document, typed_sql::SqlQueryOutput};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnvValue {
    #[serde(rename = "fromEnvVar")]
    pub from_env_var: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BinaryTargetsEnvValue {
    #[serde(rename = "fromEnvVar")]
    pub from_env_var: Option<String>,
    pub value: String,
    pub native: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GeneratorConfig {
    pub name: String,
    pub output: Option<EnvValue>,
    #[serde(rename = "isCustomOutput")]
    pub is_custom_output: Option<bool>,
    pub provider: EnvValue,
    pub config: HashMap<String, Option<String>>,
    #[serde(rename = "binaryTargets")]
    pub binary_targets: Vec<BinaryTargetsEnvValue>,
    #[serde(rename = "previewFeatures")]
    pub preview_features: Vec<String>,
    #[serde(rename = "envPaths")]
    pub env_paths: Option<EnvPaths>,
    #[serde(rename = "sourceFilePath")]
    pub source_file_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnvPaths {
    #[serde(rename = "rootEnvPath")]
    pub root_env_path: Option<String>,
    #[serde(rename = "schemaEnvPath")]
    pub schema_env_path: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConnectorType {
    Mysql,
    Mongodb,
    Sqlite,
    Postgresql,
    Postgres, // TODO: we could normalize postgres to postgresql this in engines to reduce the complexity?
    #[serde(rename = "prisma+postgres")]
    PrismaPostgres, // Note: used for Prisma Postgres, managed by PDP
    Sqlserver,
    Cockroachdb,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ActiveConnectorType {
    Mysql,
    Mongodb,
    Sqlite,
    Postgresql,
    Sqlserver,
    Cockroachdb,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataSource {
    pub name: String,
    pub provider: ConnectorType,
    #[serde(rename = "activeProvider")]
    pub active_provider: ActiveConnectorType,
    pub url: EnvValue,
    #[serde(rename = "directUrl")]
    pub direct_url: Option<EnvValue>,
    pub schemas: Vec<String>,
    #[serde(rename = "sourceFilePath")]
    pub source_file_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BinaryPaths {
    #[serde(rename = "schemaEngine")]
    pub schema_engine: Option<HashMap<String, String>>, // key: target, value: path
    #[serde(rename = "queryEngine")]
    pub query_engine: Option<HashMap<String, String>>,
    #[serde(rename = "libqueryEngine")]
    pub libquery_engine: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GeneratorOptions {
    pub generator: GeneratorConfig,
    #[serde(rename = "otherGenerators")]
    pub other_generators: Vec<GeneratorConfig>,
    #[serde(rename = "schemaPath")]
    pub schema_path: String,
    pub dmmf: Document,
    pub datasources: Vec<DataSource>,
    #[serde(rename = "datamodel")]
    pub datamodel: String,
    pub version: String, // version hash
    #[serde(rename = "binaryPaths")]
    pub binary_paths: Option<BinaryPaths>,
    pub postinstall: Option<bool>,
    #[serde(rename = "noEngine")]
    pub no_engine: Option<bool>,
    #[serde(rename = "noHints")]
    pub no_hints: Option<bool>,
    #[serde(rename = "allowNoModels")]
    pub allow_no_models: Option<bool>,
    #[serde(rename = "envPaths")]
    pub env_paths: Option<EnvPaths>,
    #[serde(rename = "typedSql")]
    pub typed_sql: Option<Vec<SqlQueryOutput>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::enum_variant_names)]
pub enum EngineType {
    QueryEngine,
    LibqueryEngine,
    SchemaEngine,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GeneratorManifest {
    #[serde(rename = "prettyName")]
    pub pretty_name: Option<String>,
    #[serde(rename = "defaultOutput")]
    pub default_output: Option<String>,
    pub denylists: Option<DenyLists>,
    #[serde(rename = "requiresGenerators")]
    pub requires_generators: Option<Vec<String>>,
    #[serde(rename = "requiresEngines")]
    pub requires_engines: Option<Vec<EngineType>>,
    pub version: Option<String>,
    #[serde(rename = "requiresEngineVersion")]
    pub requires_engine_version: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DenyLists {
    pub models: Option<Vec<String>>,
    pub fields: Option<Vec<String>>,
}
