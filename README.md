# Toml2CV

Unreasonably straightforward Rust tool built because manually tweaking my
`resume.typ` depending on the job application was too arduous.

This program reads a `toml` file w/ pre-set fields, and fills them into a Typst
resume template, eventually outputting it,

## Setup

```bash
git clone https://github.com/masroof-maindak/toml2cv.git && cd toml2cv
cargo install --path .

# Or:

cargo install toml2cv
```

## Usage

```bash
# TODO: explain cli flags
```

## Configuration

```toml
# 2cv.toml

# TODO: update schema here w.r.t structs in src/structs.rs
```

## Acknowledgements

- [Typst: Simple Technical Resume](https://typst.app/universe/package/simple-technical-resume/)
