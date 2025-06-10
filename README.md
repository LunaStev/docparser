# Doc Parser

Extract Rust documentation comments from source files and output them in JSON or WSON format.

---

## Features

- Parses Rust source files and collects:
  - Module-level documentation (`//!`)
  - Function documentation (`///`)
  - Struct and enum documentation

- Outputs in:
  - `json`: Standard JSON format
  - `wson`: Wave Serialization Object Notation (WSON)

---

## Installation
Install from source:

```bash
cargo install docparser
```

After installation, you can run `docparser` as a CLI tool.

---

## Usage

```bash
docparser <filename> [--format json|wson]
```

### Examples
Default (WSON) format:

```bash
docparser src/lib.rs
```

JSON format:

```bash
docparser src/lib.rs --format json
```

Display help:

```bash
docparser --help
```

---

## Output Example
WSON output (default):

```text
{
    module_docs = [
        "This module is a user API.",
        "All user-related features are here."
    ],

    functions = [
        {
            name = "create_user",
            docs = [
                "Create a user.",
                "",
                "# Parameters",
                "* `name` - Username",
                "* `age` - Age",
                "",
                "# Returns",
                "* `User` struct"
            ]
        }
    ],

    structs = [

    ],

    enums = [

    ]
}
```

---

## License
This project is licensed under the [Mozilla Public License 2.0](https://www.mozilla.org/en-US/MPL/2.0/). See LICENSE for more details.