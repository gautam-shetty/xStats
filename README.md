# xStats

Static analysis tool designed to calculate code metrics for Java and Python projects.

## Features

Currently supported code metrics and their descriptions

| Metric        | Description                         | Java    | Python  |
| ------------- | ----------------------------------- | ------- | ------- |
| **is_broken** | If it has any error or missing node | &check; | &check; |
| **aloc**      | Actual lines of code                | &check; | &check; |
| **eloc**      | Empty lines of code                 | &check; | &check; |
| **cloc**      | Comments lines of code              | &check; | &check; |
| **dcloc**     | Document comments lines of code     | &check; | &check; |
| **noi**       | number of imports                   | &check; | &check; |
| **noc**       | number of classes                   | &check; | &check; |
| **nom**       | number of methods                   | &check; | &check; |
| **cc**        | Cyclomatic complexity               | &check; | &check; |
| **pc**        | Parameter count                     | &check; | &check; |
| _language_    | Language of the code block          | &check; | &check; |
| _file_path_   | Path of the file                    | &check; | &check; |
| _start_row_   | Start row of the code block         | &check; | &check; |
| _start_col_   | Start column of the code block      | &check; | &check; |
| _end_row_     | End row of the code block           | &check; | &check; |
| _end_col_     | End column of the code block        | &check; | &check; |
| _node_name_   | Name of the node                    | &check; | &check; |
| _node_type_   | Type of the node                    | &check; | &check; |

> **Note**: If the node is broken, the rest of the metrics might not be accurate

#### Supported file extensions

- Java: `.java`
- Python: `.py`

## Usage

You can download the latest release artifacts from the [releases page](https://github.com/gautam-shetty/xStats/releases).

To analyze repositories using `xStats`, run the following command:

```bash
xStats --target <TARGET> --output <OUTPUT>
```

##### Options

- `-t, --target <TARGET>`: Specify the target file or directory.
- `-o, --output <OUTPUT>`: Specify the output file.
- `-a, --all-commits`: Analyze all commits.
- `--format <FORMAT>`: Specify the output format (default: json).
- `-h, --help`: Print help information.
- `-V, --version`: Print version information.

### How to build

1. Ensure you have Rust installed on your machine. If not, you can install it from [here](https://www.rust-lang.org/tools/install)
2. Clone the repository

```bash
git clone https://github.com/gautam-shetty/xStats.git
cd xStats
```

3. Build the project

```bash
cargo build --release
```
