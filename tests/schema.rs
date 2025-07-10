use prisma_rust_schema::import_types;
use serde::{Deserialize, Serialize};

import_types!(
    schema_path = "./prisma/schema.prisma",
    derive = [Serialize],
    patch = [
        struct User {
        #[serde(default)]
        pub defaulted: String
    }]
);

import_types!(
    schema_path = "./prisma/no-annotation-schema.prisma",
    prefix = "A",
    derive = [Deserialize, Serialize, Debug, PartialEq],
);
