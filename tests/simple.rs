use prisma_rust_schema::import_types;
use serde::{Deserialize, Serialize};

import_types!(
schema_path = "./prisma/test.prisma",
prefix = "A",
derive = [Deserialize, Serialize, Debug, PartialEq],
patch = [
    struct ATest {
        #[serde(default)]
        pub defaulted: String
    },
    struct ADep {
        pub d: usize
    }
    ]
);

#[test]
fn simple() {
    let dep = ADep { d: 0 };

    let t = ATest {
        id: bson::oid::ObjectId::new(),
        defaulted: Default::default(),
        dep,
    };

    assert_eq!(t.defaulted, "".to_string());
}
