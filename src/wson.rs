use wson_rs::{WsonValue, WsonMap};
use crate::{DocItem, DocOutput};

pub fn to_wson(doc: &DocOutput) -> WsonValue {
    let mut map = WsonMap::new();

    // module_docs: Array of Strings
    map.insert(
        "module_docs".to_string(),
        WsonValue::Array(doc.module_docs.iter().cloned().map(WsonValue::String).collect()),
    );

    // functions: Array of { name, docs }
    map.insert(
        "functions".to_string(),
        WsonValue::Array(
            doc.functions
                .iter()
                .map(doc_item_to_wson)
                .collect(),
        ),
    );

    // structs: Array of { name, docs }
    map.insert(
        "structs".to_string(),
        WsonValue::Array(
            doc.structs
                .iter()
                .map(doc_item_to_wson)
                .collect(),
        ),
    );

    // enums: Array of { name, docs }
    map.insert(
        "enums".to_string(),
        WsonValue::Array(
            doc.enums
                .iter()
                .map(doc_item_to_wson)
                .collect(),
        ),
    );

    WsonValue::Object(map)
}

fn doc_item_to_wson(item: &DocItem) -> WsonValue {
    let mut map = WsonMap::new();
    map.insert("name".to_string(), WsonValue::String(item.name.clone()));
    map.insert(
        "docs".to_string(),
        WsonValue::Array(item.docs.iter().cloned().map(WsonValue::String).collect()),
    );
    WsonValue::Object(map)
}
