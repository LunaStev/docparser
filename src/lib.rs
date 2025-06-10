pub mod wson;

use serde::Serialize;
use regex::Regex;

#[derive(Debug, Serialize)]
pub struct DocItem {
    pub name: String,
    pub docs: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct DocOutput {
    pub module_docs: Vec<String>,
    pub functions: Vec<DocItem>,
    pub structs: Vec<DocItem>,
    pub enums: Vec<DocItem>,
}

pub fn parse_rust_doc_text(source: &str) -> DocOutput {
    let mut module_docs = vec![];
    let mut functions = vec![];
    let mut structs = vec![];
    let mut enums = vec![];

    let mut doc_buffer: Vec<String> = vec![];

    for line in source.lines() {
        let trimmed = line.trim_start();

        if trimmed.starts_with("//!") {
            module_docs.push(trimmed.trim_start_matches("//!").trim().to_string());
        } else if trimmed.starts_with("///") {
            doc_buffer.push(trimmed.trim_start_matches("///").trim().to_string());
        } else if trimmed.starts_with("fn ") || trimmed.contains(" fn ") {
            if let Some(name) = extract_fn_name(line) {
                functions.push(DocItem {
                    name,
                    docs: std::mem::take(&mut doc_buffer),
                });
            }
        } else if trimmed.starts_with("struct ") || trimmed.contains(" struct ") {
            if let Some(name) = extract_ident_after_keyword(trimmed, "struct") {
                structs.push(DocItem {
                    name,
                    docs: std::mem::take(&mut doc_buffer),
                });
            }
        } else if trimmed.starts_with("enum ") || trimmed.contains(" enum ") {
            if let Some(name) = extract_ident_after_keyword(trimmed, "enum") {
                enums.push(DocItem {
                    name,
                    docs: std::mem::take(&mut doc_buffer),
                });
            }
        } else {
            doc_buffer.clear();
        }
    }

    DocOutput {
        module_docs,
        functions,
        structs,
        enums,
    }
}

fn extract_ident_after_keyword(line: &str, keyword: &str) -> Option<String> {
    line.split_whitespace()
        .skip_while(|s| *s != keyword)
        .nth(1)
        .map(|s| s.trim_end_matches('{').trim_end_matches('(').to_string())
}

fn extract_fn_name(line: &str) -> Option<String> {
    let re = Regex::new(r"fn\s+([a-zA-Z0-9_]+)").unwrap();
    re.captures(line).and_then(|caps| caps.get(1)).map(|m| m.as_str().to_string())
}
