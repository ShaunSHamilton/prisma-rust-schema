use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{dmmf::Document, typed_sql::SqlQueryOutput};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvValue {
    pub from_env_var: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BinaryTargetsEnvValue {
    pub from_env_var: Option<String>,
    pub value: String,
    pub native: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratorConfig {
    pub name: String,
    pub output: Option<EnvValue>,
    pub is_custom_output: Option<bool>,
    pub provider: EnvValue,
    pub config: HashMap<String, Option<String>>,
    pub binary_targets: Vec<BinaryTargetsEnvValue>,
    pub preview_features: Vec<String>,
    pub env_paths: Option<EnvPaths>,
    pub source_file_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvPaths {
    pub root_env_path: Option<String>,
    pub schema_env_path: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConnectorType {
    Mysql,
    Mongodb,
    Sqlite,
    Postgresql,
    Postgres,
    #[serde(rename = "prisma+postgres")]
    /// Note: used for Prisma Postgres, managed by PDP
    PrismaPostgres,
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
#[serde(rename_all = "camelCase")]
pub struct DataSource {
    pub name: String,
    pub provider: ConnectorType,
    pub active_provider: ActiveConnectorType,
    pub url: EnvValue,
    pub direct_url: Option<EnvValue>,
    pub schemas: Vec<String>,
    pub source_file_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BinaryPaths {
    /// key: target, value: path
    pub schema_engine: Option<HashMap<String, String>>,
    pub query_engine: Option<HashMap<String, String>>,
    pub libquery_engine: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratorOptions {
    pub generator: GeneratorConfig,
    pub other_generators: Vec<GeneratorConfig>,
    pub schema_path: String,
    pub dmmf: Document,
    pub datasources: Vec<DataSource>,
    pub datamodel: String,
    /// Hash of the version
    pub version: String,
    pub binary_paths: Option<BinaryPaths>,
    pub postinstall: Option<bool>,
    pub no_engine: Option<bool>,
    pub no_hints: Option<bool>,
    pub allow_no_models: Option<bool>,
    pub env_paths: Option<EnvPaths>,
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
#[serde(rename_all = "camelCase")]
pub struct GeneratorManifest {
    pub pretty_name: Option<String>,
    pub default_output: Option<String>,
    pub denylists: Option<DenyLists>,
    pub requires_generators: Option<Vec<String>>,
    pub requires_engines: Option<Vec<EngineType>>,
    pub version: Option<String>,
    pub requires_engine_version: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DenyLists {
    pub models: Option<Vec<String>>,
    pub fields: Option<Vec<String>>,
}
