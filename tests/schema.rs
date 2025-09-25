use prisma_rust_schema::import_types;
use serde::{Deserialize, Serialize};

import_types!(
    schema_paths = ["./prisma/schema.prisma"],
    derive = [Serialize],
    patch = [
        struct User {
        #[serde(default)]
        pub defaulted: String
    }]
);

import_types!(
    schema_paths = ["./prisma/no-annotation-schema.prisma"],
    prefix = "A",
    derive = [Deserialize, Serialize, Debug, PartialEq],
);

import_types!(
    schema_paths = [
        "https://raw.githubusercontent.com/ShaunSHamilton/prisma-rust-schema/refs/heads/master/prisma/schema.prisma"
    ],
    prefix = "U"
);
