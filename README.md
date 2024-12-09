
# RCC (Rusty Console Calculator)

RCC is a versatile CLI tool designed to perform basic mathematical calculations and manage timestamps. It is lightweight, easy to use, and can handle tasks like converting timestamps, generating human-readable date-time strings, and fetching the current Unix timestamp.

## Features

- **Mathematical Calculations**: Evaluate mathematical expressions directly from the command line.  
  Example: `RCC "1 + 2 * 3"`.
- **Timestamp Conversion**: Convert Unix timestamps to specific time zones.  
  Command: `ctz`.
- **Human-Readable Timestamps**: Translate Unix timestamps into local human-readable date-time formats.  
  Command: `ctr`.
- **Current Timestamp**: Get the current Unix timestamp.  
  Command: `ct`.

## Installation

1. Add RCC to your project by installing it via `cargo`:

   ```bash
   cargo install RCC
   ```

2. Clone the repository and build manually if needed:

   ```bash
   git clone https://github.com/yourusername/rcc.git
   cd rcc
   cargo build --release
   ```

3. Run the built binary:

   ```bash
   ./target/release/RCC
   ```

## Usage

RCC can be used in two ways: evaluating mathematical expressions or using timestamp-related commands.

### 1. Mathematical Calculations

To evaluate a mathematical expression, simply pass the expression as a string:

```bash
RCC "13 + 14 * 2"
```

Output:

```
41
```

### 2. Timestamp Commands

#### Convert a Unix Timestamp to a Specific Timezone

Use the `ctz` command to convert a Unix timestamp to a given timezone:

```bash
RCC ctz 1702216800 America/New_York
```

Output:

```
2024-12-09 07:00:00 -05:00
```

#### Convert a Unix Timestamp to a Human-Readable Format

Use the `ctr` command to convert a Unix timestamp into a human-readable date-time in the local timezone:

```bash
RCC ctr 1702216800
```

Output:

```
2024-12-09 12:00:00
```

#### Get the Current Unix Timestamp

Use the `ct` command to fetch the current Unix timestamp:

```bash
RCC ct
```

Output:

```
1702216800
```

### Help Menu

Run the `--help` flag to see all available options:

```bash
RCC --help
```

Example Output:

```
RCC 0.0.1
A CLI tool for performing calculations and managing timestamps

RCC is a versatile CLI tool that allows you to:
1. Perform basic mathematical calculations by providing an expression as a string, e.g., '1 + 2'.
2. Convert timestamps to a specific timezone using the 'ctz' command.
3. Translate timestamps into human-readable formats using the 'ctr' command.
4. Get the current Unix timestamp using the 'ct' command.

USAGE:
    RCC [EXPRESSION] [COMMAND]

ARGS:
    <EXPRESSION>    Mathematical expression to evaluate, e.g., '13 + 14 * 2'

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

COMMANDS:
    ctz   Convert a Unix timestamp to a specific timezone
    ctr   Convert a Unix timestamp to a human-readable format
    ct    Get the current Unix timestamp
    help  Print this message or the help of the given subcommand(s)
```

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature-name`).
3. Commit your changes (`git commit -m "Add feature-name"`).
4. Push your branch (`git push origin feature-name`).
5. Open a pull request.

## License

RCC is licensed under the MIT License. See the [LICENSE](LICENSE.md) file for details.

---

Happy Calculating!
