use prisma_rust_schema::import_types;
use serde::{Deserialize, Serialize};

import_types!("./prisma/schema.prisma");

import_types!(
    schema_path = "./prisma/no-annotation-schema.prisma",
    prefix = "A",
    derive = [Deserialize, Serialize, Debug, PartialEq]
);
