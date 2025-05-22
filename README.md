# 🐱 kitten

Kitten is a Rust-powered reimplementation of the classic Unix `cat` command — built as part of a broader mission to make Linux tools more **Rusty**, secure, and modern.

It supports all common `cat` flags (including `-A`, `-v`, `-E`, `-T`, `-b`, `-n`, `-s`, `-e`, and `-t`) while offering a clean, safe codebase written entirely in Rust.

---

## 🚀 Features

- `-n`, `--number`: Number all output lines.
- `-b`, `--number-nonblank`: Number non-blank lines only.
- `-s`, `--squeeze-blank`: Suppress repeated blank lines.
- `-E`, `--show-ends`: Display `$` at the end of each line.
- `-T`, `--show-tabs`: Display tabs as `^I`.
- `-v`, `--show-nonprinting`: Show non-printable characters (except tabs/newlines) as `^X`.
- `-A`, `--show-all`: Equivalent to `-v -E -T`.
- `-e`: Equivalent to `-v -E`.
- `-t`: Equivalent to `-v -T`.
- `-`: Read from standard input (stdin).

---

## 📆 Installation

### 🔹 Option 1: Download Prebuilt Release (Recommended)

 Each release includes:

- A precompiled binary: `kitten`
- An `assets/` directory with:
  - `helpfile.txt`
  - `versionfile.txt`

To install:
Grab the latest release from the [Releases page](https://github.com/Hunter2718/kitten/releases/#latest).

```bash
tar -xf kitten-v1.0.0-x86_64.tar.gz
cd kitten-v1.0.0-x86_64.tar.gz
chmod +x kitten
./kitten --help
```

> 🔥 **Note**: The `assets/` directory must remain alongside the binary for help/version messages to work correctly.

### 🔹 Option 2: Build from Source (Requires cargo)

```bash
git clone https://github.com/Hunter2718/kitten.git
cd kitten
cargo build --release
./target/release/kitten --help
```

---

## 💪 Usage

```bash
kitten [FLAGS] [FILES]...
```

### Examples

- Number all lines:
  ```bash
  kitten -n file.txt
  ```

- Squeeze blank lines and show line ends:
  ```bash
  kitten -s -E file.txt
  ```

- Show tabs and non-printable characters:
  ```bash
  kitten -t -v file.txt
  ```

- Combine files and stdin:
  ```bash
  kitten -A - file1.txt file2.txt
  ```

- Read from standard input:
  ```bash
  echo -e "hi\tbye" | kitten -t -
  ```

---

## 📂 Assets Directory

Kitten uses an `assets/` directory to display user-facing information.

- `assets/helpfile.txt`: Custom help output for `--help`
- `assets/versionfile.txt`: Custom version output for `--version`

These files are required at runtime and are bundled with releases. Keep them next to the binary.

---

## 🛠️ Transform Order

Kitten applies flags in the following order (to match GNU `cat` behavior):

1. `-s` squeeze blank lines  
2. `-T` show tabs  
3. `-v` show non-printables  
4. `-E` show ends  
5. `-n` or `-b` line numbering

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).

---

## 🌱 Project Goals

Kitten is part of a broader initiative to **rewrite essential Linux command-line tools in Rust**, improving safety, readability, and long-term maintainability.

This project aims to:

- Deepen the developer’s understanding of systems-level Rust  
- Serve as a proof-of-concept for Rust-powered coreutils  
- Inspire others to join the Rustification of Linux

---

## 🤝 Contributing

Contributions, improvements, bug reports, and feature requests are welcome!

- Fork the repo  
- Create a feature branch  
- Submit a pull request

---

## 🐾 Author

Made by me (**Hunter2718**)  
Find me on GitHub: [@Hunter2718](https://github.com/Hunter2718)

For questions, feedback, or ideas, open an issue.

