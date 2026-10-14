# `dotdim`

> **Never leak your secrets on stream again.**  
> A lightning-fast CLI utility that colorizes and intelligently masks `.env` files on the fly. Built for developers who screen share, pair program, or present live demos.

---

## ⚡ What it Does

When you're debugging or demonstrating code, inspecting `.env` files with `cat` or text editors risks exposing production tokens, database passwords, or private keys.

`dotdim` formats your configuration with clean syntax highlighting while securely masking values—keeping recognizable prefixes and suffixes so you can verify configurations without exposing raw credentials.

### Visual Preview

**Raw `.env`:**
```ini
# Database & Auth
PORT=3000
DATABASE_URL=postgres://postgres:s3cr3tp@ss@localhost:5432/mydb
STRIPE_SECRET_KEY="sk_live_51ABC1234567890XYZ_SUPER_SECRET"
SHORT_CODE=abc
EMPTY_KEY=
```

**`dotdim` Output:**
```ini
# Database & Auth
PORT=****
DATABASE_URL=pos********db
STRIPE_SECRET_KEY=sk_********ET
SHORT_CODE=***
EMPTY_KEY=
```

---

## ✨ Features

* 🛡️ **Intelligent Masking Engine**:
  * Values under 6 characters are masked to their exact length (e.g. `1234` $\to$ `****`).
  * Values 6+ characters retain their first 3 and last 2 characters for easy identification while replacing the center with `********` (e.g. `sk_live_...` $\to$ `sk_********4b`).
  * Empty keys (`KEY=`) remain cleanly empty.
  * Multi-byte UTF-8 / Unicode characters are handled safely without string corruption.
* 🎨 **Syntax Highlighting**:
  * **Cyan** for keys
  * **White** for `=` delimiters
  * **Yellow** for masked values
  * **Dim Gray** for comments (`# ...`)
* 🔁 **Streaming & Stdin Support**: Pipe configuration dumps straight from `cat`, `docker exec`, or Kubernetes secrets (`cat .env | dotdim`).
* 🧹 **Preserves Structure**: Retains empty lines and visual spacing intact so sections remain organized.
* 🔍 **Template Comparison (`--compare`)**: Audits your local `.env` against team templates (`.env.example`) and prints bold red warnings for any missing keys.
* ✅ **Validation & Linting (`--lint`)**: Validates line syntax (missing `=` signs, illegal spaces in keys) and exits with code `1`—ideal for CI/CD pipelines and pre-commit hooks.
* 🚫 **Safe Color Fallback**: Automatically disables ANSI colors when redirected into files or non-TTY environments, or when `NO_COLOR` is set.

---

## 📦 Installation

### Option A: For Rust Users

If you already have the Rust toolchain installed:

#### 1. Direct Install via Cargo (Recommended)
```bash
cargo install --git https://github.com/AshishOverflow/dotdim.git
```

#### 2. Build from Source
```bash
git clone https://github.com/AshishOverflow/dotdim.git
cd dotdim
cargo build --release
cargo install --path .
```

---

### Option B: For Non-Rust Users

You do **not** need Rust installed to use `dotdim`.

#### 1. Download Pre-built Binary
1. Go to the [Releases](https://github.com/AshishOverflow/dotdim/releases) page.
2. Download the binary for your operating system:
   * **Windows**: `dotdim-windows-x86_64.zip` (extract `dotdim.exe`)
   * **Linux**: `dotdim-linux-x86_64.tar.gz`
   * **macOS**: `dotdim-darwin-x86_64.tar.gz`

#### 2. Add to PATH

* **Windows (PowerShell)**:
  Move `dotdim.exe` to a directory in your PATH (e.g. `C:\Windows\System32` or your custom bin directory):
  ```powershell
  # Check if dotdim runs
  dotdim --version
  ```

* **macOS / Linux**:
  Make the binary executable and move it to `/usr/local/bin`:
  ```bash
  chmod +x dotdim
  sudo mv dotdim /usr/local/bin/
  dotdim --version
  ```

---

## 🚀 Usage & Examples

### 1. Basic Inspection
By default, running `dotdim` with no arguments automatically looks for `.env` in the current working directory:
```bash
dotdim
```

Specify a custom file:
```bash
dotdim .env.staging
dotdim /path/to/project/.env.local
```

### 2. Piping from Stdin
Pipe environment streams from terminal commands or container runtimes:
```bash
cat .env | dotdim
```

From Docker or remote environments:
```bash
docker exec -it my_container env | dotdim
```

Explicitly specify standard input using `-`:
```bash
curl -s http://internal-vault/app.env | dotdim -
```

### 3. Check for Missing Configuration (`--compare`)
Ensure a new clone or deployment has all required variables defined in `.env.example`:
```bash
dotdim .env --compare .env.example
```
If keys exist in `.env.example` but are absent from `.env`, `dotdim` outputs:
```text
Warning: Missing key 'STRIPE_SECRET_KEY' defined in template '.env.example'
Warning: Missing key 'REDIS_URL' defined in template '.env.example'
```

### 4. Validate Syntax in CI / Pre-commit (`--lint`)
Catch malformed lines or spaces before committing configuration changes:
```bash
dotdim .env --lint
```
If errors are found, `dotdim` prints the problem lines with line numbers and exits with status code `1`:
```text
Lint check failed: 2 malformed line(s) detected.
  [Line 4] Missing '=' delimiter: "DATABASE_PORT 5432"
  [Line 7] Key 'API KEY' contains illegal spaces: "API KEY=secret123"
```

### 5. Strip Colors for Clean Output
```bash
# Using the flag
dotdim --no-color

# Using the standard environment variable
NO_COLOR=1 dotdim

# Redirecting to a file automatically strips ANSI escape codes
dotdim > sanitized_env.txt
```

---

## ⚙️ Command-Line Reference

```
Usage: dotdim [OPTIONS] [FILE]

Arguments:
  [FILE]                    Path to the .env file (defaults to .env, or stdin if piped)

Options:
  -c, --compare <TEMPLATE>  Compare against a template file and warn on missing keys
  -l, --lint                Validation mode: exit with non-zero status code if malformed lines are detected
      --no-color            Disable colored ANSI output
  -h, --help                Print help (see a summary with '-h')
  -V, --version             Print version
```

---

## 🧪 Testing

To run the internal unit test suite:
```bash
cargo test
```

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).