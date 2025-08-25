# zlang

A command-line tool for working with Z Language.

## Features

- Run Z Language scripts from the command line
- Compile Z Language source files
- Interactive REPL for Z Language
- Lint and format Z Language code

## How to Run

### Installation

Clone the repository and build from source:

```sh
git clone https://github.com/zlang-cli/zlang.git
cd zlang
# If using Rust:
cargo build --release
# If using Node.js:
npm install
npm run build
# If using Go:
go build -o zlang .
```

### Usage

After building, run the CLI with:

```sh
./zlang --help
```

#### Common Commands

- Run a script:  
  ```sh
  ./zlang run script.zl
  ```
- Compile a file:  
  ```sh
  ./zlang compile source.zl
  ```
- Start REPL:  
  ```sh
  ./zlang repl
  ```

## Version

Current version: **v1.0.0**

Check your version with:

```sh
./zlang --version
```

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
