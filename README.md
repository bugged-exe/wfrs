# wfrs

A small command-line tool, written in Rust, that counts the lines, words,
characters and bytes in a text file. It's a simplified take on the classic
Unix `wc`.

> **Status:** learning project. It works, but it's not
production software. See [Known limitations](#known-limitations).

> **Note:** All of the code in this repository was written by me.
The README was written with the help of AI (Claude), because documentation
isn't my strong suit. I've reviewed it against the code, but if
anything in it is wrong or unclear, please tell me.

## Why this exists

I'm learning Rust by building small, real programs, 
and `wfrs` is one of them. It's part of my YouTube channel, 
**bugged.exe**, where I learn Rust (and other things) in 
public: the compiler errors, the borrow checker fights and 
the bugs included.

- **Channel:** bugged.exe on YouTube - [here](https://www.youtube.com/@bugged_exe)
- **Video for this project:** (link coming soon)

I'm a beginner, so the code is not going to be the most 
idiomatic Rust you've seen. If you spot something that could
be done better, I'd like to hear about it 
(see [Feedback](#feedback)).

## Features

- Count **lines**, **words**, **characters** and **bytes** in a file
- Pick exactly which counts you want with flags
- Sensible default when you pass no flags: lines, characters and words
- Argument parsing and `--help` / `--version` come 
from [`clap`](https://crates.io/crates/clap)

## Installation

You need a Rust toolchain. Install it from [rustup.rs](https://rustup.rs).

```
git clone https://github.com/bugged-exe/wfrs.git
cd wfrs
cargo build --release
```

The binary ends up in `target/release/`. To install it onto 
your `PATH`:

```
cargo install --path .
```

If you're setting the project up from scratch, the only dependency
is `clap` with its derive feature:

```
cargo add clap --features derive
```

## Usage

```
wfrs [OPTIONS] <FILEPATH>
```

| Flag | Long form | What it prints |
|---|---|---|
| `-c` | `--count` | Size of the file in **bytes** |
| `-l` | `--lines` | Number of **lines** |
| `-m` | `--characters` | Number of **characters** |
| `-w` | `--words` | Number of **words** |
| `-h` | `--help` | Help text |
| `-V` | `--version` | Version |

With no flags, `wfrs` prints lines, characters and words. With one
or more flags, it prints only those counts, always in the order bytes,
lines, characters, words (not the order you typed them).

While developing, run it through Cargo and put `--` before the program's
own arguments:

```
cargo run -- sample.txt
cargo run -- -l -w sample.txt
```

## Example

Given a `sample.txt` containing two lines, `hello world` and `this is a test`,
each ending in a newline:

```
$ wfrs sample.txt
The number of lines for: 'sample.txt' is 2
The number of characters for 'sample.txt' is 27
The number of words for 'sample.txt' is 6

$ wfrs -c sample.txt
The size in bytes for: 'sample.txt' is 27

$ wfrs -l -w sample.txt
The number of lines for: 'sample.txt' is 2
The number of words for 'sample.txt' is 6
```

On Windows, a file saved with CRLF line endings has one extra byte 
and character per line, so the byte and character counts will be higher.

## How it works

The whole program is one file, `src/main.rs`:

1. `clap` parses the arguments into an `Args` struct.
2. `run` reads the file into a `String` and also reads its metadata
(for the byte count).
3. Based on the flags, it calls one small `display_*` function per count.

How each number is worked out:

- **Lines:** `str::lines()`, counted.
- **Characters:** `str::chars()`, counted. These are
Unicode characters, not bytes.
- **Words:** `str::split_whitespace()`, counted.
- **Bytes:** the file size from `fs::metadata`.

## Known limitations

These are known and intentional for a first version:

- **Errors panic.** A missing file or unreadable file crashes
with a short message instead of a friendly error.
- **UTF-8 text only.** The file is read with `read_to_string`,
so a file that isn't valid UTF-8 (a binary file, for example) fails.
- **The whole file is loaded into memory.** That's fine for small files
and a poor fit for huge ones.
- **One file at a time**, and no reading from standard input.
- **Line counting differs slightly from `wc -l`.** `wc` counts newline
characters, so a file whose last line has no trailing newline can give a
different number than `wfrs`.
- **Output wording is not column-aligned like `wc`.** It prints a sentence
per count.

## Feedback

Spotted a bug, or a more idiomatic way to write something? Open an issue or
leave a comment on the video. I'm learning, so constructive feedback is welcome.

_README drafted with AI assistance; the code is hand-written._