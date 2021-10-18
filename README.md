
# logpeek

![](https://github.com/unlink2/logpeek/actions/workflows/build.yml/badge.svg)
![](https://github.com/unlink2/logpeek/actions/workflows/test.yml/badge.svg)

Logpeek is a simple logfile regex matcher program.
It allows matching a file line by line using regular expressions and can be configured
using a JSON based configuration file.

## Table of content

- [Installation](#Installation)
- [Usage](#Usage)
- [License](#License)
- [Contributing](#Contributing)
- [TODO](#TODO)

## Installation

This program requires the latest version of Rust.
To install minutecat-cli simplt clone the repository and run:

```sh
cargo install --path ./cli --locked
```

## Usage

### Command line

The following command will scan main.ccp for the word main.
Use --help for more information and a list of available options.

```sh
logpeek ./main.cpp -r "main" -o "{}" -p
```

### JSON Configuration

The following json can be specified using the -c flag.
It is functionally identical to the command line example above

```json
{
  "conditions": [
    {
      "if_match": {
        "kind": {
          "Re": {
            "expr": "main"
          }
        },
        "or": [],
        "and": [],
        "not": false
      },
      "then": {
        "Basic": {
          "message": "{}"
        }
      },
      "output_input": true,
      "else_then": null
    }
  ]
}
```

## License

This program is distributed under the terms of the MIT License.

## Contributing

All contributions are welcome.
Both pull requests and issue reports are always appreciated.
Please make sure that all existing tests pass before submitting a pull request.

## TODO

