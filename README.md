# Calc CLI

A simple command-line calculator built with Rust and [clap](https://docs.rs/clap/) for argument parsing.

## Features

- Basic operations: addition, subtraction, multiplication, and division
- Safe division by zero handling (won't crash, returns a clear error message instead)
- Supports decimal numbers (uses the `f64` type)

## Installation

Make sure [Rust and Cargo](https://www.rust-lang.org/tools/install) are installed on your machine.

Clone this repo, then build it:

```bash
git clone https://github.com/username/calc-cli.git
cd calc-cli
cargo build --release
```

The resulting binary will be located at `target/release/calc-cli`.

## Usage

General format:

```bash
calc-cli -x <first_number> -y <second_number> <operation>
```

Available operations: `add`, `sub`, `mult`, `div`

### Examples

Addition:
```bash
cargo run -- -x 10 -y 5 add
# Output: 15
```

Subtraction:
```bash
cargo run -- -x 10 -y 5 sub
# Output: 5
```

Multiplication:
```bash
cargo run -- -x 10 -y 5 mult
# Output: 50
```

Division:
```bash
cargo run -- -x 10 -y 5 div
# Output: 2
```

Division by zero (handled gracefully):
```bash
cargo run -- -x 10 -y 0 div
# Output: Can't divide by 0
```

> Note: `cargo run --` is used when running from source without building a separate binary. Once you've run `cargo build --release`, you can call the binary directly with `./target/release/calc-cli -x 10 -y 5 add`, no need for `cargo run --`.

## Argument Structure

| Flag | Long form | Type | Description |
|------|-----------|------|-------------|
| `-x` | `--number-1` | `f64` | First number |
| `-y` | `--number-2` | `f64` | Second number |

The subcommand at the end (`add`/`sub`/`mult`/`div`) determines which operation is performed.

## License

MIT
