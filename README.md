# `dotdim`

> A lightning-fast CLI utility that colorizes and intelligently masks your `.env` files. Designed for developers who frequently screen share or pair program, it formats your configuration on the fly, ensuring you can debug your environment without leaking your actual API keys or database passwords on stream.

---

## Features

### 🚀 Phase 1: Core Architecture & I/O
- **CLI Routing**: Built with `clap` (derive API) handling positional file paths (defaulting to `.env`), with standard `--help` and `--version` flags.
- **Stream Reading**: Uses `std::io::BufReader` to process target files line-by-line, guaranteeing memory efficiency even with huge configuration dumps.
- **Stdin Support**: Seamlessly detects piped input (via terminal detection), supporting `cat .env | dotdim` or `dotdim -`.

### 🔍 Phase 2: Parsing Logic
- **Comment Bypass**: Recognizes comment lines starting with `#` and routes them directly to output without parsing.
- **Whitespace Preservation**: Preserves empty lines and visual spacing exactly as formatted in the original file.
- **First-Match Splitting**: Uses `str::split_once('=')` so inline equals signs inside URLs or base64 strings (e.g. `DATABASE_URL=postgres://u:p=ss@localhost/db`) are safely preserved as part of the value.
- **Quote Stripping**: Safely trims surrounding single (`'`) or double (`"`) quotes from the value before applying the masking rules.

### 🛡️ Phase 3: Smart Masking Engine
- **Empty State**: Immediately returns an empty string if the value length is 0 (`EMPTY=`).
- **Short Value Masking**: Values shorter than 6 characters are fully replaced with an exact matching number of `*` characters (e.g. `1234` -> `****`).
- **Long Value Masking**: Retains the first 3 and last 2 characters of strings 6 characters or longer, masking the center with exactly eight `*` characters (e.g. `sk_live_1234b` -> `sk_********4b`).
- **Unicode Support**: Character-aware slicing accurately handles multi-byte UTF-8 secrets without corruption or panics.

### 🎨 Phase 4: UI & Styling
- **Color Engine**: ANSI syntax highlighting powered by `colored`.
  - **Cyan** for keys
  - **White** for the `=` delimiter
  - **Yellow** for masked values
  - **Dim Gray** for comments
- **Color Fallback**: Automatically disables ANSI colors when output is redirected or piped to a file/non-TTY environment, when the `NO_COLOR` environment variable is set, or with `--no-color`.

### ⚡ Phase 5: Advanced Utilities (V2)
- **`--compare <TEMPLATE>` (`-c`)**: Compares your local `.env` against a template file (such as `.env.example`) and prints bold red warnings for any keys defined in the template that are missing locally.
- **Validation Mode (`--lint` / `-l`)**: Validates lines for syntax issues (e.g. missing `=` delimiters, illegal spaces inside key names) and exits with a non-zero exit code (`1`) if errors are found.

---

## Usage

### Basic Usage
```bash
# Read default .env in the current directory
dotdim

# Read a specific file
dotdim .env.production

# Pipe via stdin
cat .env | dotdim
```

### Template Comparison
Check if your local `.env` is missing any keys defined in a template file:
```bash
dotdim .env --compare .env.example
```

### Linting
Validate that all entries are well-formed:
```bash
dotdim .env --lint
```

### Options & Flags
```
Usage: dotdim [OPTIONS] [FILE]

Arguments:
  [FILE]                    Path to the .env file (defaults to .env, or stdin if piped)

Options:
  -c, --compare <TEMPLATE>  Compare against a template file and warn on missing keys
  -l, --lint                Validation mode: exit with non-zero status code if malformed lines are detected
      --no-color            Disable colored ANSI output
  -h, --help                Print help
  -V, --version             Print version
```

---

## Building from Source

```bash
cargo build --release
```

Run test suite:
```bash
cargo test
```