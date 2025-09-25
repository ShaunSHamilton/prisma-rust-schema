use prisma_rust_schema::import_types;
use serde::{Deserialize, Serialize};

import_types!(
schema_paths = ["./prisma/test.prisma"],
prefix = "A",
derive = [Deserialize, Serialize, Debug, PartialEq, Default],
patch = [
    struct ATest {
        #[serde(default)]
        pub defaulted: String
    },
    struct ADep {
        pub e: ADepE
    }
    ]
);

impl Default for ADepE {
    fn default() -> Self {
        Self::A
    }
}

#[test]
fn simple() {
    let dep = ADep { e: ADepE::A };

    let t = ATest {
        id: bson::oid::ObjectId::new(),
        defaulted: Default::default(),
        dep,
    };

    assert_eq!(t.defaulted, "".to_string());
}
