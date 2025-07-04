# Rust-PHF

[![CI](https://github.com/rust-phf/rust-phf/actions/workflows/ci.yml/badge.svg)](https://github.com/rust-phf/rust-phf/actions/workflows/ci.yml) [![Latest Version](https://img.shields.io/crates/v/phf.svg)](https://crates.io/crates/phf)

[Documentation](https://docs.rs/phf)

Rust-PHF is a library to generate efficient lookup tables at compile time using
[perfect hash functions](http://en.wikipedia.org/wiki/Perfect_hash_function).

It currently uses the
[CHD algorithm](http://cmph.sourceforge.net/papers/esa09.pdf) and can generate
a 100,000 entry map in roughly .4 seconds.

MSRV (minimum supported rust version) is Rust 1.66.

## Usage

PHF data structures can be constructed via either the procedural
macros in the `phf_macros` crate or code generation supported by the
`phf_codegen` crate.

To compile the `phf` crate with a dependency on
libcore instead of libstd, enabling use in environments where libstd
will not work, set `default-features = false` for the dependency:

```toml
[dependencies]
# to use `phf` in `no_std` environments
phf = { version = "0.12", default-features = false }
```

### phf_macros

```rust
use phf::phf_map;

#[derive(Clone)]
pub enum Keyword {
    Loop,
    Continue,
    Break,
    Fn,
    Extern,
}

static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
    "loop" => Keyword::Loop,
    "continue" => Keyword::Continue,
    "break" => Keyword::Break,
    "fn" => Keyword::Fn,
    "extern" => Keyword::Extern,
};

// You can also use OR (`|`) patterns to map multiple keys to the same value:
static OPERATORS: phf::Map<&'static str, &'static str> = phf_map! {
    "+" | "add" | "plus" => "addition",
    "-" | "sub" | "minus" => "subtraction",
    "*" | "mul" | "times" => "multiplication",
};

pub fn parse_keyword(keyword: &str) -> Option<Keyword> {
    KEYWORDS.get(keyword).cloned()
}

pub fn parse_operator(operator: &str) -> Option<&'static str> {
    OPERATORS.get(operator).copied()
}
```

```toml
[dependencies]
phf = { version = "0.12", features = ["macros"] }
```

#### Note

Currently, the macro syntax has some limitations and may not
work as you want. See [#196] for example.

[#196]: https://github.com/rust-phf/rust-phf/issues/196

### phf_codegen

To use `phf_codegen` on build.rs, you have to add dependencies under `[build-dependencies]`:

```toml
[build-dependencies]
phf = { version = "0.12", default-features = false }
phf_codegen = "0.12"
```

Then put code on build.rs:

```rust
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    write!(
        &mut file,
        "static KEYWORDS: phf::Map<&'static str, Keyword> = {}",
        phf_codegen::Map::new()
            .entry("loop", "Keyword::Loop")
            .entry("continue", "Keyword::Continue")
            .entry("break", "Keyword::Break")
            .entry("fn", "Keyword::Fn")
            .entry("extern", "Keyword::Extern")
            .build()
    )
    .unwrap();
    write!(&mut file, ";\n").unwrap();

    // Example with OR patterns (note: phf_codegen doesn't support OR patterns directly)
    write!(
        &mut file,
        "static OPERATORS: phf::Map<&'static str, &'static str> = {}",
        phf_codegen::Map::new()
            .entry("+", "\"addition\"")
            .entry("add", "\"addition\"")
            .entry("plus", "\"addition\"")
            .entry("-", "\"subtraction\"")
            .entry("sub", "\"subtraction\"")
            .entry("minus", "\"subtraction\"")
            .build()
    )
    .unwrap();
    write!(&mut file, ";\n").unwrap();
}
```

and lib.rs:

```rust
#[derive(Clone)]
enum Keyword {
    Loop,
    Continue,
    Break,
    Fn,
    Extern,
}

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub fn parse_keyword(keyword: &str) -> Option<Keyword> {
    KEYWORDS.get(keyword).cloned()
}

pub fn parse_operator(operator: &str) -> Option<&'static str> {
    OPERATORS.get(operator).copied()
}
```
