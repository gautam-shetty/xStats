# xStats

Static analysis tool to calculate code metrics for Java and Python

### Supported file extensions

- Java: `.java`
- Python: `.py`

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
