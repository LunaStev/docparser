use std::fs;
use std::env;
use docparser::parse_rust_doc_text;
use docparser::wson::to_wson;
use wson_rs::WsonValue;

fn print_help() {
    println!(
        "\
Usage: docparser <filename> [--format json|wson]

Options:
  --format <format>   Output format: json or wson (default: wson)
  -h, --help          Print this help message
"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        print_help();
        return;
    }

    let filename = &args[1];
    let format = args.iter()
        .position(|arg| arg == "--format")
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
        .unwrap_or("wson");

    let content = fs::read_to_string(filename).expect("Failed to read file");
    let result = parse_rust_doc_text(&content);

    match format {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
        }
        "wson" => {
            let wson_value = to_wson(&result);
            let wson_string = wson_rs::dumps(
                match &wson_value {
                    WsonValue::Object(map) => map,
                    _ => panic!("to_wson must return WsonValue::Object"),
                }
            ).unwrap();
            println!("{}", wson_string);
        }
        other => {
            eprintln!("Unknown format: '{}'. Supported formats are 'json' and 'wson'.", other);
        }
    }
}
